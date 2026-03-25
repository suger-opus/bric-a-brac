use super::PropertiesDataModel;
use crate::{
    domain::{models::GraphIdModel, utils::ExtendElement},
    infrastructure::errors::DatabaseError,
};
use bric_a_brac_id::id;
use std::str::FromStr;

id!(NodeDataIdModel);

pub struct NodeDataModel {
    pub graph_id: GraphIdModel,
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
}

impl TryFrom<neo4rs::Node> for NodeDataModel {
    type Error = DatabaseError;

    fn try_from(node: neo4rs::Node) -> Result<Self, Self::Error> {
        let node_data_id = NodeDataIdModel::from_str(&node.extract_id("node_data_id")?)?;
        let graph_id = GraphIdModel::from_str(&node.extract_id("graph_id")?)?;
        let key = node
            .labels()
            .first()
            .ok_or_else(|| DatabaseError::UnlabeledNode {
                node_data_id: node_data_id.to_string(),
            })?
            .to_string();
        let properties = node.collect_properties()?;

        Ok(Self {
            graph_id,
            node_data_id,
            key,
            properties,
        })
    }
}

pub struct InsertNodeDataModel {
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
    pub embedding: Vec<f32>,
    pub session_id: Option<String>,
}

pub struct UpdateNodeDataModel {
    pub node_data_id: NodeDataIdModel,
    pub properties: PropertiesDataModel,
    pub embedding: Vec<f32>,
}

pub struct NodeSummaryModel {
    pub node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
    pub distance: f32,
}
