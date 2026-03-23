use crate::infrastructure::{config::AiServerConfig, errors::GrpcClientError};
use bric_a_brac_protos::{
    ai::{ai_client::AiClient as AiGrpcClient, AgentEventProto, SendMessageRequest},
    with_retry, GrpcServiceKind,
};
use tonic::transport::Channel;
use tonic::{Request, Streaming};

#[derive(Clone)]
pub struct AiClient {
    client: AiGrpcClient<Channel>,
}

impl AiClient {
    pub fn new(config: AiServerConfig) -> anyhow::Result<Self> {
        let channel = tonic::transport::Endpoint::from_shared(config.url().to_string())?
            .connect_lazy();
        Ok(Self {
            client: AiGrpcClient::new(channel),
        })
    }

    pub async fn send_message(
        &self,
        session_id: String,
        content: String,
        document_id: Option<String>,
    ) -> Result<Streaming<AgentEventProto>, GrpcClientError> {
        let client = self.client.clone();
        with_retry(
            GrpcServiceKind::Ai,
            "Failed to send message to AI agent",
            || {
                let mut c = client.clone();
                let req = Request::new(SendMessageRequest {
                    session_id: session_id.clone(),
                    content: content.clone(),
                    document_id: document_id.clone(),
                });
                async move { c.send_message(req).await }
            },
        )
        .await
        .map_err(Into::into)
    }
}
