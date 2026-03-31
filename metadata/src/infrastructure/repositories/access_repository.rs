use crate::{
    domain::{AccessModel, CreateAccessModel},
    infrastructure::InfraError,
};
use sqlx::PgConnection;

#[derive(Clone, Default)]
pub struct AccessRepository;

impl AccessRepository {
    pub const fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "access_repository.create",
        skip(self, connection, create_access),
        err
    )]
    pub async fn create(
        &self,
        connection: &mut PgConnection,
        create_access: CreateAccessModel,
    ) -> Result<AccessModel, InfraError> {
        tracing::debug!(graph_id = ?create_access.graph_id, user_id = ?create_access.user_id, role = ?create_access.role);

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
