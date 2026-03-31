use crate::{
    application::{AccessDto, AppError, CreateAccessDto},
    domain::CreateAccessModel,
    infrastructure::AccessRepository,
};
use bric_a_brac_dtos::GraphIdDto;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AccessService {
    pool: PgPool,
    repository: AccessRepository,
}

impl AccessService {
    pub const fn new(pool: PgPool, repository: AccessRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "access_service.create",
        skip(self, graph_id, create_access_dto),
        err
    )]
    pub async fn create(
        &self,
        graph_id: GraphIdDto,
        create_access_dto: CreateAccessDto,
    ) -> Result<AccessDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let access = self
            .repository
            .create(
                &mut txn,
                CreateAccessModel {
                    graph_id: graph_id.into(),
                    user_id: create_access_dto.user_id.into(),
                    role: create_access_dto.role.into(),
                },
            )
            .await?;
        txn.commit().await?;

        Ok(access.into())
    }
}
