use crate::{
    error::ApiError,
    models::{NewUser, User, UserId},
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    pub async fn create(
        &self,
        connection: &mut PgConnection,
        new_user: NewUser,
    ) -> Result<User, ApiError> {
        let user = sqlx::query_as!(
            User,
            r#"
INSERT INTO users (user_id, email, username)
VALUES ($1, $2, $3)
RETURNING
    user_id,
    email,
    username,
    created_at,
    updated_at
            "#,
            UserId::new() as _,
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
SELECT
    user_id,
    email,
    username,
    created_at,
    updated_at
FROM users
WHERE user_id = $1
            "#,
            user_id as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }
}
