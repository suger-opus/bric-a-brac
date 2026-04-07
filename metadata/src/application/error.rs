use crate::infrastructure::InfraError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    InfraError(Box<InfraError>),

    #[error("An active session already exists for this graph")]
    ActiveSessionAlreadyExists,

    #[error("Forbidden: insufficient permissions")]
    Forbidden,
}

impl From<InfraError> for AppError {
    fn from(err: InfraError) -> Self {
        Self::InfraError(Box::new(err))
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        InfraError::from(err).into()
    }
}
