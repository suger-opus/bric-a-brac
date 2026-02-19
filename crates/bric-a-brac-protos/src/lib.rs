// Generated protobuf code for all services
pub mod knowledge {
    tonic::include_proto!("knowledge");
}

pub mod metadata {
    tonic::include_proto!("metadata");
}

pub mod ai {
    tonic::include_proto!("ai");
}

// Base trait & error
mod client;
mod error;
mod tracing;
mod server;

pub use server::build_grpc_server;
pub use client::GrpcClient;
pub use error::{BaseGrpcClientError, GrpcServiceKind};
