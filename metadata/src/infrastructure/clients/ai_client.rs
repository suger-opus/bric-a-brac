use crate::infrastructure::{AiServerConfig, InfraError};
use bric_a_brac_dtos::{AgentEventDto, SessionDocumentIdDto, SessionIdDto};
use bric_a_brac_protos::{
    ai::{ai_client::AiClient as AiGrpcClient, SendMessageRequest},
    with_retry, AuthChannel, ServiceAuthInterceptor,
};
use futures_util::{Stream, StreamExt};
use secrecy::SecretString;
use tonic::Request;

#[derive(Clone)]
pub struct AiClient {
    client: AiGrpcClient<AuthChannel>,
}

impl AiClient {
    pub fn new(config: &AiServerConfig, auth_token: &SecretString) -> anyhow::Result<Self> {
        let channel =
            tonic::transport::Endpoint::from_shared(config.url().to_string())?.connect_lazy();
        Ok(Self {
            client: AiGrpcClient::with_interceptor(
                channel,
                ServiceAuthInterceptor::new(auth_token.clone()),
            ),
        })
    }

    pub async fn send_message(
        &self,
        session_id: SessionIdDto,
        content: String,
        document_id: Option<SessionDocumentIdDto>,
    ) -> Result<impl Stream<Item = AgentEventDto>, InfraError> {
        let stream = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(SendMessageRequest {
                session_id: session_id.to_string(),
                content: content.clone(),
                document_id: document_id.map(|id| id.to_string()),
            });
            async move { c.send_message(req).await }
        })
        .await?;

        Ok(stream.map(|result| match result {
            Ok(event) => AgentEventDto::from(event.event),
            Err(status) => AgentEventDto::Error {
                message: status.message().to_owned(),
            },
        }))
    }
}
