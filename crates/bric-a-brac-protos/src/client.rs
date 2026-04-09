use crate::error::GrpcRequestError;
use std::time::Duration;

const MAX_RETRIES: u32 = 2;
const BASE_DELAY_MS: u64 = 100;

/// Retry a gRPC call on transient errors (`Unavailable`, `DeadlineExceeded`).
///
/// Attempts up to `MAX_RETRIES` + 1 times with exponential backoff.
/// The closure must return a new future on each call (clone the client inside).
pub async fn with_retry<F, Fut, T>(f: F) -> Result<T, GrpcRequestError>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<tonic::Response<T>, tonic::Status>>,
{
    for attempt in 0..=MAX_RETRIES {
        match f().await {
            Ok(response) => return Ok(response.into_inner()),
            Err(status) => {
                let retryable = matches!(
                    status.code(),
                    tonic::Code::Unavailable | tonic::Code::DeadlineExceeded
                );

                if !retryable || attempt == MAX_RETRIES {
                    return Err(GrpcRequestError { source: status });
                }

                tracing::warn!(
                    attempt = attempt + 1,
                    code = ?status.code(),
                    "gRPC request failed: transient error, retrying"
                );

                tokio::time::sleep(Duration::from_millis(BASE_DELAY_MS * 2u64.pow(attempt))).await;
            }
        }
    }

    unreachable!("loop always returns via Ok or Err")
}
