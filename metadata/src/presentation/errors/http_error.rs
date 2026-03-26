use crate::{
    application::errors::{AppError, RequestError},
    infrastructure::errors::{DatabaseError, GrpcClientError},
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use bric_a_brac_protos::{BaseGrpcClientError, GrpcServiceKind};
use serde_json::{json, Value};

pub struct HttpError {
    status: StatusCode,
    message: String,
    details: Value,
}

impl HttpError {
    fn new(status: StatusCode, message: &str, details: Value) -> Self {
        Self {
            status,
            message: message.to_owned(),
            details,
        }
    }

    // 400 - { field, issue }
    fn bad_request(field: &str, issue: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "Invalid request",
            json!({ "field": field, "issue": issue }),
        )
    }

    // 401 - { reason }
    fn unauthorized(reason: &str) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "Unauthorized",
            json!({ "reason": reason }),
        )
    }

    // 404 - {}
    fn not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, "Not Found", json!({}))
    }

    // 409 - { resource, field, detail }
    fn conflict(resource: &str, field: &str, detail: &str) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "Element already exists",
            json!({ "resource": resource, "field": field, "detail": detail }),
        )
    }

    // 500 - { detail }
    fn internal_server_error(detail: &str) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            json!({ "detail": detail }),
        )
    }

    // 502 - { service, detail }
    fn bad_gateway(message: &str, service: GrpcServiceKind, detail: &str) -> Self {
        Self::new(
            StatusCode::BAD_GATEWAY,
            message,
            json!({ "service": service, "detail": detail }),
        )
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": {
                "message": self.message,
                "details": self.details,
            }
        }));

        (self.status, body).into_response()
    }
}

impl From<AppError> for HttpError {
    fn from(err: AppError) -> Self {
        match &err {
            // --- Database errors ---
            AppError::Database(DatabaseError::NotFound { source: _ }) => {
                tracing::warn!(error = ?err, "Request failed: (Database) Not Found");
                Self::not_found()
            }
            AppError::Database(DatabaseError::UniqueConstraintViolation {
                table,
                column,
                detail,
                source: _,
            }) => {
                tracing::warn!(error = ?err, "Request failed: (Database) Unique Constraint Violation");
                Self::conflict(table, column, detail)
            }
            AppError::Database(DatabaseError::BusinessConstraintViolation {
                table,
                column,
                detail,
                source: _,
                constraint,
            }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Business Constraint Violation");
                Self::internal_server_error(
                    format!(
                        "Database business constraint violation ({constraint}): {table}.{column}: {detail}"
                    )
                    .as_str(),
                )
            }
            AppError::Database(DatabaseError::PrimaryConstraintViolation {
                table,
                column,
                detail,
                source: _,
                constraint,
            }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Primary Constraint Violation");
                Self::bad_request(
                    format!("{table}.{column}").as_str(),
                    format!(
                        "Database primary constraint violation ({constraint}): {table}.{column}: {detail}"
                    )
                    .as_str(),
                )
            }
            AppError::Database(DatabaseError::UnexpectedState { reason }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Unexpected State");
                Self::internal_server_error(format!("Unexpected database state: {reason}").as_str())
            }
            AppError::Database(DatabaseError::Unknown { source: _ }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Unknown");
                Self::internal_server_error("Unknown database error")
            }

            // --- gRPC client errors ---
            AppError::GrpcClient(GrpcClientError::Base(BaseGrpcClientError::Request {
                service,
                message,
                source: _,
            })) => {
                tracing::error!(error = ?err, "Request failed: gRPC client error");
                Self::bad_gateway("External service request failed", *service, message)
            }
            AppError::GrpcClient(GrpcClientError::Conversion(_)) => {
                tracing::error!(error = ?err, "Request failed: gRPC DTO conversion error");
                Self::internal_server_error("Failed to convert data from gRPC service")
            }

            // --- Request errors ---
            AppError::Request(RequestError::Unauthorized { reason, source: _ }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Unauthorized");
                Self::unauthorized(reason)
            }
            AppError::Request(RequestError::InvalidHeader { issue, header }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Invalid Header");
                Self::bad_request(format!("Header {header}").as_str(), issue)
            }
            AppError::Request(RequestError::InvalidFile { issue, source: _ }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Invalid File");
                Self::bad_request("Body file", issue)
            }
            AppError::Request(RequestError::InvalidInput {
                field,
                issue,
                source: _,
            }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Invalid Input");
                Self::bad_request(field, issue)
            }
        }
    }
}
