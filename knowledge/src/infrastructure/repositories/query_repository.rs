use crate::{
    domain::models::{EdgeDataModel, GraphDataModel, GraphIdModel, NodeDataIdModel, NodeDataModel},
    presentation::errors::DatabaseError,
};
use neo4rs::query;
use std::str::FromStr;

pub struct QueryRepository;

impl QueryRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn load_graph(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
    ) -> Result<GraphDataModel, DatabaseError> {
        let nodes = self.load_graph_nodes(connection, graph_id).await?;
        let edges = self.load_graph_edges(connection, graph_id).await?;

        Ok(GraphDataModel { nodes, edges })
    }

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

    async fn load_graph_edges(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
    ) -> Result<Vec<EdgeDataModel>, DatabaseError> {
        let edges_query = query(
            r#"
MATCH (a { graph_id: $graph_id })-[e]->(b { graph_id: $graph_id })
RETURN
    e,
    a.node_data_id AS from_node_data_id,
    b.node_data_id AS to_node_data_id
        "#,
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
}
