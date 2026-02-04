use crate::models::{EdgeData, GraphData, NodeData};
use neo4rs::{query, BoltType, Graph};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn insert_node(
    graph: &Graph,
    graph_id: Uuid,
    label: &str,
    mut properties: HashMap<String, BoltType>,
) -> anyhow::Result<Uuid> {
    let node_id = Uuid::new_v4();
    properties.insert("graph_id".to_string(), graph_id.to_string().into());
    properties.insert("id".to_string(), node_id.to_string().into());

    let prop_keys: Vec<String> = properties
        .keys()
        .enumerate()
        .map(|(i, key)| format!("{}: $p{}", key, i))
        .collect();
    let cypher = format!(
        "CREATE (n:{} {{ {} }}) RETURN n",
        label,
        prop_keys.join(", ")
    );

    let q = properties
        .iter()
        .enumerate()
        .fold(query(&cypher), |q, (i, (_key, value))| {
            q.param(&format!("p{}", i), value.clone())
        });

    let mut result = graph.execute(q).await?;
    result.next().await?;

    Ok(node_id)
}

pub async fn insert_edge(
    graph: &Graph,
    from_id: Uuid,
    to_id: Uuid,
    edge_type: &str,
    mut properties: HashMap<String, BoltType>,
) -> anyhow::Result<Uuid> {
    let edge_id = Uuid::new_v4();
    properties.insert("id".to_string(), edge_id.to_string().into());
    properties.insert("from_node_id".to_string(), from_id.to_string().into());
    properties.insert("to_node_id".to_string(), to_id.to_string().into());

    let prop_keys: Vec<String> = properties
        .keys()
        .enumerate()
        .map(|(i, key)| format!("{}: $p{}", key, i))
        .collect();
    let edge_props = format!(" {{ {} }}", prop_keys.join(", "));
    let cypher = format!(
        "MATCH (a {{ id: $from_id }}), (b {{ id: $to_id }}) CREATE (a)-[e:{}{}]->(b) RETURN e",
        edge_type, edge_props
    );

    let q = properties.iter().enumerate().fold(
        query(&cypher)
            .param("from_id", from_id.to_string())
            .param("to_id", to_id.to_string()),
        |q, (i, (_key, value))| q.param(&format!("p{}", i), value.clone()),
    );

    let mut result = graph.execute(q).await?;
    result.next().await?;

    Ok(edge_id)
}

pub async fn load_graph(graph: &Graph, graph_id: Uuid) -> anyhow::Result<GraphData> {
    let cypher = "MATCH (n { graph_id: $graph_id }) \
                  OPTIONAL MATCH (n)-[e]->(m { graph_id: $graph_id }) \
                  RETURN n, e, m";

    let q = query(cypher).param("graph_id", graph_id.to_string());

    let mut result = graph.execute(q).await?;
    let mut nodes_map: HashMap<String, NodeData> = HashMap::new();
    let mut edges_vec: Vec<EdgeData> = Vec::new();

    while let Some(row) = result.next().await? {
        let neo_node: neo4rs::Node = row.get("n")?;
        let node: NodeData = (&neo_node).into();
        nodes_map.insert(node.id.clone(), node);

        if let Ok(neo_edge) = row.get::<neo4rs::Relation>("e") {
            if let Ok(neo_other) = row.get::<neo4rs::Node>("m") {
                let edge: EdgeData = (&neo_edge).into();
                let other: NodeData = (&neo_other).into();

                edges_vec.push(edge);
                nodes_map.insert(other.id.clone(), other);
            }
        }
    }

    Ok(GraphData {
        nodes: nodes_map.into_values().collect(),
        edges: edges_vec,
    })
}
