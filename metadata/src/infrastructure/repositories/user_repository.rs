use crate::{
    domain::models::{CreateUser, User, UserId},
    presentation::errors::DatabaseError,
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn create(
        &self,
        connection: &mut PgConnection,
        create_user: CreateUser,
    ) -> Result<User, DatabaseError> {
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
            create_user.email,
            create_user.username
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn get(
        &self,
        connection: &mut PgConnection,
        user_id: UserId,
    ) -> Result<User, DatabaseError> {
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
