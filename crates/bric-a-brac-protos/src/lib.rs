// Generated protobuf code for all services
pub mod common {
    tonic::include_proto!("common");
}

pub mod ai {
    tonic::include_proto!("ai");
}

pub mod knowledge {
    tonic::include_proto!("knowledge");
}

pub mod metadata {
    tonic::include_proto!("metadata");
}

// Base trait & error
mod client;
mod error;
mod server;
mod tracing;

pub use client::with_retry;
pub use error::GrpcRequestError;
pub use server::build_grpc_server;
