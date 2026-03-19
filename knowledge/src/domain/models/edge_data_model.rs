use super::PropertiesDataModel;
use crate::{
    domain::{models::NodeDataIdModel, utils::ExtendElement},
    infrastructure::errors::DatabaseError,
};
use bric_a_brac_id::id;
use std::str::FromStr;

id!(EdgeDataIdModel);

pub struct EdgeDataModel {
    pub edge_data_id: EdgeDataIdModel,
    pub key: String,
    pub from_node_data_id: NodeDataIdModel,
    pub to_node_data_id: NodeDataIdModel,
    pub properties: PropertiesDataModel,
}

impl TryFrom<neo4rs::Relation> for EdgeDataModel {
    type Error = DatabaseError;

    fn try_from(relation: neo4rs::Relation) -> Result<Self, Self::Error> {
        let edge_data_id = EdgeDataIdModel::from_str(&relation.extract_id("edge_data_id")?)?;
        let key = relation.typ().to_string();
        let properties = relation.collect_properties()?;

        Ok(EdgeDataModel {
            edge_data_id,
            key,
            from_node_data_id: NodeDataIdModel::default(), // Placeholder, will be set in the repository
            to_node_data_id: NodeDataIdModel::default(), // Placeholder, will be set in the repository
            properties,
        })
    }
}

impl TryFrom<neo4rs::UnboundedRelation> for EdgeDataModel {
    type Error = DatabaseError;

    fn try_from(relation: neo4rs::UnboundedRelation) -> Result<Self, Self::Error> {
        let edge_data_id = EdgeDataIdModel::from_str(&relation.extract_id("edge_data_id")?)?;
        let key = relation.typ().to_string();
        let properties = relation.collect_properties()?;

        Ok(EdgeDataModel {
            edge_data_id,
            key,
            from_node_data_id: NodeDataIdModel::default(), // Set from path order
            to_node_data_id: NodeDataIdModel::default(),   // Set from path order
            properties,
        })
    }
}

pub struct InsertEdgeDataModel {
    pub edge_data_id: EdgeDataIdModel,
    pub from_node_data_id: NodeDataIdModel,
    pub to_node_data_id: NodeDataIdModel,
    pub key: String,
    pub properties: PropertiesDataModel,
    pub session_id: Option<String>,
}

