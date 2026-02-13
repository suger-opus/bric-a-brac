use crate::{dtos::CreatePropertySchemaRequest, models::NewEdgeSchema};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateEdgeSchemaRequest {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<CreatePropertySchemaRequest>,
}

impl CreateEdgeSchemaRequest {
    pub fn into_domain(self) -> NewEdgeSchema {
        NewEdgeSchema {
            label: self.label,
            formatted_label: self.formatted_label,
            color: self.color,
            properties: self
                .properties
                .into_iter()
                .map(CreatePropertySchemaRequest::into_domain)
                .collect(),
        }
    }
}
