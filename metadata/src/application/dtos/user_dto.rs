use crate::domain::models::{CreateUser, User, UserId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    #[schema(format = "email")]
    pub email: String,
    #[validate(length(min = 3, max = 50))]
    #[schema(min_length = 3, max_length = 50)]
    pub username: String,
}

impl CreateUserDto {
    pub fn into_domain(self) -> CreateUser {
        CreateUser {
            email: self.email,
            username: self.username,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    pub user_id: UserId,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        UserDto {
            user_id: user.user_id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
