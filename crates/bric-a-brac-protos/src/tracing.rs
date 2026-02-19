use tower_http::{
    classify::{GrpcErrorsAsFailures, GrpcFailureClass, SharedClassifier},
    trace::{DefaultOnBodyChunk, DefaultOnEos, TraceLayer},
};
use uuid::Uuid;

#[derive(Clone)]
pub struct GrpcMakeSpan;
impl<B> tower_http::trace::MakeSpan<B> for GrpcMakeSpan {
    fn make_span(&mut self, _request: &http::Request<B>) -> tracing::Span {
        let request_id = Uuid::new_v4();
        tracing::info_span!("gRPC", id = ?request_id)
    }
}

#[derive(Clone)]
pub struct GrpcOnRequest;
impl<B> tower_http::trace::OnRequest<B> for GrpcOnRequest {
    fn on_request(&mut self, request: &http::Request<B>, span: &tracing::Span) {
        tracing::info!(parent: span,
            method = ?request.method(),
            uri = ?request.uri(),
            "gRPC request received"
        );
    }
}

#[derive(Clone)]
pub struct GrpcOnResponse;
impl<B> tower_http::trace::OnResponse<B> for GrpcOnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        tracing::info!(parent: span,
            status = ?response.status(),
            latency = ?latency,
            "gRPC response sent"
        );
    }
}

#[derive(Clone)]
pub struct GrpcOnFailure;
impl tower_http::trace::OnFailure<GrpcFailureClass> for GrpcOnFailure {
    fn on_failure(
        &mut self,
        error: GrpcFailureClass,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        tracing::error!(parent: span,
            latency = ?latency,
            error = ?error,
            "gRPC request failed"
        );
    }
}

pub type GrpcTraceLayer = TraceLayer<
    SharedClassifier<GrpcErrorsAsFailures>,
    GrpcMakeSpan,
    GrpcOnRequest,
    GrpcOnResponse,
    DefaultOnBodyChunk,
    DefaultOnEos,
    GrpcOnFailure,
>;

pub fn grpc_tracing_layer() -> GrpcTraceLayer {
    TraceLayer::new_for_grpc()
        .make_span_with(GrpcMakeSpan)
        .on_request(GrpcOnRequest)
        .on_response(GrpcOnResponse)
        .on_failure(GrpcOnFailure)
}
