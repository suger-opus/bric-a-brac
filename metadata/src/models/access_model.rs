use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::Type)]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}
