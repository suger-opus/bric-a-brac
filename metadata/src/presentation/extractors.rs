use super::errors::RequestError;
use crate::application::dtos::UserIdDto;
use axum::http::request::Parts;
use axum::extract::FromRequestParts;

pub struct AuthenticatedUser {
    pub user_id: UserIdDto,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = RequestError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        tracing::debug!("Extracting authenticated user from request");

        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| RequestError::Unauthorized {
                reason: "Missing user_id header".to_string(),
            })?;

        let user_id = user_id_str
            .parse::<UserIdDto>()
            .map_err(|_| RequestError::Unauthorized {
                reason: "Invalid user_id format - must be a valid UUID".to_string(),
            })?;

        tracing::debug!(user_id = ?user_id, "Successfully extracted authenticated user");
        Ok(AuthenticatedUser { user_id })
    }
}
