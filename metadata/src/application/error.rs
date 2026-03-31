use crate::infrastructure::InfraError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    InfraError(#[from] InfraError),

    #[error("An active session already exists for this graph")]
    ActiveSessionAlreadyExists,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        InfraError::from(err).into()
    }
}
