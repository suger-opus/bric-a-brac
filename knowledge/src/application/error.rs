use crate::infrastructure::DatabaseError;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    Database(#[from] DatabaseError),
}

impl From<neo4rs::Error> for AppError {
    fn from(err: neo4rs::Error) -> Self {
        DatabaseError::from(err).into()
    }
}
