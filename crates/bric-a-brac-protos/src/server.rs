use super::{tracing::grpc_tracing_layer, ServiceAuthLayer};
use secrecy::SecretString;
use std::{future::Future, net::SocketAddr};
use tonic::server::NamedService;

pub fn build_grpc_server<S>(
    service: S,
    addr: SocketAddr,
    auth_token: &SecretString,
) -> impl Future<Output = Result<(), tonic::transport::Error>>
where
    S: tower::Service<
            http::Request<tonic::body::Body>,
            Response = http::Response<tonic::body::Body>,
            Error = std::convert::Infallible,
        > + NamedService
        + Clone
        + Send
        + Sync
        + 'static,
    S::Future: Send + 'static,
{
    tonic::transport::Server::builder()
        .layer(grpc_tracing_layer())
        .layer(ServiceAuthLayer::new(auth_token))
        .add_service(service)
        .serve(addr)
}
