use crate::models::{edge_schema_model::EdgeSchemaId, node_schema_model::NodeSchemaId};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(PropertyId);

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PropertyMetadata {
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "property_type")]
pub enum PropertyType {
    Number,
    String,
    Boolean,
    Select,
}

// TODO: rename Property to PropertySchema
// Also rename tables edge_schemas and node_schemas to edges_schemas and node_schemas
#[derive(Debug, Serialize)]
pub struct Property {
    pub property_id: PropertyId,
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub formatted_label: String,
    pub property_type: PropertyType,
    pub metadata: PropertyMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
