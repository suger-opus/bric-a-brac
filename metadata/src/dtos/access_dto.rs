use serde::Deserialize;

use crate::models::{access_model::Role, user_model::UserId};

#[derive(Debug, Deserialize)]
pub struct PostAccess {
    pub user_id: UserId,
    pub role: Role,
}
