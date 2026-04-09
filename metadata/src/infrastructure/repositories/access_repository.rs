use crate::{
    domain::{AccessModel, CreateAccessModel, GraphIdModel, RoleModel, UserIdModel},
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
        name = "access_repository.get_role",
        skip(self, connection, graph_id, user_id),
        err
    )]
    pub async fn get_role(
        &self,
        connection: &mut PgConnection,
        graph_id: GraphIdModel,
        user_id: UserIdModel,
    ) -> Result<RoleModel, InfraError> {
        tracing::debug!(graph_id = ?graph_id, user_id = ?user_id);

        let row: Option<RoleModel> = sqlx::query_scalar!(
            r#"
SELECT COALESCE(role, 'none'::role_type) AS "role!:RoleModel"
FROM accesses
WHERE graph_id = $1 AND user_id = $2
            "#,
            graph_id as _,
            user_id as _,
        )
        .fetch_optional(connection)
        .await?;

        Ok(row.unwrap_or(RoleModel::None))
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
