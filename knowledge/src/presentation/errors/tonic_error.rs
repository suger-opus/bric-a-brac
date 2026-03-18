use super::AppError;
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
            AppError::NotFound { .. } => TonicError::new(Code::NotFound, format!("{}", err)),
            AppError::InvalidInput { .. } => {
                TonicError::new(Code::InvalidArgument, format!("{}", err))
            }
            AppError::Conversion(_) => TonicError::new(Code::InvalidArgument, format!("{}", err)),
            _ => TonicError::new(Code::Internal, format!("Application error: {}", err)),
        }
    }
}
