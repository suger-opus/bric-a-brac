mod http_error;

use crate::application::errors::{AppError, RequestError};
use axum::response::{IntoResponse, Response};
pub use http_error::HttpError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        HttpError::from(self).into_response()
    }
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        HttpError::from(AppError::from(self)).into_response()
    }
}
