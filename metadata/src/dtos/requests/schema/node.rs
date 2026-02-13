use crate::{dtos::CreatePropertySchemaRequest, models::NewNodeSchema};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateNodeSchemaRequest {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<CreatePropertySchemaRequest>,
}

impl CreateNodeSchemaRequest {
    pub fn into_domain(self) -> NewNodeSchema {
        NewNodeSchema {
            label: self.label,
            formatted_label: self.formatted_label,
            color: self.color,
            properties: self
                .properties
                .into_iter()
                .map(|prop| prop.into_domain())
                .collect(),
        }
    }
}
