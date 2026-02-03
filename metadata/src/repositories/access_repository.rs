use crate::error::ApiError;
use crate::models::{access_model::Role, graph_model::GraphId, user_model::UserId};
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
    ) -> Result<(), ApiError> {
        sqlx::query!(
            r#"
INSERT INTO accesses (graph_id, user_id, role)
VALUES ($1, $2, $3)
            "#,
            graph_id as _,
            user_id as _,
            role as _,
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
