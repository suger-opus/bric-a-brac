use super::super::dtos::{AccessDto, CreateAccessDto};
use crate::{
    domain::models::GraphId,
    infrastructure::repositories::AccessRepository,
    presentation::error::{AppError, ResultExt},
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

    pub async fn create_access(
        &self,
        graph_id: GraphId,
        create_access_dto: CreateAccessDto,
    ) -> Result<AccessDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for access creation")?;
        let access = self
            .repository
            .create_access(&mut txn, create_access_dto.into_domain(graph_id))
            .await
            .context("Failed to create access in repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after creating access")?;

        Ok(access.into())
    }
}
