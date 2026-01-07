use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use crate::dto::ErrorResponse;

// Custom error type for better error handling
pub enum AppError {
    KnowledgeService(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::KnowledgeService(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::KnowledgeService(err.to_string())
    }
}
