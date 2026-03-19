use crate::{application::errors::AppError, infrastructure::errors::OpenRouterClientError};
use tonic::{Code, Status};

pub struct TonicError {
    message: String,
    code: Code,
}

impl TonicError {
    fn new(code: Code, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl From<TonicError> for Status {
    fn from(err: TonicError) -> Self {
        Self::new(err.code, err.message)
    }
}

impl From<AppError> for TonicError {
    fn from(err: AppError) -> Self {
        match &err {
            AppError::FileParsing { .. } => {
                tracing::warn!(error = ?err);
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
            AppError::OpenRouterClient(
                OpenRouterClientError::Request { .. }
                | OpenRouterClientError::ReadResponse { .. }
                | OpenRouterClientError::NoSuccessResponse { .. },
            ) => {
                tracing::error!(error = ?err);
                Self::new(Code::Unavailable, format!("{err}"))
            }
            _ => {
                tracing::error!(error = ?err);
                Self::new(Code::Internal, format!("{err}"))
            }
        }
    }
}
