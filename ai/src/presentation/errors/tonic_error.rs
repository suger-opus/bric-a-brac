use super::{AppError, OpenRouterClientError};
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
        tracing::error!(error = ?err);
        match &err {
            AppError::FileParsing { .. } => {
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
            AppError::OpenRouterClient(
                OpenRouterClientError::Request { .. }
                | OpenRouterClientError::ReadResponse { .. }
                | OpenRouterClientError::NoSuccessResponse { .. },
            ) => Self::new(Code::Unavailable, format!("{err}")),
            _ => Self::new(Code::Internal, format!("{err}")),
        }
    }
}
