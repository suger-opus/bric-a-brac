use crate::{
    domain::models::{
        CreateEdgeDataModel, CreateNodeDataModel, EdgeDataModel, GraphIdModel, NodeDataModel,
    },
    presentation::errors::DatabaseError,
};
use neo4rs::{query, BoltString, BoltType};
use std::collections::HashMap;

pub struct MutateRepository;

impl MutateRepository {
    pub fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.insert_node",
        skip(self, connection, graph_id, create_node_data)
    )]
    pub async fn insert_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        create_node_data: CreateNodeDataModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        tracing::debug!(graph_id = ?graph_id, node_data_id = ?create_node_data.node_data_id);

        let mut properties: HashMap<BoltString, BoltType> =
            create_node_data.properties.try_into()?;
        properties.insert("graph_id".to_string().into(), graph_id.to_string().into());
        properties.insert(
            "node_data_id".to_string().into(),
            create_node_data.node_data_id.to_string().into(),
        );
        let prop_keys: Vec<String> = properties
            .keys()
            .enumerate()
            .map(|(i, key)| format!("{}: $p{}", key, i))
            .collect();

        let cypher = format!(
            r#"
CREATE (n:{} {{ {} }})
RETURN n
        "#,
            create_node_data.key,
            prop_keys.join(", ")
        );
        let q = properties
            .iter()
            .enumerate()
            .fold(query(&cypher), |q, (i, (_key, value))| {
                q.param(&format!("p{}", i), value.clone())
            });
        let mut result = connection.execute(q).await?;
        let row = result
            .next(connection)
            .await?
            .ok_or_else(|| DatabaseError::NoneRow())?;
        let neo_node: neo4rs::Node = row.get("n")?;

        Ok(NodeDataModel::try_from(neo_node)?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.insert_edge",
        skip(self, connection, _graph_id, create_edge_data)
    )]
    pub async fn insert_edge(
        &self,
        connection: &mut neo4rs::Txn,
        _graph_id: GraphIdModel,
        create_edge_data: CreateEdgeDataModel,
    ) -> Result<EdgeDataModel, DatabaseError> {
        tracing::debug!(graph_id = ?_graph_id, edge_data_id = ?create_edge_data.edge_data_id);

        let mut properties: HashMap<BoltString, BoltType> =
            create_edge_data.properties.try_into()?;
        properties.insert(
            "edge_data_id".to_string().into(),
            create_edge_data.edge_data_id.to_string().into(),
        );
        let prop_keys: Vec<String> = properties
            .keys()
            .enumerate()
            .map(|(i, key)| format!("{}: $p{}", key, i))
            .collect();
        let edge_props = format!(" {{ {} }}", prop_keys.join(", "));
        let cypher = format!(
            r#"
MATCH
    (a {{ node_data_id: $from_node_data_id }}),
    (b {{ node_data_id: $to_node_data_id }})
CREATE (a)-[e:{}{}]->(b)
RETURN
    e,
    a.node_data_id AS from_node_data_id,
    b.node_data_id AS to_node_data_id
        "#,
            create_edge_data.key, edge_props
        );

        let q = properties.iter().enumerate().fold(
            query(&cypher)
                .param(
                    "from_node_data_id",
                    create_edge_data.from_node_data_id.to_string(),
                )
                .param(
                    "to_node_data_id",
                    create_edge_data.to_node_data_id.to_string(),
                ),
            |q, (i, (_key, value))| q.param(&format!("p{}", i), value.clone()),
        );
        let mut result = connection.execute(q).await?;
        let row = result
            .next(connection)
            .await?
            .ok_or_else(|| DatabaseError::NoneRow())?;
        let neo_edge: neo4rs::Relation = row.get("e")?;
        let from_node_data_id = row.get("from_node_data_id")?;
        let to_node_data_id = row.get("to_node_data_id")?;
        let mut edge_data = EdgeDataModel::try_from(neo_edge)?;
        edge_data.from_node_data_id = from_node_data_id;
        edge_data.to_node_data_id = to_node_data_id;

        Ok(edge_data)
    }
}
