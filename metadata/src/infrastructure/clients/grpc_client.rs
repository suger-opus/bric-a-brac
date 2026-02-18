use crate::presentation::errors::{GrpcClientError, ServiceKind};
use axum::http::Uri;
use std::sync::{Arc, Mutex};
use tonic::async_trait;

#[async_trait]
pub trait GrpcClient {
    type Client: Clone;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>>;
    fn service_kind(&self) -> ServiceKind;
    fn url(&self) -> &Uri;
    async fn connect(&self) -> Result<Self::Client, GrpcClientError>;

    #[tracing::instrument(level = "trace", skip(self))]
    fn reset_connection(&self) {
        if let Ok(mut client_lock) = self.client().lock() {
            *client_lock = None;
            tracing::info!("Reset {} service connection", self.service_kind());
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn ensure_connection(&self) -> Result<(), GrpcClientError> {
        {
            let client_lock = self.acquire_client()?;
            if client_lock.is_some() {
                return Ok(());
            }
        } // Lock dropped here - Holding mutex guards across .await is an anti-pattern
        tracing::info!(url = %self.url(), "Connection to {} service not yet established, connecting...", self.service_kind());

        let client = self.connect().await?;
        {
            let mut client_lock = self.acquire_client()?;
            *client_lock = Some(client);
        }

        tracing::info!(
            url = %self.url(),
            "Successfully connected to {} service",
            self.service_kind()
        );

        Ok(())
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn clone_client(&self) -> Result<Self::Client, GrpcClientError> {
        let client_lock = self.acquire_client()?;

        client_lock
            .as_ref()
            .ok_or_else(|| GrpcClientError::Disconnected {
                service: self.service_kind(),
            })
            .map(|client| client.clone()) // Clone the client to avoid holding the lock across await
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn acquire_client(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, Option<Self::Client>>, GrpcClientError> {
        self.client()
            .lock()
            .map_err(|err| GrpcClientError::MutexPoisoned {
                message: format!(
                    "Failed to acquire {} client lock: {}",
                    self.service_kind(),
                    err
                ),
            })
    }
}
