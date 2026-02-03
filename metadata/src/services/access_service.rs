use crate::error::ApiError;
use crate::models::{
    access_model::{Access, Role},
    graph_model::GraphId,
    user_model::UserId,
};
use crate::repositories::access_repository::AccessRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AccessService {
    pool: PgPool,
    repository: AccessRepository,
}

impl AccessService {
    pub fn new(pool: &PgPool, repository: &AccessRepository) -> Self {
        AccessService {
            pool: pool.clone(),
            repository: repository.clone(),
        }
    }

    pub async fn post(
        &self,
        user_id: UserId,
        graph_id: GraphId,
        role: Role,
    ) -> Result<Access, ApiError> {
        let mut txn = self.pool.begin().await?;
        let access = self
            .repository
            .post(&mut txn, graph_id, user_id, role)
            .await?;
        txn.commit().await?;
        Ok(access)
    }
}
