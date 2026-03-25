use crate::{
    domain::models::{
        EdgeDataModel, GraphDataModel, GraphIdModel, NodeDataIdModel, NodeDataModel,
        NodeSummaryModel,
    },
    infrastructure::errors::DatabaseError,
};
use neo4rs::query;
use std::str::FromStr;

#[derive(Default)]
pub struct QueryRepository;

impl QueryRepository {
    pub const fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "query_repository.load_graph",
        skip(self, connection, graph_id),
        err
    )]
    pub async fn load_graph(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
    ) -> Result<GraphDataModel, DatabaseError> {
        tracing::debug!(graph_id = ?graph_id);

        let nodes = self.load_graph_nodes(connection, graph_id).await?;
        let edges = self.load_graph_edges(connection, graph_id).await?;

        Ok(GraphDataModel { nodes, edges })
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_repository.load_graph_nodes",
        skip(self, connection, graph_id),
        err
    )]
    async fn load_graph_nodes(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
    ) -> Result<Vec<NodeDataModel>, DatabaseError> {
        let nodes_query = query("MATCH (n { graph_id: $graph_id }) RETURN n")
            .param("graph_id", graph_id.to_string());
        let mut nodes_result = connection.execute(nodes_query).await?;

        let mut nodes = Vec::new();
        while let Some(row) = nodes_result.next(&mut *connection).await? {
            let neo_node: neo4rs::Node = row.get("n")?;
            nodes.push(NodeDataModel::try_from(neo_node)?);
        }

        Ok(nodes)
    }

    #[tracing::instrument(
        level = "trace",
        name = "query_repository.load_graph_edges",
        skip(self, connection, graph_id),
        err
    )]
    async fn load_graph_edges(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
    ) -> Result<Vec<EdgeDataModel>, DatabaseError> {
        let edges_query = query(
            r"
MATCH (a { graph_id: $graph_id })-[e]->(b { graph_id: $graph_id })
RETURN
    e,
    a.node_data_id AS from_node_data_id,
    b.node_data_id AS to_node_data_id
        ",
        )
        .param("graph_id", graph_id.to_string());
        let mut edges_result = connection.execute(edges_query).await?;

        let mut edges = Vec::new();
        while let Some(row) = edges_result.next(&mut *connection).await? {
            let neo_edge: neo4rs::Relation = row.get("e")?;
            let from_node_data_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
            let to_node_data_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
            let mut edge_data = EdgeDataModel::try_from(neo_edge)?;
            edge_data.from_node_data_id = from_node_data_id;
            edge_data.to_node_data_id = to_node_data_id;

            edges.push(edge_data);
        }

        Ok(edges)
    }

    #[tracing::instrument(
        level = "debug",
        name = "query_repository.get_node",
        skip(self, connection, graph_id, node_data_id),
        err
    )]
    pub async fn get_node(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        node_data_id: NodeDataIdModel,
    ) -> Result<NodeDataModel, DatabaseError> {
        let mut result = connection
            .execute(
                query("MATCH (n { node_data_id: $nid, graph_id: $gid }) RETURN n")
                    .param("nid", node_data_id.to_string())
                    .param("gid", graph_id.to_string()),
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
        name = "query_repository.search_nodes",
        skip(self, connection, graph_id, embedding),
        err
    )]
    pub async fn search_nodes(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        node_key: Option<String>,
        embedding: Vec<f32>,
        limit: usize,
    ) -> Result<Vec<NodeSummaryModel>, DatabaseError> {
        let emb: Vec<f64> = embedding.into_iter().map(f64::from).collect();

        let labels = if let Some(key) = node_key {
            vec![key]
        } else {
            // Get all distinct labels for this graph
            let mut label_result = connection
                .execute(
                    query(
                        "MATCH (n { graph_id: $gid }) WITH labels(n) AS lbls UNWIND lbls AS lbl RETURN DISTINCT lbl",
                    )
                    .param("gid", graph_id.to_string()),
                )
                .await?;
            let mut labels = Vec::new();
            while let Some(row) = label_result.next(&mut *connection).await? {
                let lbl: String = row.get("lbl")?;
                labels.push(lbl);
            }
            labels
        };

        let mut all_results: Vec<NodeSummaryModel> = Vec::new();

        for label in labels {
            let cypher = format!(
                r#"CALL vector_search.search("idx_{label}_embedding", {limit}, $emb)
YIELD node, distance
WITH node, distance
WHERE node.graph_id = $gid
RETURN node, distance"#
            );
            let mut result = connection
                .execute(
                    query(&cypher)
                        .param("emb", emb.clone())
                        .param("gid", graph_id.to_string()),
                )
                .await?;

            while let Some(row) = result.next(&mut *connection).await? {
                let neo_node: neo4rs::Node = row.get("node")?;
                let distance: f32 = row.get("distance")?;
                let node_model = NodeDataModel::try_from(neo_node)?;
                let mut summary = NodeSummaryModel::from(node_model);
                summary.distance = distance;
                all_results.push(summary);
            }
        }

        all_results.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        all_results.truncate(limit);

        Ok(all_results)
    }

    #[tracing::instrument(
        level = "debug",
        name = "query_repository.get_neighbors",
        skip(self, connection, graph_id, node_data_id),
        err
    )]
    pub async fn get_neighbors(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        node_data_id: NodeDataIdModel,
        edge_key: Option<String>,
        depth: u32,
    ) -> Result<GraphDataModel, DatabaseError> {
        let gid = graph_id.to_string();
        let nid = node_data_id.to_string();

        // Build traversal pattern based on edge_key filter
        let (node_cypher, edge_cypher) = edge_key.as_ref().map_or_else(|| (
                format!(
                    "MATCH (n {{node_data_id: $nid, graph_id: $gid}})-[*0..{depth}]-(m {{graph_id: $gid}}) RETURN DISTINCT m"
                ),
                format!(
                    r"MATCH (n {{node_data_id: $nid, graph_id: $gid}})-[*0..{depth}]-(m {{graph_id: $gid}})
WITH collect(DISTINCT m.node_data_id) AS nids
MATCH (a {{graph_id: $gid}})-[r]->(b {{graph_id: $gid}})
WHERE a.node_data_id IN nids AND b.node_data_id IN nids
RETURN DISTINCT r, a.node_data_id AS from_node_data_id, b.node_data_id AS to_node_data_id"
                ),
            ), |ek| (
                format!(
                    "MATCH (n {{node_data_id: $nid, graph_id: $gid}})-[:{ek}*0..{depth}]-(m {{graph_id: $gid}}) RETURN DISTINCT m"
                ),
                format!(
                    r"MATCH (n {{node_data_id: $nid, graph_id: $gid}})-[:{ek}*0..{depth}]-(m {{graph_id: $gid}})
WITH collect(DISTINCT m.node_data_id) AS nids
MATCH (a {{graph_id: $gid}})-[r:{ek}]->(b {{graph_id: $gid}})
WHERE a.node_data_id IN nids AND b.node_data_id IN nids
RETURN DISTINCT r, a.node_data_id AS from_node_data_id, b.node_data_id AS to_node_data_id"
                ),
            ));

        // Query nodes
        let mut nodes = Vec::new();
        let mut node_result = connection
            .execute(
                query(&node_cypher)
                    .param("nid", nid.clone())
                    .param("gid", gid.clone()),
            )
            .await?;
        while let Some(row) = node_result.next(&mut *connection).await? {
            let neo_node: neo4rs::Node = row.get("m")?;
            nodes.push(NodeDataModel::try_from(neo_node)?);
        }

        // Query edges
        let mut edges = Vec::new();
        let mut edge_result = connection
            .execute(query(&edge_cypher).param("nid", nid).param("gid", gid))
            .await?;
        while let Some(row) = edge_result.next(&mut *connection).await? {
            let neo_edge: neo4rs::Relation = row.get("r")?;
            let from_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
            let to_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
            let mut edge_data = EdgeDataModel::try_from(neo_edge)?;
            edge_data.from_node_data_id = from_id;
            edge_data.to_node_data_id = to_id;
            edges.push(edge_data);
        }

        Ok(GraphDataModel { nodes, edges })
    }

    #[tracing::instrument(
        level = "debug",
        name = "query_repository.find_paths",
        skip(self, connection, graph_id, from_id, to_id),
        err
    )]
    pub async fn find_paths(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        from_id: NodeDataIdModel,
        to_id: NodeDataIdModel,
        max_depth: u32,
    ) -> Result<Vec<GraphDataModel>, DatabaseError> {
        let cypher = format!(
            "MATCH path = shortestPath((a {{node_data_id: $from, graph_id: $gid}})-[*..{max_depth}]-(b {{node_data_id: $to, graph_id: $gid}})) RETURN path"
        );

        let mut result = connection
            .execute(
                query(&cypher)
                    .param("from", from_id.to_string())
                    .param("to", to_id.to_string())
                    .param("gid", graph_id.to_string()),
            )
            .await?;

        let mut paths = Vec::new();

        while let Some(row) = result.next(&mut *connection).await? {
            let path: neo4rs::Path = row.get("path")?;
            let path_nodes = path.nodes();
            let path_rels = path.rels();

            let mut nodes = Vec::new();
            for neo_node in &path_nodes {
                nodes.push(NodeDataModel::try_from(neo_node.clone())?);
            }

            let mut edges = Vec::new();
            for (i, rel) in path_rels.into_iter().enumerate() {
                let mut edge = EdgeDataModel::try_from(rel)?;
                // Infer from/to from path order
                if i < path_nodes.len() - 1 {
                    edge.from_node_data_id = NodeDataIdModel::from_str(
                        &path_nodes
                            .get(i)
                            .and_then(|n| n.get::<String>("node_data_id").ok())
                            .unwrap_or_default(),
                    )?;
                    edge.to_node_data_id = NodeDataIdModel::from_str(
                        &path_nodes
                            .get(i + 1)
                            .and_then(|n| n.get::<String>("node_data_id").ok())
                            .unwrap_or_default(),
                    )?;
                }
                edges.push(edge);
            }

            paths.push(GraphDataModel { nodes, edges });
        }

        Ok(paths)
    }
}
