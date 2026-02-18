use super::super::dtos::{AccessDto, CreateAccessDto};
use crate::{
    domain::models::GraphId,
    infrastructure::repositories::AccessRepository,
    presentation::errors::{AppError, DatabaseError},
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AccessService {
    pool: PgPool,
    repository: AccessRepository,
}

impl AccessService {
    pub fn new(pool: PgPool, repository: AccessRepository) -> Self {
        AccessService { pool, repository }
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_access_dto))]
    pub async fn create(
        &self,
        graph_id: GraphId,
        create_access_dto: CreateAccessDto,
    ) -> Result<AccessDto, AppError> {
        let mut txn = self.pool.begin().await.map_err(DatabaseError::from)?;
        let access = self
            .repository
            .create(&mut txn, create_access_dto.into_domain(graph_id))
            .await
            .map_err(DatabaseError::from)?;
        txn.commit().await.map_err(DatabaseError::from)?;

        Ok(access.into())
    }
}
