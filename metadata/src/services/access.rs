use crate::{
    error::ApiError,
    models::{Access, NewAccess},
    repositories::AccessRepository,
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

    pub async fn create_access(&self, new_access: NewAccess) -> Result<Access, ApiError> {
        let mut txn = self.pool.begin().await?;
        let access = self.repository.create_access(&mut txn, new_access).await?;
        txn.commit().await?;
        Ok(access)
    }
}
