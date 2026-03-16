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
        Status::new(err.code, err.message)
    }
}

impl From<AppError> for TonicError {
    fn from(err: AppError) -> Self {
        tracing::error!(error = ?err);
        match &err {
            AppError::FileParsing { .. } => {
                TonicError::new(Code::InvalidArgument, format!("{}", err))
            }
            AppError::OpenRouterClient(
                OpenRouterClientError::Request { .. }
                | OpenRouterClientError::ReadResponse { .. }
                | OpenRouterClientError::NoSuccessResponse { .. },
            ) => TonicError::new(Code::Unavailable, format!("{}", err)),
            _ => TonicError::new(Code::Internal, format!("{}", err)),
        }
    }
}
