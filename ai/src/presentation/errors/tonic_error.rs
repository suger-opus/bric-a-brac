use tonic::{Code, Status};

use crate::presentation::errors::AppError;

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
            // gRPC client errors
            _ => TonicError::new(Code::Internal, format!("Application error: {}", err)),
        }
    }
}
