use crate::infrastructure::errors::{DatabaseError, GrpcClientError};

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("Request: unauthorized - {reason}")]
    Unauthorized { reason: String },

    #[error("Request: invalid header {header} - {issue}")]
    InvalidHeader { issue: String, header: String },

    #[error("Request: invalid file - {issue}")]
    InvalidFile { issue: String },

    #[error("Request: invalid input {field} - {issue}")]
    InvalidInput { issue: String, field: String },
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error(transparent)]
    GrpcClient(#[from] GrpcClientError),

    #[error(transparent)]
    Request(#[from] RequestError),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError::from(err).into()
    }
}
