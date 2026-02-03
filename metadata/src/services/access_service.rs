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
}
