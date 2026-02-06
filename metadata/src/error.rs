use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{error::ErrorKind, postgres::PgDatabaseError};
use tonic::Status;

#[derive(Debug, Serialize)]
pub struct ApiErrorContent<T> {
    pub message: String,
    pub details: T,
}

#[derive(Debug, Serialize)]
pub struct ConstraintViolationContext {
    pub table: String,
    pub column: String,
    pub constraint: String,
}

#[derive(Debug, Serialize)]
pub struct ValidationContext {
    pub field: String,
    pub issue: String,
}

#[derive(Debug)]
pub enum ApiError {
    ConstraintViolation(ApiErrorContent<ConstraintViolationContext>),
    NotFound(ApiErrorContent<String>),
    Conflict(ApiErrorContent<ConstraintViolationContext>),
    Unauthorized(ApiErrorContent<String>),
    UnknownDatabaseError(ApiErrorContent<sqlx::Error>),
    ValidationError(ApiErrorContent<ValidationContext>),
    KnowledgeError(ApiErrorContent<String>),
}

impl From<&ApiError> for StatusCode {
    fn from(val: &ApiError) -> Self {
        match val {
            ApiError::ConstraintViolation(_) => StatusCode::BAD_REQUEST,
            ApiError::ValidationError(_) => StatusCode::BAD_REQUEST,
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::UnknownDatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::KnowledgeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!(error=?self, "request failed");
        let status = StatusCode::from(&self);
        let payload = match self {
            Self::ConstraintViolation(content) => Json(json!(content)),
            Self::ValidationError(content) => Json(json!(content)),
            Self::NotFound(content) => Json(json!(content)),
            Self::Conflict(content) => Json(json!(content)),
            Self::Unauthorized(content) => Json(json!(content)),
            Self::UnknownDatabaseError(content) => Json(json!(ApiErrorContent {
                message: content.message,
                details: content.details.to_string(),
            })),
            Self::KnowledgeError(content) => Json(json!(content)),
        };
        (status, payload).into_response()
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!(error=?err, "database error occurred");
        match err {
            sqlx::Error::RowNotFound => ApiError::NotFound(ApiErrorContent {
                message: "Element not found".to_string(),
                details: "An element should have been returned, but none was found".to_string(),
            }),
            sqlx::Error::Database(ref db_err) => {
                let pg_err = db_err.downcast_ref::<PgDatabaseError>();
                let details = pg_err.detail().unwrap_or("unknown").to_string();
                let table = pg_err.table().unwrap_or("unknown").to_string();
                let constraint = pg_err.constraint().unwrap_or("unknown").to_string();
                let column = extract_field_name_from_error(pg_err);
                match db_err.kind() {
                    ErrorKind::UniqueViolation => ApiError::Conflict(ApiErrorContent {
                        message: details,
                        details: ConstraintViolationContext {
                            table,
                            column,
                            constraint: "unique".to_string(),
                        },
                    }),
                    ErrorKind::NotNullViolation => ApiError::ConstraintViolation(ApiErrorContent {
                        message: details,
                        details: ConstraintViolationContext {
                            table,
                            column,
                            constraint: "not null".to_string(),
                        },
                    }),
                    ErrorKind::CheckViolation => Self::ConstraintViolation(ApiErrorContent {
                        message: details,
                        details: ConstraintViolationContext {
                            table,
                            column,
                            constraint,
                        },
                    }),
                    _ => Self::UnknownDatabaseError(ApiErrorContent {
                        message: "Unknown Database Error".to_string(),
                        details: err,
                    }),
                }
            }
            _ => Self::UnknownDatabaseError(ApiErrorContent {
                message: "Unknown Database Error".to_string(),
                details: err,
            }),
        }
    }
}

fn extract_field_name_from_error(pg_err: &PgDatabaseError) -> String {
    // Constraint names follow pattern: tablename_fieldname_key
    // Remove "_key" suffix, then skip table name prefix
    pg_err
        .constraint()
        .and_then(|constraint| {
            constraint
                .strip_suffix("_key")
                .and_then(|s| s.split_once('_').map(|(_, field)| field))
        })
        .unwrap_or("unknown")
        .to_string()
}

impl From<Status> for ApiError {
    fn from(status: Status) -> Self {
        tracing::error!(error=?status, "gRPC status error occurred");
        ApiError::KnowledgeError(ApiErrorContent {
            message: "(knowledge) gRPC Status Error".to_string(),
            details: status.message().to_string(),
        })
    }
}
