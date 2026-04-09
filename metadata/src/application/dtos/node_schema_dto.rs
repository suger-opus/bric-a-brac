use crate::domain::{NodeSchemaIdModel, NodeSchemaModel};
use bric_a_brac_dtos::{NodeSchemaDto, NodeSchemaIdDto};

impl From<NodeSchemaIdModel> for NodeSchemaIdDto {
    fn from(graph_id: NodeSchemaIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<NodeSchemaIdDto> for NodeSchemaIdModel {
    fn from(graph_id: NodeSchemaIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<NodeSchemaModel> for NodeSchemaDto {
    fn from(model: NodeSchemaModel) -> Self {
        Self {
            node_schema_id: model.node_schema_id.into(),
            graph_id: model.graph_id.into(),
            label: model.label.into(),
            key: model.key.into(),
            color: model.color.into(),
            description: model.description.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
