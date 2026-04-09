use crate::presentation::error::PresentationError;
use axum::{extract::FromRequestParts, http::request::Parts};
use bric_a_brac_dtos::UserIdDto;

pub struct AuthenticatedUser {
    pub user_id: UserIdDto,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = PresentationError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("Extracting authenticated user from request");

        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| PresentationError::MissingHeader {
                header: "user_id".to_owned(),
            })?;

        let user_id: UserIdDto = user_id_str
            .to_owned()
            .try_into()
            .map_err(PresentationError::from)?;

        tracing::debug!(user_id = ?user_id, "Successfully extracted authenticated user");
        Ok(Self { user_id })
    }
}
