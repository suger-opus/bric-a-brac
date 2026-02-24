use super::{EdgeSchemaId, NodeSchemaId};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(PropertySchemaId);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMetadata {
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize, sqlx::Type)]
#[sqlx(type_name = "property_type")]
pub enum PropertyType {
    Number,
    String,
    Boolean,
    Select,
}

impl TryFrom<&str> for PropertyType {
    type Error = String;

    fn try_from(type_str: &str) -> Result<Self, Self::Error> {
        match type_str {
            "Number" => Ok(PropertyType::Number),
            "String" => Ok(PropertyType::String),
            "Boolean" => Ok(PropertyType::Boolean),
            "Select" => Ok(PropertyType::Select),
            _ => Err(format!("Unsupported property type: {}", type_str)),
        }
    }
}

#[derive(Debug)]
pub struct PropertySchema {
    pub property_schema_id: PropertySchemaId,
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub key: String,
    pub property_type: PropertyType,
    pub metadata: PropertyMetadata,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreatePropertySchema {
    pub node_schema_id: Option<NodeSchemaId>,
    pub edge_schema_id: Option<EdgeSchemaId>,
    pub label: String,
    pub key: String,
    pub property_type: PropertyType,
    pub metadata: PropertyMetadata,
}
