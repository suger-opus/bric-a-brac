use bric_a_brac_dtos::DtosConversionError;
use bric_a_brac_protos::BaseGrpcClientError;
use sqlx::{error::ErrorKind, postgres::PgDatabaseError};

#[derive(Debug, thiserror::Error)]
pub enum GrpcClientError {
    #[error(transparent)]
    Base(#[from] BaseGrpcClientError),

    #[error(transparent)]
    Conversion(#[from] DtosConversionError),
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

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound { source: err },
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                let detail = pg_err.detail().unwrap_or("unknown").to_string();
                let table = pg_err.table().unwrap_or("unknown").to_string();
                let constraint_name = pg_err.constraint().unwrap_or("unknown").to_string();
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
