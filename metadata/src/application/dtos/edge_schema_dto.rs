use super::CreatePropertySchemaIntoDomain;
use crate::{
    domain::{
        models::{CreateEdgeSchemaModel, EdgeSchemaIdModel, EdgeSchemaModel},
        utils::generate_key,
    },
};
use bric_a_brac_dtos::{CreateEdgeSchemaDto, EdgeSchemaDto, EdgeSchemaIdDto};

impl From<EdgeSchemaIdModel> for EdgeSchemaIdDto {
    fn from(graph_id: EdgeSchemaIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<EdgeSchemaIdDto> for EdgeSchemaIdModel {
    fn from(graph_id: EdgeSchemaIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<EdgeSchemaModel> for EdgeSchemaDto {
    fn from(model: EdgeSchemaModel) -> Self {
        Self {
            edge_schema_id: model.edge_schema_id.into(),
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

impl From<CreateEdgeSchemaDto> for CreateEdgeSchemaModel {
    fn from(dto: CreateEdgeSchemaDto) -> Self {
        let edge_schema_id = EdgeSchemaIdModel::new();

        Self {
            edge_schema_id,
            label: dto.label,
            key: generate_key(),
            color: dto.color,
            properties: dto
                .properties
                .into_iter()
                .map(|prop| prop.into_domain(None, Some(edge_schema_id)))
                .collect(),
        }
    }
}
