use crate::models::{GraphId, NewPropertySchema, PropertySchema};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};

id!(EdgeSchemaId);

#[derive(Debug)]
pub struct EdgeSchema {
    pub edge_schema_id: EdgeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchema>,
}

#[derive(Debug)]
pub struct NewEdgeSchema {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<NewPropertySchema>,
}
