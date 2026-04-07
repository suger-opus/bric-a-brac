use crate::{
    application::CreateSessionDocumentDto,
    presentation::{
        error::PresentationError,
        http::{ApiState, AuthenticatedUser, ChatMessageUpload},
    },
};
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use bric_a_brac_dtos::GraphIdDto;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::convert::Infallible;

#[tracing::instrument(
    level = "trace",
    name = "chat_handler.chat",
    skip(state, user_id, _graph_id, session_id, content, file_text, file_name)
)]
pub async fn chat(
    State(state): State<ApiState>,
    Path(_graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    ChatMessageUpload {
        session_id,
        content,
        file_text,
        file_name,
    }: ChatMessageUpload,
) -> Result<Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>>, PresentationError> {
    // Verify the user owns this session
    state
        .session_service
        .get_session(session_id, user_id)
        .await
        .map_err(PresentationError::from)?;

    // If a file was uploaded, save it as a session document and pass its ID.
    // The AI service will load the content on its own.
    let document_id = if let Some(doc_text) = file_text {
        let content_hash = format!("{:x}", Sha256::digest(doc_text.as_bytes()));
        let file_name = file_name.unwrap_or_else(|| "upload".to_owned());
        let doc = state
            .session_service
            .create_document(user_id, CreateSessionDocumentDto {
                session_id,
                filename: file_name,
                content_hash,
                content: doc_text,
            })
            .await?;

        Some(doc.document_id)
    } else {
        None
    };

    let stream = state
        .chat_service
        .send_message(session_id, content, document_id)
        .await?;

    let sse_stream = stream.map(|dto| {
        let event_name = dto.event_name();
        let data = serde_json::to_string(&dto).unwrap_or_default();
        Ok::<_, Infallible>(Event::default().event(event_name).data(data))
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
