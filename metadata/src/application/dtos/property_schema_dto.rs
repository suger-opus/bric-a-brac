use crate::domain::{
    models::{
        CreatePropertySchemaModel, EdgeSchemaIdModel, NodeSchemaIdModel, PropertyMetadataModel,
        PropertySchemaIdModel, PropertySchemaModel, PropertyTypeModel,
    },
    utils::generate_key,
};
use bric_a_brac_dtos::{
    CreatePropertySchemaDto, PropertyMetadataDto, PropertySchemaDto, PropertySchemaIdDto,
    PropertyTypeDto,
};

impl From<PropertySchemaIdModel> for PropertySchemaIdDto {
    fn from(graph_id: PropertySchemaIdModel) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<PropertySchemaIdDto> for PropertySchemaIdModel {
    fn from(graph_id: PropertySchemaIdDto) -> Self {
        Self::from(*graph_id.as_ref())
    }
}

impl From<PropertySchemaModel> for PropertySchemaDto {
    fn from(model: PropertySchemaModel) -> Self {
        Self {
            property_schema_id: model.property_schema_id.into(),
            node_schema_id: model.node_schema_id.map(From::from),
            edge_schema_id: model.edge_schema_id.map(From::from),
            label: model.label,
            key: model.key,
            property_type: model.property_type.into(),
            metadata: model.metadata.into(),
            created_at: model.created_at,
            updated_at: model.updated_at,
        }
    }
}

impl From<PropertyMetadataModel> for PropertyMetadataDto {
    fn from(value: PropertyMetadataModel) -> Self {
        Self {
            options: value
                .options
                .map(|options| options.into_iter().map(From::from).collect()),
        }
    }
}

impl From<PropertyMetadataDto> for PropertyMetadataModel {
    fn from(value: PropertyMetadataDto) -> Self {
        Self {
            options: value
                .options
                .map(|options| options.into_iter().map(From::from).collect()),
        }
    }
}

impl From<PropertyTypeModel> for PropertyTypeDto {
    fn from(property_type: PropertyTypeModel) -> Self {
        match property_type {
            PropertyTypeModel::String => Self::String,
            PropertyTypeModel::Number => Self::Number,
            PropertyTypeModel::Boolean => Self::Boolean,
            PropertyTypeModel::Select => Self::Select,
        }
    }
}

impl From<PropertyTypeDto> for PropertyTypeModel {
    fn from(property_type: PropertyTypeDto) -> Self {
        match property_type {
            PropertyTypeDto::String => Self::String,
            PropertyTypeDto::Number => Self::Number,
            PropertyTypeDto::Boolean => Self::Boolean,
            PropertyTypeDto::Select => Self::Select,
        }
    }
}

pub trait CreatePropertySchemaIntoDomain {
    fn into_domain(
        self,
        node_schema_id: Option<NodeSchemaIdModel>,
        edge_schema_id: Option<EdgeSchemaIdModel>,
    ) -> CreatePropertySchemaModel;
}

impl CreatePropertySchemaIntoDomain for CreatePropertySchemaDto {
    fn into_domain(
        self,
        node_schema_id: Option<NodeSchemaIdModel>,
        edge_schema_id: Option<EdgeSchemaIdModel>,
    ) -> CreatePropertySchemaModel {
        CreatePropertySchemaModel {
            property_schema_id: PropertySchemaIdModel::new(),
            node_schema_id,
            edge_schema_id,
            label: self.label,
            key: generate_key(),
            property_type: self.property_type.into(),
            metadata: self.metadata.into(),
        }
    }
}
