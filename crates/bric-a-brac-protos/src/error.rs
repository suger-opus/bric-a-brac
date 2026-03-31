use axum::{
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
#[error("gRPC request failed")]
pub struct GrpcRequestError {
    #[source]
    pub source: tonic::Status,
}

impl IntoResponse for GrpcRequestError {
    fn into_response(self) -> Response {
        tracing::error!(error = ?self.source);
        (
            http::StatusCode::BAD_GATEWAY,
            Json(json!({ "error": "External service request failed" })),
        )
            .into_response()
    }
}

impl From<GrpcRequestError> for tonic::Status {
    fn from(err: GrpcRequestError) -> Self {
        tracing::error!(error = ?err);
        match err.source.code() {
            tonic::Code::Unavailable | tonic::Code::DeadlineExceeded => {
                Self::unavailable("A downstream service is temporarily unavailable")
            }
            _ => Self::internal("Internal server error"),
        }
    }
}
