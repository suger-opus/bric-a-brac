use super::ExtendElement;
use crate::{
    domain::{EdgeDataIdModel, EdgeDataModel, NodeDataIdModel},
    infrastructure::DatabaseError,
};

impl EdgeDataModel {
    pub fn try_from_relation(
        relation: &neo4rs::Relation,
        from_node_data_id: NodeDataIdModel,
        to_node_data_id: NodeDataIdModel,
    ) -> Result<Self, DatabaseError> {
        let edge_data_id: EdgeDataIdModel = relation.extract_id("edge_data_id")?.into();
        let key = relation.typ().to_owned();
        let properties = relation.collect_properties()?;

        Ok(Self {
            edge_data_id,
            key,
            from_node_data_id,
            to_node_data_id,
            properties,
        })
    }

    pub fn try_from_unbounded_relation(
        relation: &neo4rs::UnboundedRelation,
        from_node_data_id: NodeDataIdModel,
        to_node_data_id: NodeDataIdModel,
    ) -> Result<Self, DatabaseError> {
        let edge_data_id: EdgeDataIdModel = relation.extract_id("edge_data_id")?.into();
        let key = relation.typ().to_owned();
        let properties = relation.collect_properties()?;

        Ok(Self {
            edge_data_id,
            key,
            from_node_data_id,
            to_node_data_id,
            properties,
        })
    }
}
