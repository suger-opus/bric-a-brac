use crate::domain::{CreateUserModel, UserIdModel, UserModel};
use bric_a_brac_dtos::UserIdDto;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

impl From<UserIdModel> for UserIdDto {
    fn from(user_id: UserIdModel) -> Self {
        Self::from(*user_id.as_ref())
    }
}

impl From<UserIdDto> for UserIdModel {
    fn from(user_id: UserIdDto) -> Self {
        Self::from(*user_id.as_ref())
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserDto {
    pub user_id: UserIdDto,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<UserModel> for UserDto {
    fn from(user: UserModel) -> Self {
        Self {
            user_id: user.user_id.into(),
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct CreateUserDto {
    #[validate(email)]
    #[schema(format = "email")]
    pub email: String,

    #[validate(length(min = 3, max = 50))]
    #[schema(min_length = 3, max_length = 50)]
    pub username: String,
}

impl From<CreateUserDto> for CreateUserModel {
    fn from(dto: CreateUserDto) -> Self {
        Self {
            user_id: UserIdModel::new(),
            email: dto.email,
            username: dto.username,
        }
    }
}
