use crate::infrastructure::errors::OpenRouterClientError;
use std::time::Duration;

const MAX_RETRIES: u32 = 2;
const BASE_DELAY_MS: u64 = 200;

const fn is_retryable_status(status: reqwest::StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 500 | 502 | 503 | 504)
}

fn is_retryable_error(err: &reqwest::Error) -> bool {
    err.is_timeout() || err.is_connect()
}

fn parse_retry_after(response: &reqwest::Response) -> Option<Duration> {
    response
        .headers()
        .get("retry-after")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.parse::<u64>().ok())
        .map(Duration::from_secs)
}

/// Send an HTTP request with retry on transient errors (429, 5xx, timeouts).
///
/// The closure must return a fresh `RequestBuilder` on each call because
/// `send()` consumes the builder.
pub async fn send_with_retry<F>(
    operation: &str,
    build_request: F,
) -> Result<reqwest::Response, OpenRouterClientError>
where
    F: Fn() -> reqwest::RequestBuilder,
{
    for attempt in 0..=MAX_RETRIES {
        match build_request().send().await {
            Ok(response) if is_retryable_status(response.status()) && attempt < MAX_RETRIES => {
                let status = response.status();
                let delay = parse_retry_after(&response)
                    .unwrap_or_else(|| Duration::from_millis(BASE_DELAY_MS * 2u64.pow(attempt)));

                tracing::warn!(
                    attempt = attempt + 1,
                    %status,
                    "{operation}: retryable HTTP status, retrying"
                );
                tokio::time::sleep(delay).await;
            }
            Ok(response) => return Ok(response),
            Err(err) if is_retryable_error(&err) && attempt < MAX_RETRIES => {
                let delay = Duration::from_millis(BASE_DELAY_MS * 2u64.pow(attempt));
                tracing::warn!(
                    attempt = attempt + 1,
                    "{operation}: transient network error, retrying"
                );
                tokio::time::sleep(delay).await;
            }
            Err(err) => {
                return Err(OpenRouterClientError::Request {
                    message: operation.to_owned(),
                    source: err,
                });
            }
        }
    }

    unreachable!("retry loop always returns or continues")
}
