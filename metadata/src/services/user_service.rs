use crate::dtos::user_dto::PostUser;
use crate::error::ApiError;
use crate::models::user_model::{User, UserId};
use crate::repositories::user_repository::UserRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
    repository: UserRepository,
}

impl UserService {
    pub fn new(pool: &PgPool, repository: &UserRepository) -> Self {
        UserService {
            pool: pool.clone(),
            repository: repository.clone(),
        }
    }

    pub async fn post(&self, new_user: &PostUser) -> Result<User, ApiError> {
        let mut txn = self.pool.begin().await?;
        let user = self.repository.post(&mut txn, new_user).await?;
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
