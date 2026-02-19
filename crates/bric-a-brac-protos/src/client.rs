use crate::error::{BaseGrpcClientError, GrpcServiceKind};
use http::Uri;
use std::sync::{Arc, Mutex};

#[tonic::async_trait]
pub trait GrpcClient {
    type Client: Clone;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>>;
    fn service_kind(&self) -> GrpcServiceKind;
    fn url(&self) -> &Uri;
    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error>;

    #[tracing::instrument(level = "trace", skip(self))]
    fn reset_connection(&self) {
        if let Ok(mut client_lock) = self.client().lock() {
            *client_lock = None;
            tracing::warn!("Reset {} service connection", self.service_kind());
        }
    }

    #[tracing::instrument(level = "trace", skip(self))]
    async fn ensure_connection(&self) -> Result<(), BaseGrpcClientError> {
        {
            let client_lock = self.acquire_client()?;
            if client_lock.is_some() {
                return Ok(());
            }
        } // Lock dropped here - Holding mutex guards across .await is an anti-pattern
        tracing::info!(url = %self.url(), "Connection to {} service not yet established, connecting...", self.service_kind());

        let client = self.connect().await.map_err(|err| {
            tracing::error!(
                error = ?err,
                "Failed to connect to {} service",
                self.service_kind()
            );
            BaseGrpcClientError::Inaccessible {
                service: self.service_kind(),
                source: err,
            }
        })?;
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
    fn clone_client(&self) -> Result<Self::Client, BaseGrpcClientError> {
        let client_lock = self.acquire_client()?;

        client_lock
            .as_ref()
            .ok_or_else(|| {
                tracing::warn!(
                    "Attempted to clone {} client but connection is not established",
                    self.service_kind()
                );
                BaseGrpcClientError::Disconnected {
                    service: self.service_kind(),
                }
            })
            .map(|client| client.clone()) // Clone the client to avoid holding the lock across await
    }

    #[tracing::instrument(level = "trace", skip(self))]
    fn acquire_client(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, Option<Self::Client>>, BaseGrpcClientError> {
        self.client().lock().map_err(|err| {
            tracing::error!(error = ?err, "Failed to acquire {} client lock", self.service_kind());
            BaseGrpcClientError::MutexPoisoned {
                message: format!(
                    "Failed to acquire {} client lock: {}",
                    self.service_kind(),
                    err
                ),
            }
        })
    }
}
