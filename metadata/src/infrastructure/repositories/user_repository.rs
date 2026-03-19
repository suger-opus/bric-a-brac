use crate::{
    domain::models::{CreateUserModel, UserIdModel, UserModel},
    infrastructure::errors::DatabaseError,
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        UserRepository
    }

    #[tracing::instrument(
        level = "debug",
        name = "user_repository.create",
        skip(self, connection, create_user)
    )]
    pub async fn create(
        &self,
        connection: &mut PgConnection,
        create_user: CreateUserModel,
    ) -> Result<UserModel, DatabaseError> {
        tracing::debug!(user_id = ?create_user.user_id);

        let user = sqlx::query_as!(
            UserModel,
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
            create_user.user_id as _,
            create_user.email,
            create_user.username
        )
        .fetch_one(connection)
        .await?;

        Ok(user)
    }

    #[tracing::instrument(
        level = "debug",
        name = "user_repository.get",
        skip(self, connection, user_id)
    )]
    pub async fn get(
        &self,
        connection: &mut PgConnection,
        user_id: UserIdModel,
    ) -> Result<UserModel, DatabaseError> {
        tracing::debug!(user_id = ?user_id);

        let user = sqlx::query_as!(
            UserModel,
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
