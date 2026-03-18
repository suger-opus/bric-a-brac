use crate::domain::models::{EdgeSchemaIdModel, EdgeSchemaModel};
use bric_a_brac_dtos::{EdgeSchemaDto, EdgeSchemaIdDto};

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
            label: model.label.into(),
            key: model.key.into(),
            color: model.color.into(),
            description: model.description,
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}
