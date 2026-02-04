use bric_a_brac_id::id;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

id!(GraphId);

#[derive(Debug, Serialize, Deserialize)]
pub struct Reddit {}

#[derive(Debug, Serialize)]
pub struct Graph {
    pub graph_id: GraphId,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub reddit: Reddit,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub nb_data_nodes: u32,
    pub nb_data_edges: u32,
}
