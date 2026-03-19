mod tonic_error;

pub use tonic_error::TonicError;

use crate::application::errors::AppError;

impl From<AppError> for tonic::Status {
    fn from(err: AppError) -> Self {
        TonicError::from(err).into()
    }
}
