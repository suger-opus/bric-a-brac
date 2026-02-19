use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize, derive_more::Display)]
pub enum GrpcServiceKind {
    Ai,
    Knowledge,
    Metadata,
}

#[derive(Debug, thiserror::Error)]
pub enum BaseGrpcClientError {
    #[error("gRPC service {service}: disconnected")]
    Disconnected { service: GrpcServiceKind },

    #[error("Mutex lock poisoned")]
    MutexPoisoned { message: String },

    #[error("gRPC service {service}: inaccessible")]
    Inaccessible {
        service: GrpcServiceKind,
        #[source]
        source: tonic::transport::Error,
    },

    #[error("gRPC service {service}: request failed - {message}")]
    Request {
        service: GrpcServiceKind,
        message: String,
        #[source]
        source: tonic::Status,
    },
}

impl BaseGrpcClientError {
    pub fn is_connection_error(&self) -> bool {
        match self {
            BaseGrpcClientError::Inaccessible { .. } => true,
            BaseGrpcClientError::Disconnected { .. } => true,
            BaseGrpcClientError::Request { source, .. } => {
                return matches!(
                    source.code(),
                    tonic::Code::Unavailable
                        | tonic::Code::DeadlineExceeded
                        | tonic::Code::Cancelled
                        | tonic::Code::Unknown
                );
            }
            _ => false,
        }
    }
}
