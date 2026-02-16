use crate::domain::models::UserId;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::{error::ErrorKind, postgres::PgDatabaseError};
use tonic::Status;

#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Invalid input: {field} - {issue}")]
    ValidationError { field: String, issue: String },

    #[error("Resource not found: {entity} with identifier '{identifier}'")]
    NotFound { entity: String, identifier: String },

    #[error("Conflict: {entity} with {field}='{value}' already exists")]
    AlreadyExists {
        entity: String,
        field: String,
        value: String,
    },

    #[error("User {user_id} lacks permission: {required_permission}")]
    InsufficientPermission {
        user_id: UserId,
        required_permission: String,
    },

    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("Invalid graph schema: {reason}")]
    InvalidSchema { reason: String },

    #[error("Property '{property}' validation failed: {reason}")]
    PropertyValidation { property: String, reason: String },

    #[error("Business rule violation: {rule}")]
    BusinessRuleViolation { rule: String },
}

#[derive(Debug, thiserror::Error)]
pub enum InfraError {
    #[error("Database error: {context}")]
    Database {
        #[source]
        source: sqlx::Error,
        context: String,
    },

    #[error("Database constraint violation: {constraint} on {table}.{column}")]
    ConstraintViolation {
        table: String,
        column: String,
        constraint: String,
        detail: String,
    },

    #[error("gRPC service error: {service} - {message}")]
    GrpcService {
        service: String,
        message: String,
        #[source]
        source: Option<Status>,
    },

    #[error("gRPC transport error: {source}")]
    GrpcTransport {
        #[source]
        source: tonic::transport::Error,
    },

    #[error("UUID conversion error: {context}")]
    UuidConversion {
        #[source]
        source: uuid::Error,
        context: String,
    },

    #[error("Unexpected database state: {reason}")]
    DatabaseState { reason: String },
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Domain(#[from] DomainError),

    #[error(transparent)]
    Infra(#[from] InfraError),

    #[error("{context}: {source}")]
    Contextual {
        context: String,
        #[source]
        source: Box<AppError>,
    },
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self, "Request failed");

        let (status, message, details) = match self {
            AppError::Domain(DomainError::ValidationError { field, issue }) => (
                StatusCode::BAD_REQUEST,
                format!("Validation error: {}", field),
                json!({ "field": field, "issue": issue }),
            ),
            AppError::Domain(DomainError::NotFound { entity, identifier }) => (
                StatusCode::NOT_FOUND,
                format!("{} not found", entity),
                json!({ "entity": entity, "identifier": identifier }),
            ),
            AppError::Domain(DomainError::AlreadyExists {
                entity,
                field,
                value,
            }) => (
                StatusCode::CONFLICT,
                format!("{} already exists", entity),
                json!({ "entity": entity, "field": field, "value": value }),
            ),
            AppError::Domain(DomainError::InsufficientPermission {
                user_id,
                required_permission,
            }) => (
                StatusCode::FORBIDDEN,
                "Insufficient permissions".to_string(),
                json!({
                    "user_id": user_id.to_string(),
                    "required": required_permission
                }),
            ),
            AppError::Domain(DomainError::Unauthorized { reason }) => (
                StatusCode::UNAUTHORIZED,
                "Unauthorized".to_string(),
                json!({ "reason": reason }),
            ),
            AppError::Domain(DomainError::InvalidSchema { reason }) => (
                StatusCode::BAD_REQUEST,
                "Invalid graph schema".to_string(),
                json!({ "reason": reason }),
            ),
            AppError::Domain(DomainError::InvalidInput { reason }) => (
                StatusCode::BAD_REQUEST,
                "Invalid input".to_string(),
                json!({ "reason": reason }),
            ),
            AppError::Domain(DomainError::PropertyValidation { property, reason }) => (
                StatusCode::BAD_REQUEST,
                format!("Property validation failed: {}", property),
                json!({ "property": property, "reason": reason }),
            ),
            AppError::Domain(DomainError::BusinessRuleViolation { rule }) => (
                StatusCode::BAD_REQUEST,
                "Business rule violation".to_string(),
                json!({ "rule": rule }),
            ),

            AppError::Infra(InfraError::ConstraintViolation {
                table,
                column,
                constraint,
                detail,
            }) => (
                if constraint == "unique" {
                    StatusCode::CONFLICT
                } else {
                    StatusCode::BAD_REQUEST
                },
                detail.clone(),
                json!({
                    "table": table,
                    "column": column,
                    "constraint": constraint,
                    "detail": detail
                }),
            ),
            AppError::Infra(InfraError::Database { source, context }) => {
                if matches!(source, sqlx::Error::RowNotFound) {
                    (
                        StatusCode::NOT_FOUND,
                        "Resource not found".to_string(),
                        json!({ "context": context }),
                    )
                } else {
                    tracing::error!(error = ?source, context = %context, "Database error");
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal server error".to_string(),
                        json!({ "context": context }),
                    )
                }
            }
            AppError::Infra(InfraError::GrpcService {
                service,
                message,
                source,
            }) => {
                if let Some(status) = source {
                    tracing::error!(
                        service = %service,
                        code = ?status.code(),
                        "gRPC service error"
                    );
                }
                (
                    StatusCode::BAD_GATEWAY,
                    format!("External service error: {}", service),
                    json!({ "service": service, "message": message }),
                )
            }
            AppError::Infra(InfraError::GrpcTransport { source }) => {
                tracing::error!(error = ?source, "gRPC transport error");
                (
                    StatusCode::BAD_GATEWAY,
                    "Failed to communicate with external service".to_string(),
                    json!({ "error": source.to_string() }),
                )
            }
            AppError::Infra(InfraError::UuidConversion { source, context }) => {
                tracing::error!(error = ?source, context = %context, "UUID conversion error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal error".to_string(),
                    json!({ "context": context }),
                )
            }
            AppError::Infra(InfraError::DatabaseState { reason }) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected database state".to_string(),
                json!({ "reason": reason }),
            ),
            AppError::Contextual { context: _, source } => return (*source).into_response(),
        };

        let body = Json(json!({
            "error": {
                "message": message,
                "details": details
            }
        }));

        (status, body).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(error = ?err, "Database error occurred");

        match &err {
            sqlx::Error::RowNotFound => InfraError::Database {
                source: err,
                context: "Expected row not found in database".to_string(),
            }
            .into(),
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                let detail = pg_err.detail().unwrap_or("unknown").to_string();
                let table = pg_err.table().unwrap_or("unknown").to_string();
                let constraint_name = pg_err.constraint().unwrap_or("unknown").to_string();
                let column = extract_field_name_from_error(pg_err);

                match db_err.kind() {
                    ErrorKind::UniqueViolation => InfraError::ConstraintViolation {
                        table,
                        column,
                        constraint: "unique".to_string(),
                        detail,
                    }
                    .into(),
                    ErrorKind::NotNullViolation => InfraError::ConstraintViolation {
                        table,
                        column,
                        constraint: "not null".to_string(),
                        detail,
                    }
                    .into(),
                    ErrorKind::CheckViolation => InfraError::ConstraintViolation {
                        table,
                        column,
                        constraint: constraint_name,
                        detail,
                    }
                    .into(),
                    ErrorKind::ForeignKeyViolation => InfraError::ConstraintViolation {
                        table,
                        column,
                        constraint: "foreign key".to_string(),
                        detail,
                    }
                    .into(),
                    kind => InfraError::Database {
                        source: err,
                        context: format!("Database error: {:?}", kind),
                    }
                    .into(),
                }
            }
            _ => InfraError::Database {
                source: err,
                context: "Unknown database error".to_string(),
            }
            .into(),
        }
    }
}

