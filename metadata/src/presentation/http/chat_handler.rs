use crate::{
    application::{
        dtos::CreateSessionDocumentDto,
        errors::RequestError,
        services::file_extraction,
    },
    presentation::{extractors::AuthenticatedUser, state::ApiState},
};
use axum::{
    extract::{Multipart, Path, State},
    response::sse::{Event, KeepAlive, Sse},
};
use bric_a_brac_dtos::GraphIdDto;
use futures_util::StreamExt;
use sha2::{Digest, Sha256};
use std::convert::Infallible;

#[tracing::instrument(
    level = "trace",
    name = "chat_handler.chat",
    skip(state, _user_id, _graph_id, multipart)
)]
pub async fn chat(
    State(state): State<ApiState>,
    Path(_graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
    mut multipart: Multipart,
) -> Result<
    Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>>,
    crate::application::errors::AppError,
> {
    let mut session_id: Option<String> = None;
    let mut content: Option<String> = None;
    let mut file_text: Option<String> = None;
    let mut file_name: Option<String> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| RequestError::InvalidFile {
            issue: format!("Failed to read multipart field: {e}"),
        })?
    {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "session_id" => {
                session_id = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| RequestError::InvalidInput {
                            field: "session_id".to_string(),
                            issue: format!("Failed to read session_id: {e}"),
                        })?,
                );
            }
            "content" => {
                content = Some(
                    field
                        .text()
                        .await
                        .map_err(|e| RequestError::InvalidInput {
                            field: "content".to_string(),
                            issue: format!("Failed to read content: {e}"),
                        })?,
                );
            }
            "file" => {
                file_name = field.file_name().map(ToString::to_string);
                let content_type = field.content_type().map(ToString::to_string);
                let bytes =
                    field
                        .bytes()
                        .await
                        .map_err(|e| RequestError::InvalidFile {
                            issue: format!("Failed to read file bytes: {e}"),
                        })?;

                let extracted = file_extraction::extract_text(
                    &bytes,
                    content_type.as_deref(),
                    file_name.as_deref(),
                )?;
                file_text = Some(extracted);
            }
            _ => {}
        }
    }

    let session_id = session_id.ok_or(RequestError::InvalidInput {
        field: "session_id".to_string(),
        issue: "session_id is required".to_string(),
    })?;

    let user_content = content.filter(|s| !s.is_empty());

    // If a file was uploaded, save it as a session document and pass its ID.
    // The AI service will load the content on its own.
    let document_id = if let Some(ref doc_text) = file_text {
        let content_hash = format!("{:x}", Sha256::digest(doc_text.as_bytes()));
        let filename = file_name.unwrap_or_else(|| "upload".to_string());

        let doc = state
            .document_service
            .create_document(CreateSessionDocumentDto {
                session_id: session_id.clone(),
                filename,
                content_hash,
                content: doc_text.clone(),
            })
            .await?;

        Some(doc.document_id.to_string())
    } else {
        None
    };

    let final_content = user_content.unwrap_or_default();

    if final_content.is_empty() && document_id.is_none() {
        return Err(RequestError::InvalidInput {
            field: "content".to_string(),
            issue: "At least one of 'content' or 'file' must be provided".to_string(),
        }
        .into());
    }

    let stream = state
        .ai_service
        .chat(session_id, final_content, document_id)
        .await?;

    let sse_stream = stream.map(|dto| {
        let event_name = dto.event_name();
        let data = serde_json::to_string(&dto).unwrap_or_default();
        Ok::<_, Infallible>(Event::default().event(event_name).data(data))
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
