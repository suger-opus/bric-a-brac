use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PostUser {
    pub email: String,
    pub username: String,
}
