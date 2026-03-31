use crate::{
    application::AppError,
    infrastructure::{DatabaseError, InfraError},
};
use axum::{
    extract::multipart::MultipartError,
    http,
    response::{IntoResponse, Response},
    Json,
};
use bric_a_brac_dtos::DtosConversionError;
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum PresentationError {
    #[error(transparent)]
    AppError(Box<AppError>),

    #[error(transparent)]
    DtosConversionError(#[from] DtosConversionError),

    #[error("Missing required header '{header}'")]
    MissingHeader { header: String },

    #[error("Missing required multipart field '{field}'")]
    MissingMultipartField { field: String },

    #[error("Failed to read multipart field")]
    MultipartReadError {
        #[source]
        source: MultipartError,
    },

    #[error("Invalid multipart file: {reason}")]
    InvalidMultipartFile { reason: String },

    #[error("Failed to read multipart file")]
    MultipartFileReadError {
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}

impl From<AppError> for PresentationError {
    fn from(err: AppError) -> Self {
        Self::AppError(Box::new(err))
    }
}

fn error_response(status: http::StatusCode, message: impl std::fmt::Display) -> Response {
    (status, Json(json!({ "error": message.to_string() }))).into_response()
}

impl IntoResponse for PresentationError {
    #[allow(clippy::cognitive_complexity)]
    fn into_response(self) -> Response {
        match self {
            Self::AppError(err) => err.into_response(),
            Self::DtosConversionError(err) => {
                tracing::warn!(error = ?err);
                error_response(http::StatusCode::BAD_REQUEST, err)
            }
            Self::MissingHeader { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::BAD_REQUEST, self)
            }
            Self::MissingMultipartField { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::BAD_REQUEST, self)
            }
            Self::MultipartReadError { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::BAD_REQUEST, self)
            }
            Self::InvalidMultipartFile { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::BAD_REQUEST, self)
            }
            Self::MultipartFileReadError { .. } => {
                tracing::error!(error = ?self);
                error_response(http::StatusCode::INTERNAL_SERVER_ERROR, self)
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::InfraError(err) => err.into_response(),
            Self::ActiveSessionAlreadyExists => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::CONFLICT, self)
            }
        }
    }
}

impl IntoResponse for InfraError {
    fn into_response(self) -> Response {
        match self {
            Self::DatabaseError(err) => err.into_response(),
            Self::GrpcRequestError(err) => err.into_response(),
            Self::DtosConversionError(err) => {
                tracing::error!(error = ?err);
                error_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal data conversion error",
                )
            }
        }
    }
}

impl IntoResponse for DatabaseError {
    #[allow(clippy::cognitive_complexity)]
    fn into_response(self) -> Response {
        match self {
            Self::NotFound { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::NOT_FOUND, self)
            }
            Self::UniqueConstraintViolation { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::CONFLICT, self)
            }
            Self::PrimaryConstraintViolation { .. } => {
                tracing::warn!(error = ?self);
                error_response(http::StatusCode::BAD_REQUEST, self)
            }
            _ => {
                tracing::error!(error = ?self);
                error_response(
                    http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                )
            }
        }
    }
}

impl From<PresentationError> for tonic::Status {
    #[allow(clippy::cognitive_complexity)]
    fn from(err: PresentationError) -> Self {
        match err {
            PresentationError::AppError(err) => (*err).into(),
            PresentationError::DtosConversionError(err) => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            PresentationError::MissingHeader { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            PresentationError::MissingMultipartField { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            PresentationError::MultipartReadError { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            PresentationError::InvalidMultipartFile { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            PresentationError::MultipartFileReadError { .. } => {
                tracing::error!(error = ?err);
                Self::new(tonic::Code::Internal, err.to_string())
            }
        }
    }
}

impl From<AppError> for tonic::Status {
    fn from(err: AppError) -> Self {
        match err {
            AppError::InfraError(err) => err.into(),
            AppError::ActiveSessionAlreadyExists => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::AlreadyExists, err.to_string())
            }
        }
    }
}

impl From<InfraError> for tonic::Status {
    fn from(err: InfraError) -> Self {
        match err {
            InfraError::DatabaseError(err) => err.into(),
            InfraError::GrpcRequestError(err) => err.into(),
            InfraError::DtosConversionError(err) => {
                tracing::error!(error = ?err);
                Self::new(tonic::Code::Internal, err.to_string())
            }
        }
    }
}

impl From<DatabaseError> for tonic::Status {
    #[allow(clippy::cognitive_complexity)]
    fn from(err: DatabaseError) -> Self {
        match err {
            DatabaseError::NotFound { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::NotFound, err.to_string())
            }
            DatabaseError::UniqueConstraintViolation { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::AlreadyExists, err.to_string())
            }
            DatabaseError::PrimaryConstraintViolation { .. } => {
                tracing::warn!(error = ?err);
                Self::new(tonic::Code::InvalidArgument, err.to_string())
            }
            _ => {
                tracing::error!(error = ?err);
                Self::new(tonic::Code::Internal, err.to_string())
            }
        }
    }
}
