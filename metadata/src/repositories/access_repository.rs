use crate::error::ApiError;
use crate::models::{
    access_model::{Access, Role},
    graph_model::GraphId,
    user_model::UserId,
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct AccessRepository;

impl AccessRepository {
    pub fn new() -> Self {
        AccessRepository
    }

    pub async fn post(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphId,
        user_id: UserId,
        role: Role,
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
            graph_id as _,
            user_id as _,
            role as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(access)
    }
}
