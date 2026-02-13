use crate::models::{GraphId, NewPropertySchema, PropertySchema};
use bric_a_brac_id::id;
use chrono::{DateTime, Utc};

id!(NodeSchemaId);

#[derive(Debug)]
pub struct NodeSchema {
    pub node_schema_id: NodeSchemaId,
    pub graph_id: GraphId,
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub properties: Vec<PropertySchema>,
}

#[derive(Debug)]
pub struct NewNodeSchema {
    pub label: String,
    pub formatted_label: String,
    pub color: String,
    pub properties: Vec<NewPropertySchema>,
}
