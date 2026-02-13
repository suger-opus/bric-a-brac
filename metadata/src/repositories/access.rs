use crate::{
    error::ApiError,
    models::{Access, NewAccess},
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct AccessRepository;

impl AccessRepository {
    pub fn new() -> Self {
        AccessRepository
    }

    pub async fn create_access(
        &self,
        connection: &mut PgConnection,
        new_access: NewAccess,
    ) -> Result<Access, ApiError> {
        let access = sqlx::query_as!(
            Access,
            r#"
INSERT INTO accesses (graph_id, user_id, role)
VALUES ($1, $2, $3)
RETURNING
    graph_id,
    user_id,
    role AS "role!:_",
    created_at,
    updated_at
            "#,
            new_access.graph_id as _,
            new_access.user_id as _,
            new_access.role as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(access)
    }
}
