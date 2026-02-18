use super::HttpError;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use sqlx::{error::ErrorKind, postgres::PgDatabaseError};

#[derive(Debug, Clone, Copy, Serialize, derive_more::Display)]
pub enum ServiceKind {
    Ai,
    Knowledge,
}

#[derive(Debug, thiserror::Error)]
pub enum GrpcClientError {
    #[error("gRPC service {service}: inaccessible")]
    Inaccessible {
        service: ServiceKind,
        #[source]
        source: tonic::transport::Error,
    },

    #[error("gRPC service {service}: disconnected")]
    Disconnected { service: ServiceKind },

    #[error("gRPC service {service}: failed to deserialize response into {expected_struct}")]
    Deserialization {
        service: ServiceKind,
        expected_struct: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("gRPC service {service}: response could not be converted to domain model - {reason}")]
    DomainConversion {
        service: ServiceKind,
        reason: String,
    },

    #[error(
        "gRPC service {service}: response uuid could not be converted to domain uuid - {source}"
    )]
    DomainUuidConversion {
        service: ServiceKind,
        #[source]
        source: uuid::Error,
    },

    #[error("gRPC service {service}: request failed - {message}")]
    Request {
        service: ServiceKind,
        message: String,
        #[source]
        source: tonic::Status,
    },

    #[error("Mutex lock poisoned")]
    MutexPoisoned { message: String },

    // TODO: should be handle by caller
    #[error("Invalid input: {field} - {issue}")]
    InvalidInput {
        issue: String,
        field: String,
        value: String,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database: unknown error")]
    Unknown {
        #[source]
        source: sqlx::Error,
    },

    #[error("Database: unique constraint violation on {table}.{column}")]
    UniqueConstraintViolation {
        table: String,
        column: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Database: business constraint violation {constraint} on {table}.{column}")]
    BusinessConstraintViolation {
        table: String,
        column: String,
        constraint: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Database: primary constraint violation {constraint} on {table}.{column}")]
    PrimaryConstraintViolation {
        table: String,
        column: String,
        constraint: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Database: resource not found")]
    NotFound {
        #[source]
        source: sqlx::Error,
    },

    #[error("Database: unexpected state - {reason}")]
    UnexpectedState { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    GrpcService(#[from] GrpcClientError),

    // TODO: group following errors (with invalid input)
    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("Invalid header: {header} - {issue}")]
    InvalidHeader {
        issue: String,
        header: String,
        value: String,
    },

    #[error("Invalid file: {issue}")]
    InvalidFile { issue: String },

    #[error("Invalid property: {property} - {issue}")]
    PropertyValidation { property: String, issue: String },
}

impl GrpcClientError {
    pub fn is_connection_error(&self) -> bool {
        match self {
            GrpcClientError::Inaccessible { .. } => true,
            GrpcClientError::Disconnected { .. } => true,
            GrpcClientError::Request { source, .. } => {
                return matches!(
                    source.code(),
                    tonic::Code::Unavailable
                        | tonic::Code::DeadlineExceeded
                        | tonic::Code::Cancelled
                        | tonic::Code::Unknown
                );
            }
            _ => false,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        HttpError::from(self).into_response()
    }
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound { source: err },
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                let detail = pg_err.detail().unwrap_or("unknown").to_string();
                let table = pg_err.table().unwrap_or("unknown").to_string();
                let constraint_name = pg_err.constraint().unwrap_or("unknown").to_string();
                // Constraint names follow pattern: tablename_fieldname_key
                // Remove "_key" suffix, then skip table name prefix
                let column = pg_err
                    .constraint()
                    .and_then(|constraint| {
                        constraint
                            .strip_suffix("_key")
                            .and_then(|s| s.split_once('_').map(|(_, field)| field))
                    })
                    .unwrap_or("unknown")
                    .to_string();

                match db_err.kind() {
                    ErrorKind::UniqueViolation => DatabaseError::UniqueConstraintViolation {
                        table,
                        column,
                        detail,
                        source: err,
                    },
                    ErrorKind::NotNullViolation => DatabaseError::PrimaryConstraintViolation {
                        table,
                        column,
                        constraint: "not null".to_string(),
                        detail,
                        source: err,
                    },
                    ErrorKind::ForeignKeyViolation => DatabaseError::PrimaryConstraintViolation {
                        table,
                        column,
                        constraint: "foreign key".to_string(),
                        detail,
                        source: err,
                    },
                    ErrorKind::CheckViolation => DatabaseError::BusinessConstraintViolation {
                        table,
                        column,
                        constraint: constraint_name,
                        detail,
                        source: err,
                    },
                    _ => DatabaseError::Unknown { source: err },
                }
            }
            _ => DatabaseError::Unknown { source: err },
        }
    }
}
