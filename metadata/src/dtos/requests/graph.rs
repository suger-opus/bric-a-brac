use crate::models::NewGraph;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateGraphRequest {
    pub name: String,
    pub description: String,
    pub is_public: bool,
}

impl CreateGraphRequest {
    pub fn into_domain(self) -> NewGraph {
        NewGraph {
            name: self.name,
            description: self.description,
            is_public: self.is_public,
        }
    }
}
