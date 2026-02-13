use crate::{
    dtos::PropertiesDto,
    models::{NewNodeData, NodeSchemaId},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateNodeDataRequest {
    pub node_schema_id: NodeSchemaId,
    pub formatted_label: String,
    pub properties: PropertiesDto,
}

impl CreateNodeDataRequest {
    pub fn into_domain(self) -> NewNodeData {
        NewNodeData {
            node_schema_id: self.node_schema_id,
            formatted_label: self.formatted_label,
            properties: self.properties.into(),
        }
    }
}
