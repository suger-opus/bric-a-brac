use crate::{
    error::ApiError,
    models::{NewUser, User, UserId},
    repositories::UserRepository,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
    repository: UserRepository,
}

impl UserService {
    pub fn new(pool: PgPool, repository: UserRepository) -> Self {
        UserService { pool, repository }
    }

    pub async fn create(&self, new_user: NewUser) -> Result<User, ApiError> {
        let mut txn = self.pool.begin().await?;
        let user = self.repository.create(&mut txn, new_user).await?;
        txn.commit().await?;
        Ok(user)
    }

    pub async fn get(&self, user_id: UserId) -> Result<User, ApiError> {
        let mut txn = self.pool.begin().await?;
        let user = self.repository.get(&mut txn, user_id).await?;
        txn.commit().await?;
        Ok(user)
    }
}
