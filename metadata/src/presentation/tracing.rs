use axum::http;
use tower_http::{
    classify::{ServerErrorsAsFailures, ServerErrorsFailureClass, SharedClassifier},
    trace::{DefaultOnBodyChunk, DefaultOnEos, TraceLayer},
};
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

pub fn setup() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "warn,metadata=debug,bric_a_brac_protos=info".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(true)
                .with_thread_ids(true)
                .with_line_number(true)
                .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE),
        )
        .init();
}

#[derive(Clone)]
pub struct HttpMakeSpan;
impl<B> tower_http::trace::MakeSpan<B> for HttpMakeSpan {
    fn make_span(&mut self, _request: &http::Request<B>) -> tracing::Span {
        let request_id = Uuid::new_v4();
        tracing::info_span!("HTTP", id = ?request_id)
    }
}

#[derive(Clone)]
pub struct HttpOnRequest;
impl<B> tower_http::trace::OnRequest<B> for HttpOnRequest {
    fn on_request(&mut self, request: &http::Request<B>, span: &tracing::Span) {
        tracing::info!(parent: span,
            method = ?request.method(),
            uri = ?request.uri(),
            headers = ?request.headers(),
            "HTTP request received"
        );
    }
}

#[derive(Clone)]
pub struct HttpOnResponse;
impl<B> tower_http::trace::OnResponse<B> for HttpOnResponse {
    fn on_response(
        self,
        response: &http::Response<B>,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        tracing::info!(parent: span,
            status = ?response.status(),
            latency = ?latency,
            headers = ?response.headers(),
            "HTTP response sent"
        );
    }
}

#[derive(Clone)]
pub struct HttpOnFailure;
impl tower_http::trace::OnFailure<ServerErrorsFailureClass> for HttpOnFailure {
    fn on_failure(
        &mut self,
        error: ServerErrorsFailureClass,
        latency: std::time::Duration,
        span: &tracing::Span,
    ) {
        tracing::error!(parent: span,
            latency = ?latency,
            error = ?error,
            "HTTP request failed"
        );
    }
}

pub type HttpTraceLayer = TraceLayer<
    SharedClassifier<ServerErrorsAsFailures>,
    HttpMakeSpan,
    HttpOnRequest,
    HttpOnResponse,
    DefaultOnBodyChunk,
    DefaultOnEos,
    HttpOnFailure,
>;

pub fn http_tracing_layer() -> HttpTraceLayer {
    TraceLayer::new_for_http()
        .make_span_with(HttpMakeSpan)
        .on_request(HttpOnRequest)
        .on_response(HttpOnResponse)
        .on_failure(HttpOnFailure)
}
