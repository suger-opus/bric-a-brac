use super::ExtendElement;
use crate::{
    domain::{GraphIdModel, NodeDataIdModel, NodeDataModel},
    infrastructure::DatabaseError,
};

impl TryFrom<neo4rs::Node> for NodeDataModel {
    type Error = DatabaseError;

    fn try_from(node: neo4rs::Node) -> Result<Self, Self::Error> {
        let node_data_id: NodeDataIdModel = node.extract_id("node_data_id")?.into();
        let graph_id: GraphIdModel = node.extract_id("graph_id")?.into();
        let key = node
            .labels()
            .first()
            .ok_or_else(|| DatabaseError::CorruptedNodeLabelState {
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
