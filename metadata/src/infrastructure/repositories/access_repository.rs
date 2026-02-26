use crate::{
    domain::models::{AccessModel, CreateAccessModel},
    presentation::errors::DatabaseError,
};
use sqlx::PgConnection;

#[derive(Clone)]
pub struct AccessRepository;

impl AccessRepository {
    pub fn new() -> Self {
        AccessRepository
    }

    #[tracing::instrument(level = "debug", skip(self, connection))]
    pub async fn create(
        &self,
        connection: &mut PgConnection,
        create_access: CreateAccessModel,
    ) -> Result<AccessModel, DatabaseError> {
        let access = sqlx::query_as!(
            AccessModel,
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
            create_access.graph_id as _,
            create_access.user_id as _,
            create_access.role as _,
        )
        .fetch_one(connection)
        .await?;

        Ok(access)
    }
}
