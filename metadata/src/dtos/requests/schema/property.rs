use crate::{
    dtos::{PropertyMetadataDto, PropertyTypeDto},
    models::{EdgeSchemaId, NewPropertySchema, NodeSchemaId},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePropertySchemaRequest {
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub formatted_label: String,
    pub property_type: PropertyTypeDto,
    pub metadata: PropertyMetadataDto,
}

impl CreatePropertySchemaRequest {
    pub fn into_domain(self) -> NewPropertySchema {
        NewPropertySchema {
            node_schema_id: self.node_schema_id,
            edge_schema_id: self.edge_schema_id,
            label: self.label,
            formatted_label: self.formatted_label,
            property_type: self.property_type.into(),
            metadata: self.metadata.into(),
        }
    }
}
