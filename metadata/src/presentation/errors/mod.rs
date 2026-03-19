mod http_error;

pub use http_error::HttpError;

use crate::application::errors::{AppError, RequestError};
use axum::response::{IntoResponse, Response};

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
