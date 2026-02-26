use bric_a_brac_id::id;
use chrono::{DateTime, Utc};

id!(UserIdModel);

#[derive(Debug, sqlx::FromRow)]
pub struct UserModel {
    pub user_id: UserIdModel,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct CreateUserModel {
    pub user_id: UserIdModel,
    pub username: String,
    pub email: String,
}
