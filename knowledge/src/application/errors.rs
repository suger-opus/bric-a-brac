use crate::infrastructure::errors::DatabaseError;
use bric_a_brac_dtos::DtosConversionError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Conversion(#[from] DtosConversionError),

    #[error(transparent)]
    Database(#[from] DatabaseError),

    #[error("Internal error: {context}")]
    Internal { context: String },

    #[error("Not found: {entity}")]
    NotFound { entity: String },

    #[error("Invalid input: {reason}")]
    InvalidInput { reason: String },
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        DatabaseError::from(err).into()
    }
}
