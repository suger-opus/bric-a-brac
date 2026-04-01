use secrecy::{ExposeSecret, SecretString};
use std::{
    convert::Infallible,
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tonic::{
    service::{interceptor::InterceptedService, Interceptor},
    transport::Channel,
    Request, Status,
};
use tower::{Layer, Service};

// Client-side: injects auth token into outgoing gRPC requests

#[derive(Clone)]
pub struct ServiceAuthInterceptor {
    token: SecretString,
}

impl ServiceAuthInterceptor {
    pub fn new(token: SecretString) -> Self {
        Self { token }
    }
}

impl Interceptor for ServiceAuthInterceptor {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        let bearer = format!("Bearer {}", self.token.expose_secret());
        request.metadata_mut().insert(
            http::header::AUTHORIZATION.as_str(),
            bearer
                .parse()
                .map_err(|_| Status::internal("Invalid auth token format"))?,
        );
        Ok(request)
    }
}

/// Authenticated gRPC channel type for use in client wrappers.
pub type AuthChannel = InterceptedService<Channel, ServiceAuthInterceptor>;

// Server-side: validates auth token on incoming gRPC requests

#[derive(Clone)]
pub struct ServiceAuthLayer {
    expected_bearer: String,
}

impl ServiceAuthLayer {
    pub fn new(token: &SecretString) -> Self {
        Self {
            expected_bearer: format!("Bearer {}", token.expose_secret()),
        }
    }
}

impl<S> Layer<S> for ServiceAuthLayer {
    type Service = ServiceAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ServiceAuthMiddleware {
            inner,
            expected_bearer: self.expected_bearer.clone(),
        }
    }
}

#[derive(Clone)]
pub struct ServiceAuthMiddleware<S> {
    inner: S,
    expected_bearer: String,
}

impl<S, ReqBody, ResBody> Service<http::Request<ReqBody>> for ServiceAuthMiddleware<S>
where
    S: Service<http::Request<ReqBody>, Response = http::Response<ResBody>, Error = Infallible>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
    ResBody: Default + Send + 'static,
{
    type Response = http::Response<ResBody>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: http::Request<ReqBody>) -> Self::Future {
        let authorized = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .is_some_and(|v| v == self.expected_bearer);

        if authorized {
            Box::pin(self.inner.call(req))
        } else {
            tracing::warn!("Rejected unauthenticated gRPC request");
            let response = http::Response::builder()
                .header("content-type", "application/grpc")
                .header("grpc-status", "16")
                .header("grpc-message", "Invalid or missing service auth token")
                .body(ResBody::default())
                .unwrap();
            Box::pin(std::future::ready(Ok(response)))
        }
    }
}
