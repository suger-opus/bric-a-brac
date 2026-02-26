use super::{CreatePropertySchemaModel, GraphIdModel, PropertySchemaModel};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(NodeSchemaIdModel);

#[derive(Debug, Clone, Serialize)]
pub struct NodeSchemaModel {
    pub node_schema_id: NodeSchemaIdModel,
    pub graph_id: GraphIdModel,
    pub label: String,
    pub key: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchemaModel>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateNodeSchemaModel {
    pub node_schema_id: NodeSchemaIdModel,
    pub label: String,
    pub key: String,
    pub color: String,
    pub properties: Vec<CreatePropertySchemaModel>,
}
