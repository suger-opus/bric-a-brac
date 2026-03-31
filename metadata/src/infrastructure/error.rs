use bric_a_brac_dtos::DtosConversionError;
use bric_a_brac_protos::GrpcRequestError;
use sqlx::{error::ErrorKind, postgres::PgDatabaseError};

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum InfraError {
    #[error(transparent)]
    DatabaseError(#[from] DatabaseError),

    #[error(transparent)]
    GrpcRequestError(#[from] GrpcRequestError),

    #[error(transparent)]
    DtosConversionError(#[from] DtosConversionError),
}

impl From<sqlx::Error> for InfraError {
    fn from(err: sqlx::Error) -> Self {
        Self::DatabaseError(DatabaseError::from(err))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Unknown database error")]
    Unknown {
        #[source]
        source: sqlx::Error,
    },

    #[error("Unique constraint violation on {table}.{column}")]
    UniqueConstraintViolation {
        table: String,
        column: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Business constraint violation {constraint} on {table}.{column}")]
    BusinessConstraintViolation {
        table: String,
        column: String,
        constraint: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Primary constraint violation {constraint} on {table}.{column}")]
    PrimaryConstraintViolation {
        table: String,
        column: String,
        constraint: String,
        detail: String,
        #[source]
        source: sqlx::Error,
    },

    #[error("Resource not found")]
    NotFound {
        #[source]
        source: Option<sqlx::Error>,
    },

    #[error("Unexpected state - {reason}")]
    UnexpectedState { reason: String },
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => Self::NotFound { source: Some(err) },
            sqlx::Error::Database(db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                let detail = pg_err.detail().unwrap_or("unknown").to_owned();
                let table = pg_err.table().unwrap_or("unknown").to_owned();
                let constraint_name = pg_err.constraint().unwrap_or("unknown").to_owned();
                let column = pg_err
                    .constraint()
                    .and_then(|constraint| {
                        constraint
                            .strip_suffix("_key")
                            .and_then(|s| s.split_once('_').map(|(_, field)| field))
                    })
                    .unwrap_or("unknown")
                    .to_owned();

                match db_err.kind() {
                    ErrorKind::UniqueViolation => Self::UniqueConstraintViolation {
                        table,
                        column,
                        detail,
                        source: err,
                    },
                    ErrorKind::NotNullViolation => Self::PrimaryConstraintViolation {
                        table,
                        column,
                        constraint: "not null".to_owned(),
                        detail,
                        source: err,
                    },
                    ErrorKind::ForeignKeyViolation => Self::PrimaryConstraintViolation {
                        table,
                        column,
                        constraint: "foreign key".to_owned(),
                        detail,
                        source: err,
                    },
                    ErrorKind::CheckViolation => Self::BusinessConstraintViolation {
                        table,
                        column,
                        constraint: constraint_name,
                        detail,
                        source: err,
                    },
                    _ => Self::Unknown { source: err },
                }
            }
            _ => Self::Unknown { source: err },
        }
    }
}
