use crate::{application::AppError, infrastructure::AiClient};
use bric_a_brac_dtos::{AgentEventDto, SessionDocumentIdDto, SessionIdDto};
use futures_util::Stream;

#[derive(Clone)]
pub struct ChatService {
    client: AiClient,
}

impl ChatService {
    pub const fn new(client: AiClient) -> Self {
        Self { client }
    }

    #[tracing::instrument(
        level = "trace",
        name = "chat_service.send_message",
        skip(self, session_id, content, document_id),
        err
    )]
    pub async fn send_message(
        &self,
        session_id: SessionIdDto,
        content: Option<String>,
        document_id: Option<SessionDocumentIdDto>,
    ) -> Result<impl Stream<Item = AgentEventDto>, AppError> {
        self.client
            .send_message(session_id, content.unwrap_or_default(), document_id)
            .await
            .map_err(From::from)
    }
}
