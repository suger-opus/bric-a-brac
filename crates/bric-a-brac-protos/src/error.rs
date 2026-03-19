use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, derive_more::Display)]
pub enum GrpcServiceKind {
    Ai,
    Knowledge,
    Metadata,
}

#[derive(Debug, thiserror::Error)]
pub enum BaseGrpcClientError {
    #[error("gRPC service {service}: request failed - {message}")]
    Request {
        service: GrpcServiceKind,
        message: String,
        #[source]
        source: tonic::Status,
    },
}
