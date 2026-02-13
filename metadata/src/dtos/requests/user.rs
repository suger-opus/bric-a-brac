use serde::Deserialize;

use crate::models::NewUser;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
}

impl CreateUserRequest {
    pub fn into_domain(self) -> NewUser {
        NewUser {
            email: self.email,
            username: self.username,
        }
    }
}
