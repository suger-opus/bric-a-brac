use super::CreatePropertySchemaIntoDomain;
use crate::domain::{
    models::{CreateNodeSchemaModel, NodeSchemaIdModel, NodeSchemaModel},
    utils::generate_key,
};
use bric_a_brac_dtos::{CreateNodeSchemaDto, NodeSchemaDto, NodeSchemaIdDto};

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
            label: model.label,
            key: model.key,
            color: model.color,
            created_at: model.created_at,
            updated_at: model.updated_at,
            properties: model.properties.into_iter().map(From::from).collect(),
        }
    }
}

impl From<CreateNodeSchemaDto> for CreateNodeSchemaModel {
    fn from(dto: CreateNodeSchemaDto) -> Self {
        let node_schema_id = NodeSchemaIdModel::new();

        Self {
            node_schema_id,
            label: dto.label,
            key: generate_key(),
            color: dto.color,
            properties: dto
                .properties
                .into_iter()
                .map(|prop| prop.into_domain(Some(node_schema_id), None))
                .collect(),
        }
    }
}
