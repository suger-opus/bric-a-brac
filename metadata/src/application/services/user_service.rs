use super::super::dtos::{CreateUserDto, UserDto};
use crate::{
    domain::models::UserId,
    infrastructure::repositories::UserRepository,
    presentation::error::{AppError, ResultExt},
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

    pub async fn create(&self, create_user_dto: CreateUserDto) -> Result<UserDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for create_user")?;
        let user = self
            .repository
            .create(&mut txn, create_user_dto.into_domain())
            .await
            .context("Failed to create user in repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after creating user")?;

        Ok(user.into())
    }

    pub async fn get(&self, user_id: UserId) -> Result<UserDto, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for get_user")?;
        let user = self
            .repository
            .get(&mut txn, user_id)
            .await
            .context("Failed to fetch user from repository")?;
        txn.commit()
            .await
            .context("Failed to commit transaction after fetching user")?;

        Ok(user.into())
    }
}
