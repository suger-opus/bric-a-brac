use crate::models::graph_model::GraphId;
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::Serialize;

id!(NodeSchemaId);

#[derive(Debug, Serialize)]
pub struct NodeSchema {
    pub node_schema_id: NodeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
