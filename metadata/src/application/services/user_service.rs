use crate::{
    application::dtos::{CreateUserDto, UserDto, UserIdDto},
    infrastructure::repositories::UserRepository,
    application::errors::AppError,
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

    #[tracing::instrument(level = "trace", name = "user_service.create", skip(self, create_user))]
    pub async fn create(&self, create_user: CreateUserDto) -> Result<UserDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let user = self.repository.create(&mut txn, create_user.into()).await?;
        txn.commit().await?;

        Ok(user.into())
    }

    #[tracing::instrument(level = "trace", name = "user_service.get", skip(self, user_id))]
    pub async fn get(&self, user_id: UserIdDto) -> Result<UserDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let user = self.repository.get(&mut txn, user_id.into()).await?;
        txn.commit().await?;

        Ok(user.into())
    }
}
