use crate::{
    domain::models::{
        EdgeDataModel,
        GraphIdModel, InsertEdgeDataModel, InsertNodeDataModel, NodeDataIdModel,
        NodeDataModel, UpdateNodeDataModel,
    },
    infrastructure::errors::DatabaseError,
};
use neo4rs::{query, BoltFloat, BoltList, BoltMap, BoltString, BoltType};
use std::{collections::HashMap, str::FromStr};

pub struct MutateRepository;

impl MutateRepository {
    pub fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.insert_node",
        skip(self, connection, graph_id, data)
    )]
    pub async fn insert_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: InsertNodeDataModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        let mut props: HashMap<BoltString, BoltType> = data.properties.try_into()?;
        props.insert("graph_id".into(), graph_id.to_string().into());
        props.insert("node_data_id".into(), data.node_data_id.to_string().into());
        if let Some(sid) = data.session_id {
            props.insert("session_id".into(), sid.into());
        }

        let emb_bolt = BoltType::List(BoltList {
            value: data
                .embedding
                .into_iter()
                .map(|f| BoltType::Float(BoltFloat::new(f as f64)))
                .collect(),
        });

        let cypher = format!(
            "CREATE (n:{}) SET n = $props, n.embedding = $emb RETURN n",
            data.key
        );
        let mut result = connection
            .execute(
                query(&cypher)
                    .param("props", BoltType::Map(BoltMap { value: props }))
                    .param("emb", emb_bolt),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoneRow())?;
        let neo_node: neo4rs::Node = row.get("n")?;
        NodeDataModel::try_from(neo_node)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.update_node",
        skip(self, connection, graph_id, data)
    )]
    pub async fn update_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: UpdateNodeDataModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        let props: HashMap<BoltString, BoltType> = data.properties.try_into()?;
        let emb_bolt = BoltType::List(BoltList {
            value: data
                .embedding
                .into_iter()
                .map(|f| BoltType::Float(BoltFloat::new(f as f64)))
                .collect(),
        });

        let mut result = connection
            .execute(
                query("MATCH (n { node_data_id: $nid, graph_id: $gid }) SET n += $props, n.embedding = $emb RETURN n")
                    .param("nid", data.node_data_id.to_string())
                    .param("gid", graph_id.to_string())
                    .param("props", BoltType::Map(BoltMap { value: props }))
                    .param("emb", emb_bolt),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoneRow())?;
        let neo_node: neo4rs::Node = row.get("n")?;
        NodeDataModel::try_from(neo_node)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.insert_edge",
        skip(self, connection, graph_id, data)
    )]
    pub async fn insert_edge(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: InsertEdgeDataModel,
    ) -> Result<EdgeDataModel, DatabaseError> {
        let mut edge_props: HashMap<BoltString, BoltType> = data.properties.try_into()?;
        edge_props.insert("edge_data_id".into(), data.edge_data_id.to_string().into());
        edge_props.insert("graph_id".into(), graph_id.to_string().into());
        if let Some(sid) = data.session_id {
            edge_props.insert("session_id".into(), sid.into());
        }

        let cypher = format!(
            r#"MATCH (a {{node_data_id: $from, graph_id: $gid}}), (b {{node_data_id: $to, graph_id: $gid}})
CREATE (a)-[r:{}]->(b)
SET r = $props
RETURN r, a.node_data_id AS from_node_data_id, b.node_data_id AS to_node_data_id"#,
            data.key
        );
        let mut result = connection
            .execute(
                query(&cypher)
                    .param("from", data.from_node_data_id.to_string())
                    .param("to", data.to_node_data_id.to_string())
                    .param("gid", graph_id.to_string())
                    .param("props", BoltType::Map(BoltMap { value: edge_props })),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoneRow())?;
        let neo_edge: neo4rs::Relation = row.get("r")?;
        let from_node_data_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
        let to_node_data_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
        let mut edge_data = EdgeDataModel::try_from(neo_edge)?;
        edge_data.from_node_data_id = from_node_data_id;
        edge_data.to_node_data_id = to_node_data_id;
        Ok(edge_data)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.initialize_schema",
        skip(self, graph, node_keys)
    )]
    pub async fn initialize_schema(
        &self,
        graph: &neo4rs::Graph,
        node_keys: Vec<String>,
    ) -> Result<(), DatabaseError> {
        for key in node_keys {
            let cypher = format!(
                "CREATE VECTOR INDEX ON :{}(embedding) OPTIONS {{dimension: 1536, capacity: 10000, metric: \"cos\"}}",
                key
            );
            match graph.run(query(&cypher)).await {
                Ok(_) => {
                    tracing::debug!(key = %key, "Vector index created");
                }
                Err(e) => {
                    tracing::warn!(key = %key, error = ?e, "Failed to create vector index (may already exist)");
                }
            }
        }
        Ok(())
    }
}