impl From<Status> for AppError {
    fn from(status: Status) -> Self {
        tracing::error!(code = ?status.code(), message = %status.message(), "gRPC status error");
        InfraError::GrpcService {
            service: "knowledge".to_string(),
            message: status.message().to_string(),
            source: Some(status),
        }
        .into()
    }
}

impl From<tonic::transport::Error> for AppError {
    fn from(err: tonic::transport::Error) -> Self {
        tracing::error!(error = ?err, "gRPC transport error");
        InfraError::GrpcTransport { source: err }.into()
    }
}

impl From<uuid::Error> for AppError {
    fn from(err: uuid::Error) -> Self {
        InfraError::UuidConversion {
            source: err,
            context: "Failed to convert UUID".to_string(),
        }
        .into()
    }
}

fn extract_field_name_from_error(pg_err: &PgDatabaseError) -> String {
    // Constraint names follow pattern: tablename_fieldname_key
    // Remove "_key" suffix, then skip table name prefix
    pg_err
        .constraint()
        .and_then(|constraint| {
            constraint
                .strip_suffix("_key")
                .and_then(|s| s.split_once('_').map(|(_, field)| field))
        })
        .unwrap_or("unknown")
        .to_string()
}

pub trait ResultExt<T> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError>;

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String;
}

impl<T> ResultExt<T> for Result<T, sqlx::Error> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|err| AppError::Contextual {
            context: context.into(),
            source: Box::new(AppError::from(err)),
        })
    }

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|err| AppError::Contextual {
            context: f(),
            source: Box::new(AppError::from(err)),
        })
    }
}

impl<T> ResultExt<T> for Result<T, Status> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|status| AppError::Contextual {
            context: context.into(),
            source: Box::new(AppError::from(Status::from(status))),
        })
    }

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|status| AppError::Contextual {
            context: f(),
            source: Box::new(AppError::from(Status::from(status))),
        })
    }
}

impl<T> ResultExt<T> for Result<T, AppError> {
    fn context(self, context: impl Into<String>) -> Result<T, AppError> {
        self.map_err(|err| AppError::Contextual {
            context: context.into(),
            source: Box::new(err),
        })
    }

    fn with_context<F>(self, f: F) -> Result<T, AppError>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|err| AppError::Contextual {
            context: f(),
            source: Box::new(err),
        })
    }
}
