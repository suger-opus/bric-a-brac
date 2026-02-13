use bric_a_brac_id::id;
use chrono::{DateTime, Utc};

id!(UserId);

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub user_id: UserId,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewUser {
    pub username: String,
    pub email: String,
}
