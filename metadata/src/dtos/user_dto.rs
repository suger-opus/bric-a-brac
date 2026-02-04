use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostUser {
    pub email: String,
    pub username: String,
}
