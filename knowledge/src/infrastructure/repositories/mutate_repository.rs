use crate::{
    domain::models::{
        CreateEdgeDataModel, CreateGraphDataModel, CreateNodeDataModel, EdgeDataModel,
        GraphDataModel, GraphIdModel, NodeDataIdModel, NodeDataModel,
    },
    presentation::errors::DatabaseError,
};
use neo4rs::{query, BoltList, BoltMap, BoltString, BoltType};
use std::{collections::HashMap, str::FromStr};

pub struct MutateRepository;

impl MutateRepository {
    pub fn new() -> Self {
        Self
    }

    #[tracing::instrument(
        level = "debug",
        name = "mutate_repository.insert_graph",
        skip(self, connection, graph_id, create_graph_data)
    )]
    pub async fn insert_graph(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        create_graph_data: CreateGraphDataModel,
    ) -> Result<GraphDataModel, DatabaseError> {
        tracing::debug!(graph_id = ?graph_id, nodes_len = create_graph_data.nodes.len(), edges_len = create_graph_data.edges.len());
        tracing::debug!(
            "Graph data: {}", serde_json::to_string_pretty(&create_graph_data).unwrap_or_default()
        );

        let nodes = self
            .insert_graph_nodes(connection, graph_id, create_graph_data.nodes)
            .await?;
        let edges = self
            .insert_graph_edges(connection, graph_id, create_graph_data.edges)
            .await?;

        Ok(GraphDataModel { nodes, edges })
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_repository.insert_graph_nodes",
        skip(self, connection, graph_id, create_nodes_data)
    )]
    async fn insert_graph_nodes(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        create_nodes_data: Vec<CreateNodeDataModel>,
    ) -> Result<Vec<NodeDataModel>, DatabaseError> {
        let mut nodes_by_label: HashMap<String, Vec<CreateNodeDataModel>> = HashMap::new();
        for node in create_nodes_data {
            nodes_by_label
                .entry(node.key.clone())
                .or_default()
                .push(node);
        }

        let mut nodes: Vec<NodeDataModel> = Vec::new();
        for (label, label_nodes) in nodes_by_label {
            let nodes_bolt = label_nodes
                .into_iter()
                .map(|n| {
                    let mut props: HashMap<BoltString, BoltType> = n.properties.try_into()?;
                    props.insert("graph_id".into(), graph_id.to_string().into());
                    props.insert("node_data_id".into(), n.node_data_id.to_string().into());
                    Ok(BoltType::Map(BoltMap { value: props }))
                })
                .collect::<Result<Vec<BoltType>, DatabaseError>>()?;

            let cypher = format!(
                "UNWIND $nodes AS props CREATE (n:{}) SET n = props RETURN n",
                label
            );
            let mut result = connection
                .execute(
                    query(&cypher).param("nodes", BoltType::List(BoltList { value: nodes_bolt })),
                )
                .await?;
            while let Some(row) = result.next(&mut *connection).await? {
                let neo_node: neo4rs::Node = row.get("n")?;
                nodes.push(NodeDataModel::try_from(neo_node)?);
            }
        }

        Ok(nodes)
    }

    #[tracing::instrument(
        level = "trace",
        name = "mutate_repository.insert_graph_edges",
        skip(self, connection, graph_id, create_edges_data)
    )]
    async fn insert_graph_edges(
        &self,
        connection: &mut neo4rs::Txn,
        graph_id: GraphIdModel,
        create_edges_data: Vec<CreateEdgeDataModel>,
    ) -> Result<Vec<EdgeDataModel>, DatabaseError> {
        let mut edges_by_type: HashMap<String, Vec<CreateEdgeDataModel>> = HashMap::new();
        for edge in create_edges_data {
            edges_by_type
                .entry(edge.key.clone())
                .or_default()
                .push(edge);
        }

        let mut edges: Vec<EdgeDataModel> = Vec::new();
        for (edge_type, type_edges) in edges_by_type {
            let expected_count = type_edges.len();
            let edges_bolt = type_edges
                .into_iter()
                .map(|e| {
                    let mut edge_props: HashMap<BoltString, BoltType> = e.properties.try_into()?;
                    edge_props.insert("edge_data_id".into(), e.edge_data_id.to_string().into());
                    edge_props.insert("graph_id".into(), graph_id.to_string().into());
                    let edge_map: HashMap<BoltString, BoltType> = [
                        ("from_id".into(), e.from_node_data_id.to_string().into()),
                        ("to_id".into(), e.to_node_data_id.to_string().into()),
                        ("props".into(), BoltType::Map(BoltMap { value: edge_props })),
                    ]
                    .into_iter()
                    .collect();
                    Ok(BoltType::Map(BoltMap { value: edge_map }))
                })
                .collect::<Result<Vec<BoltType>, DatabaseError>>()?;

            let cypher = format!(
                r#"UNWIND $edges AS e
MATCH (a {{node_data_id: e.from_id, graph_id: $graph_id}}), (b {{node_data_id: e.to_id, graph_id: $graph_id}})
CREATE (a)-[r:{}]->(b)
SET r = e.props
RETURN r, a.node_data_id AS from_node_data_id, b.node_data_id AS to_node_data_id"#,
                edge_type
            );
            let mut result = connection
                .execute(
                    query(&cypher)
                        .param("edges", BoltType::List(BoltList { value: edges_bolt }))
                        .param("graph_id", graph_id.to_string()),
                )
                .await?;
            let before = edges.len();
            while let Some(row) = result.next(&mut *connection).await? {
                let neo_edge: neo4rs::Relation = row.get("r")?;
                let from_node_data_id = NodeDataIdModel::from_str(row.get("from_node_data_id")?)?;
                let to_node_data_id = NodeDataIdModel::from_str(row.get("to_node_data_id")?)?;
                let mut edge_data = EdgeDataModel::try_from(neo_edge)?;
                edge_data.from_node_data_id = from_node_data_id;
                edge_data.to_node_data_id = to_node_data_id;
                edges.push(edge_data);
            }
            let created = edges.len() - before;
            if created != expected_count {
                return Err(DatabaseError::PartialInsert {
                    kind: edge_type,
                    expected: expected_count,
                    actual: created,
                });
            }
        }

        Ok(edges)
    }
}
