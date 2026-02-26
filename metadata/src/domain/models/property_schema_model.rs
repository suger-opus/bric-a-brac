use super::{EdgeSchemaIdModel, NodeSchemaIdModel};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(PropertySchemaIdModel);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMetadataModel {
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "property_type")]
pub enum PropertyTypeModel {
    Number,
    String,
    Boolean,
    Select,
}

#[derive(Debug, Clone, Serialize)]
pub struct PropertySchemaModel {
    pub property_schema_id: PropertySchemaIdModel,
    pub node_schema_id: Option<NodeSchemaIdModel>,
    pub edge_schema_id: Option<EdgeSchemaIdModel>,
    pub label: String,
    pub key: String,
    pub property_type: PropertyTypeModel,
    pub metadata: PropertyMetadataModel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePropertySchemaModel {
    pub property_schema_id: PropertySchemaIdModel,
    pub node_schema_id: Option<NodeSchemaIdModel>,
    pub edge_schema_id: Option<EdgeSchemaIdModel>,
    pub label: String,
    pub key: String,
    pub property_type: PropertyTypeModel,
    pub metadata: PropertyMetadataModel,
}
