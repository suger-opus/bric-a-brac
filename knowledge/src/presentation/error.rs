use crate::{application::AppError, infrastructure::DatabaseError};
use bric_a_brac_dtos::DtosConversionError;
use tonic::{Code, Status};

#[derive(Debug, thiserror::Error)]
pub enum PresentationError {
    #[error(transparent)]
    AppError(#[from] AppError),

    #[error(transparent)]
    DtosConversionError(#[from] DtosConversionError),

    #[error("Validation errors: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),

    #[error("Missing required field: {0}")]
    MissingField(String),

    #[error("Depth out of range: {value} (must be between 1 and 10)")]
    DepthOutOfRange { value: i32 },
}

impl From<PresentationError> for Status {
    #[allow(clippy::cognitive_complexity)]
    fn from(err: PresentationError) -> Self {
        match err {
            PresentationError::AppError(app_err) => app_err.into(),
            PresentationError::DtosConversionError(dto_err) => {
                tracing::warn!(error = ?dto_err);
                Self::new(Code::InvalidArgument, format!("{dto_err}"))
            }
            PresentationError::ValidationErrors(validation_err) => {
                tracing::warn!(error = ?validation_err);
                Self::new(Code::InvalidArgument, format!("{validation_err}"))
            }
            PresentationError::MissingField(_) => {
                tracing::warn!(error = ?err);
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
            PresentationError::DepthOutOfRange { .. } => {
                tracing::warn!(error = ?err);
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
        }
    }
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Database(db_err) => db_err.into(),
        }
    }
}

impl From<DatabaseError> for Status {
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NoRows() | DatabaseError::NodeNotFoundInPath { .. } => {
                tracing::warn!(error = ?err);
                Self::new(Code::NotFound, format!("{err}"))
            }
            DatabaseError::CorruptedNumber { .. }
            | DatabaseError::CorruptedIdState { .. }
            | DatabaseError::CorruptedId { .. }
            | DatabaseError::CorruptedPropertyState { .. }
            | DatabaseError::CorruptedProperty { .. }
            | DatabaseError::CorruptedNodeLabelState { .. }
            | DatabaseError::AuthenticationError { .. }
            | DatabaseError::Unknown { .. } => {
                tracing::error!(error = ?err);
                Self::new(Code::Internal, format!("{err}"))
            }
            DatabaseError::ConnectionError { .. } => {
                tracing::error!(error = ?err);
                Self::new(Code::Unavailable, format!("{err}"))
            }
        }
    }
}
