use crate::{
    domain::{
        CreateEdgeDataModel, CreateNodeDataModel, EdgeDataIdModel, EdgeDataModel, GraphIdModel,
        NodeDataIdModel, NodeDataModel, UpdateEdgeDataModel, UpdateNodeDataModel,
    },
    infrastructure::DatabaseError,
};
use neo4rs::{query, BoltFloat, BoltList, BoltMap, BoltString, BoltType};
use std::{collections::HashMap, str::FromStr};

#[derive(Default)]
pub struct MutateRepository;

impl MutateRepository {
    pub const fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.create_node",
        skip(self, connection, graph_id, data),
        err
    )]
    pub async fn create_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: CreateNodeDataModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        let mut props: HashMap<BoltString, BoltType> = data.properties.into();
        props.insert("graph_id".into(), graph_id.to_string().into());
        props.insert("node_data_id".into(), data.node_data_id.to_string().into());

        let emb_bolt = BoltType::List(BoltList {
            value: data
                .embedding
                .into_iter()
                .map(|f| BoltType::Float(BoltFloat::new(f64::from(f))))
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
            .ok_or(DatabaseError::NoRows())?;
        let neo_node: neo4rs::Node = row.get("n")?;
        NodeDataModel::try_from(neo_node)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.update_node",
        skip(self, connection, graph_id, data),
        err
    )]
    pub async fn update_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: UpdateNodeDataModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        let props: HashMap<BoltString, BoltType> = data.properties.into();
        let emb_bolt = BoltType::List(BoltList {
            value: data
                .embedding
                .into_iter()
                .map(|f| BoltType::Float(BoltFloat::new(f64::from(f))))
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
            .ok_or(DatabaseError::NoRows())?;
        let neo_node: neo4rs::Node = row.get("n")?;
        NodeDataModel::try_from(neo_node)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.create_edge",
        skip(self, connection, graph_id, data),
        err
    )]
    pub async fn create_edge(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: CreateEdgeDataModel,
    ) -> Result<EdgeDataModel, DatabaseError> {
        let mut edge_props: HashMap<BoltString, BoltType> = data.properties.into();
        edge_props.insert("edge_data_id".into(), data.edge_data_id.to_string().into());
        edge_props.insert("graph_id".into(), graph_id.to_string().into());

        // MERGE ensures edge uniqueness per (from, to, edge_key) triple.
        // ON CREATE sets properties for new edges, ON MATCH updates for existing ones.
        let cypher = format!(
            r"MATCH (a {{node_data_id: $from, graph_id: $gid}}), (b {{node_data_id: $to, graph_id: $gid}})
MERGE (a)-[r:{}]->(b)
ON CREATE SET r = $props
ON MATCH SET r += $props
RETURN r, a.node_data_id AS from_node_data_id, b.node_data_id AS to_node_data_id",
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
            .ok_or(DatabaseError::NoRows())?;
        let neo_edge: neo4rs::Relation = row.get("r")?;
        let from_node_data_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
        let to_node_data_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
        EdgeDataModel::try_from_relation(&neo_edge, from_node_data_id, to_node_data_id)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.update_edge",
        skip(self, connection, graph_id, data),
        err
    )]
    pub async fn update_edge(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        data: UpdateEdgeDataModel,
    ) -> Result<EdgeDataModel, DatabaseError> {
        let props: HashMap<BoltString, BoltType> = data.properties.into();

        let mut result = connection
            .execute(
                query(
                    "MATCH ({ graph_id: $gid })-[r { edge_data_id: $eid }]->({ graph_id: $gid }) \
                     SET r += $props \
                     RETURN r, startNode(r).node_data_id AS from_node_data_id, endNode(r).node_data_id AS to_node_data_id"
                )
                    .param("eid", data.edge_data_id.to_string())
                    .param("gid", graph_id.to_string())
                    .param("props", BoltType::Map(BoltMap { value: props })),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoRows())?;
        let neo_edge: neo4rs::Relation = row.get("r")?;
        let from_node_data_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
        let to_node_data_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
        EdgeDataModel::try_from_relation(&neo_edge, from_node_data_id, to_node_data_id)
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.delete_node",
        skip(self, connection, graph_id, node_data_id),
        err
    )]
    pub async fn delete_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        node_data_id: NodeDataIdModel,
    ) -> Result<(), DatabaseError> {
        let mut result = connection
            .execute(
                query("MATCH (n { node_data_id: $nid, graph_id: $gid }) DETACH DELETE n RETURN count(n) AS deleted")
                    .param("nid", node_data_id.to_string())
                    .param("gid", graph_id.to_string()),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoRows())?;
        let deleted: i64 = row.get("deleted")?;
        if deleted == 0 {
            return Err(DatabaseError::NoRows());
        }
        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.delete_edge",
        skip(self, connection, graph_id, edge_data_id),
        err
    )]
    pub async fn delete_edge(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        edge_data_id: EdgeDataIdModel,
    ) -> Result<(), DatabaseError> {
        let mut result = connection
            .execute(
                query(
                    "MATCH ({ graph_id: $gid })-[r { edge_data_id: $eid }]->({ graph_id: $gid }) DELETE r RETURN count(r) AS deleted"
                )
                    .param("eid", edge_data_id.to_string())
                    .param("gid", graph_id.to_string()),
            )
            .await?;

        let row = result
            .next(&mut *connection)
            .await?
            .ok_or(DatabaseError::NoRows())?;
        let deleted: i64 = row.get("deleted")?;
        if deleted == 0 {
            return Err(DatabaseError::NoRows());
        }
        Ok(())
    }

    // DDL statements (DROP INDEX) cannot run inside a transaction in Memgraph.
    // We therefore take a direct pool reference (`neo4rs::Graph`) instead of a
    // transaction handle (`neo4rs::Txn`) for this method.
    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.delete_graph_data",
        skip(self, graph, graph_id, node_keys),
        err
    )]
    pub async fn delete_graph_data(
        &self,
        graph: &neo4rs::Graph,
        graph_id: GraphIdModel,
        node_keys: Vec<String>,
    ) -> Result<(), DatabaseError> {
        // Delete all nodes (and their edges via DETACH) for this graph
        graph
            .run(
                query("MATCH (n { graph_id: $gid }) DETACH DELETE n")
                    .param("gid", graph_id.to_string()),
            )
            .await?;

        // Drop vector indexes for each node schema key
        for key in node_keys {
            let cypher = format!("DROP INDEX ON :{key}(embedding)");
            match graph.run(query(&cypher)).await {
                Ok(()) => {
                    tracing::debug!(key = %key, "Vector index dropped");
                }
                Err(err) => {
                    tracing::warn!(key = %key, error = ?err, "Failed to drop vector index (may not exist)");
                }
            }
        }
        Ok(())
    }

    // DDL statements (CREATE VECTOR INDEX) cannot run inside a transaction in Memgraph.
    // We therefore take a direct pool reference (`neo4rs::Graph`) instead of a
    // transaction handle (`neo4rs::Txn`) for this method.
    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.initialize_schema",
        skip(self, graph, node_keys),
        err
    )]
    pub async fn initialize_schema(
        &self,
        graph: &neo4rs::Graph,
        node_keys: Vec<String>,
    ) -> Result<(), DatabaseError> {
        for key in node_keys {
            let cypher = format!(
                "CREATE VECTOR INDEX idx_{key}_embedding ON :{key}(embedding) WITH CONFIG {{\"dimension\": 1536, \"capacity\": 10000, \"metric\": \"cos\"}}"
            );
            match graph.run(query(&cypher)).await {
                Ok(()) => {
                    tracing::debug!(key = %key, "Vector index created");
                }
                Err(err) => {
                    tracing::warn!(key = %key, error = ?err, "Failed to create vector index (may already exist)");
                }
            }
        }
        Ok(())
    }
}
