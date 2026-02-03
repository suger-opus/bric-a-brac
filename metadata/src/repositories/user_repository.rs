use crate::dtos::user_dto::PostUser;
use crate::error::{ApiError, NotFoundContext, WithErrorContext};
use crate::models::user_model::{User, UserId};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    pub async fn post(
        &self,
        connection: &mut PgConnection,
        new_user: PostUser,
    ) -> Result<User, ApiError> {
        let user_id = UserId::new();
        let user = sqlx::query_as!(
            User,
            r#"
INSERT INTO users (user_id, email, username)
VALUES ($1, $2, $3)
RETURNING user_id AS "user_id!:_", email, username, created_at, updated_at
            "#,
            user_id as _,
            new_user.email,
            new_user.username
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    pub async fn get(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
    ) -> Result<User, ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
SELECT user_id AS "user_id!:_", email, username, created_at, updated_at
FROM users
WHERE user_id = $1
            "#,
            user_id as _,
        )
        .fetch_one(connection)
        .await
        .with_not_found_context(NotFoundContext {
            table: "users".to_string(),
            record_id: user_id.to_string(),
        })?;

        Ok(user)
    }
}
