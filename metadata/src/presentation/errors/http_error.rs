use super::{AppError, DatabaseError, GrpcClientError, RequestError, ServiceKind};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
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
            message: message.to_string(),
            details,
        }
    }

    // 400 - { field, issue, value }
    pub fn bad_request(field: &str, issue: &str) -> Self {
        Self::new(
            StatusCode::BAD_REQUEST,
            "Invalid request",
            json!({ "field": field, "issue": issue }),
        )
    }

    // 401 - { reason }
    pub fn unauthorized(reason: &str) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "Unauthorized",
            json!({ "reason": reason }),
        )
    }

    // // 403 - { user_id, action }
    // pub fn forbidden(message: Option<&str>, user_id: &str, action: &str) -> Self {
    //     let message = message.unwrap_or("Insufficient permissions");
    //     Self::new(StatusCode::FORBIDDEN, message,
    //         json!({ "user_id": user_id, "action": action }))
    // }

    // 404 - {}
    pub fn not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, "Not Found", json!({}))
    }

    // 409 - { resource, field, detail }
    pub fn conflict(resource: &str, field: &str, detail: &str) -> Self {
        Self::new(
            StatusCode::CONFLICT,
            "Element already exists",
            json!({ "resource": resource, "field": field, "detail": detail }),
        )
    }

    // 500 - { detail }
    pub fn internal_server_error(detail: &str) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
            json!({ "detail": detail }),
        )
    }

    // 502 - { service, detail }
    pub fn bad_gateway(message: &str, service: ServiceKind, detail: &str) -> Self {
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
                HttpError::not_found()
            }
            AppError::Database(DatabaseError::UniqueConstraintViolation {
                table,
                column,
                detail,
                source: _,
            }) => {
                tracing::warn!(error = ?err, "Request failed: (Database) Unique Constraint Violation");
                HttpError::conflict(table, column, detail)
            }
            AppError::Database(DatabaseError::BusinessConstraintViolation {
                table,
                column,
                detail,
                source: _,
                constraint,
            }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Business Constraint Violation");
                HttpError::internal_server_error(
                    format!(
                        "Database business constraint violation ({}): {}.{}: {}",
                        constraint, table, column, detail
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
                HttpError::bad_request(
                    format!("{}.{}", table, column).as_str(),
                    format!(
                        "Database primary constraint violation ({}): {}.{}: {}",
                        constraint, table, column, detail
                    )
                    .as_str(),
                )
            }
            AppError::Database(DatabaseError::UnexpectedState { reason }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Unexpected State");
                HttpError::internal_server_error(
                    format!("Unexpected database state: {}", reason).as_str(),
                )
            }
            AppError::Database(DatabaseError::Unknown { source: _ }) => {
                tracing::error!(error = ?err, "Request failed: (Database) Unknown");
                HttpError::internal_server_error("Unknown database error")
            }

            // --- gRPC service errors ---
            AppError::GrpcService(GrpcClientError::Inaccessible { service, source: _ }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Inaccessible");
                HttpError::bad_gateway("External service not accessible", *service, "")
            }
            AppError::GrpcService(GrpcClientError::Disconnected { service }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Disconnected");
                HttpError::bad_gateway("External service disconnected", *service, "")
            }
            AppError::GrpcService(GrpcClientError::Deserialization {
                service,
                expected_struct,
                source: _,
            }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Deserialization");
                HttpError::bad_gateway(
                    "External service response deserialization failed",
                    *service,
                    expected_struct,
                )
            }
            AppError::GrpcService(GrpcClientError::Request {
                service,
                message,
                source: _,
            }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Request");
                HttpError::bad_gateway("External service request failed", *service, message)
            }
            AppError::GrpcService(GrpcClientError::DomainConversion { service, reason }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Domain Conversion");
                HttpError::bad_gateway(
                    "External service response conversion to domain model failed",
                    *service,
                    reason,
                )
            }
            AppError::GrpcService(GrpcClientError::DomainUuidConversion { service, source: _ }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Domain UUID Conversion");
                HttpError::bad_gateway(
                    "External service response UUID conversion to domain UUID failed",
                    *service,
                    "UUID conversion error",
                )
            }
            AppError::GrpcService(GrpcClientError::MutexPoisoned { message }) => {
                tracing::error!(error = ?err, "Request failed: (gRPC) Mutex Poisoned");
                HttpError::internal_server_error(
                    format!("Synchronization failed: {}", message).as_str(),
                )
            }

            // --- Request errors ---
            AppError::Request(RequestError::Unauthorized { reason }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Unauthorized");
                HttpError::unauthorized(reason)
            }
            AppError::Request(RequestError::InvalidHeader { issue, header }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Invalid Header");
                HttpError::bad_request(format!("Header {header}").as_str(), issue)
            }
            AppError::Request(RequestError::InvalidFile { issue }) => {
                tracing::warn!(error = ?err, "Request failed: (Domain) Invalid File");
                HttpError::bad_request("Body file", issue)
            }
            AppError::Request(RequestError::InvalidInput { field, issue }) => {
                tracing::warn!(error = ?err, "Request failed: (gRPC) Invalid Input");
                HttpError::bad_request(field, issue)
            }
        }
    }
}
