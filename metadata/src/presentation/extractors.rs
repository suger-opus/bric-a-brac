use super::error::{AppError, DomainError};
use crate::domain::models::UserId;

use axum::{extract::FromRequestParts, http::request::Parts};

pub struct AuthenticatedUser {
    pub user_id: UserId,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id_str = parts
            .headers
            .get("user_id")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                AppError::Domain(DomainError::Unauthorized {
                    reason: "Missing user_id header".to_string(),
                })
            })?;

        let user_id = user_id_str.parse::<UserId>().map_err(|_| {
            AppError::Domain(DomainError::Unauthorized {
                reason: "Invalid user_id format - must be a valid UUID".to_string(),
            })
        })?;

        Ok(AuthenticatedUser { user_id })
    }
}
