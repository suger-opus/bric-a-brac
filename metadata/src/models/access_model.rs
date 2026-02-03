use serde::Serialize;

#[derive(Debug, Clone, Serialize, sqlx::Type)]
#[sqlx(type_name = "role_type")]
pub enum Role {
    Owner,
    Admin,
    Editor,
    Viewer,
    None,
}
