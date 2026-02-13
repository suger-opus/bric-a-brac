use crate::{
    dtos::PropertiesDto,
    models::{NewNodeData, NodeSchemaId},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateNodeDataRequest {
    pub node_schema_id: NodeSchemaId,
    pub properties: PropertiesDto,
}

impl CreateNodeDataRequest {
    pub fn into_domain(self) -> NewNodeData {
        NewNodeData {
            node_schema_id: self.node_schema_id,
            properties: self.properties.into(),
        }
    }
}
