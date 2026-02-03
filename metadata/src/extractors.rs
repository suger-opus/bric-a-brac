use crate::error::{ApiError, ApiErrorContent};
use crate::models::user_model::UserId;
use axum::{extract::FromRequestParts, http::request::Parts};

pub struct AuthenticatedUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                ApiError::Unauthorized(ApiErrorContent {
                    message: "Missing user_id header".to_string(),
                    details: "Authentication required".to_string(),
                })
            })?;

        let user_id = user_id_str.parse::<UserId>().map_err(|_| {
            ApiError::Unauthorized(ApiErrorContent {
                message: "Invalid user_id format".to_string(),
                details: "user_id must be a valid UUID".to_string(),
            })
        })?;

        Ok(AuthenticatedUser { user_id })
    }
}
