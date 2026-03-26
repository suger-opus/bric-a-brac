use crate::{
    application::{dtos::AgentEventDto, errors::AppError},
    infrastructure::clients::AiClient,
};
use futures_util::{Stream, StreamExt};

#[derive(Clone)]
pub struct AiService {
    client: AiClient,
}

impl AiService {
    pub const fn new(client: AiClient) -> Self {
        Self { client }
    }

    #[tracing::instrument(level = "trace", name = "ai_service.chat", skip(self, session_id, content, document_id), err)]
    pub async fn chat(
        &self,
        session_id: String,
        content: String,
        document_id: Option<String>,
    ) -> Result<impl Stream<Item = AgentEventDto>, AppError> {
        let stream = self.client.send_message(session_id, content, document_id).await?;

        Ok(stream.map(|result| match result {
            Ok(event) => AgentEventDto::from(event.event),
            Err(status) => AgentEventDto::Error {
                message: status.message().to_owned(),
            },
        }))
    }
}
