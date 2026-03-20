use crate::{
    application::dtos::ChatRequestDto,
    presentation::{extractors::AuthenticatedUser, state::ApiState},
};
use axum::{
    extract::{Path, State},
    response::sse::{Event, KeepAlive, Sse},
    Json,
};
use bric_a_brac_dtos::GraphIdDto;
use futures_util::StreamExt;
use std::convert::Infallible;

#[tracing::instrument(
    level = "trace",
    name = "chat_handler.chat",
    skip(state, _user_id, _graph_id, payload)
)]
pub async fn chat(
    State(state): State<ApiState>,
    Path(_graph_id): Path<GraphIdDto>,
    AuthenticatedUser { user_id: _user_id }: AuthenticatedUser,
    Json(payload): Json<ChatRequestDto>,
) -> Result<
    Sse<impl futures_util::Stream<Item = Result<Event, Infallible>>>,
    crate::application::errors::AppError,
> {
    let stream = state
        .ai_service
        .chat(payload.session_id, payload.content)
        .await?;

    let sse_stream = stream.map(|dto| {
        let event_name = dto.event_name();
        let data = serde_json::to_string(&dto).unwrap_or_default();
        Ok::<_, Infallible>(Event::default().event(event_name).data(data))
    });

    Ok(Sse::new(sse_stream).keep_alive(KeepAlive::default()))
}
