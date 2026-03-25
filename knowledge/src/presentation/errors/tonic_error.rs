use crate::application::errors::AppError;
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
            AppError::NotFound { .. } | AppError::InvalidInput { .. } | AppError::Conversion(_) => {
                tracing::warn!(error = ?err);
            }
            _ => {
                tracing::error!(error = ?err);
            }
        }
        match &err {
            AppError::NotFound { .. } => Self::new(Code::NotFound, format!("{err}")),
            AppError::InvalidInput { .. } | AppError::Conversion(_) => {
                Self::new(Code::InvalidArgument, format!("{err}"))
            }
            _ => Self::new(Code::Internal, format!("Application error: {err}")),
        }
    }
}
