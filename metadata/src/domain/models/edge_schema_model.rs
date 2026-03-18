use super::GraphIdModel;
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(EdgeSchemaIdModel);

#[derive(Debug, Clone, Serialize)]
pub struct EdgeSchemaModel {
    pub edge_schema_id: EdgeSchemaIdModel,
    pub graph_id: GraphIdModel,
    pub label: String,
    pub key: String,
    pub color: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
