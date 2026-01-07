use crate::models::{EdgeData, GraphData, NodeData};
use neo4rs::{query, BoltType, Graph};
use std::collections::HashMap;
use uuid::Uuid;

pub async fn insert_node(
    graph: &Graph,
    graph_id: Uuid,
    label: &str,
    mut properties: HashMap<String, BoltType>,
) -> anyhow::Result<GraphData> {
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
    let row = result
        .next()
        .await?
        .ok_or_else(|| anyhow::anyhow!("No node returned"))?;

    let neo_node: neo4rs::Node = row.get("n")?;
    let node: NodeData = (&neo_node).into();

    Ok(GraphData {
        nodes: vec![node],
        edges: vec![],
    })
}

pub async fn insert_edge(
    graph: &Graph,
    from_id: Uuid,
    to_id: Uuid,
    edge_type: &str,
    mut properties: HashMap<String, BoltType>,
) -> anyhow::Result<GraphData> {
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
        "MATCH (a {{ id: $from_id }}), (b {{ id: $to_id }}) CREATE (a)-[e:{}{}]->(b) RETURN a, e, b",
        edge_type, edge_props
    );

    let q = properties.iter().enumerate().fold(
        query(&cypher)
            .param("from_id", from_id.to_string())
            .param("to_id", to_id.to_string()),
        |q, (i, (_key, value))| q.param(&format!("p{}", i), value.clone()),
    );

    let mut result = graph.execute(q).await?;
    let row = result
        .next()
        .await?
        .ok_or_else(|| anyhow::anyhow!("No edge returned"))?;

    let from_node: neo4rs::Node = row.get("a")?;
    let neo_edge: neo4rs::Relation = row.get("e")?;
    let to_node: neo4rs::Node = row.get("b")?;

    Ok(GraphData {
        nodes: vec![(&from_node).into(), (&to_node).into()],
        edges: vec![(&neo_edge).into()],
    })
}

pub async fn search(
    graph: &Graph,
    graph_id: Option<Uuid>,
    node_label: Option<&str>,
    node_properties: HashMap<String, BoltType>,
    edge_label: Option<&str>,
    edge_properties: HashMap<String, BoltType>,
    include_edges: bool,
) -> anyhow::Result<GraphData> {
    let mut match_props = node_properties.clone();

    if let Some(gid) = graph_id {
        match_props.insert("graph_id".to_string(), gid.to_string().into());
    }

    let label_clause = node_label.map(|l| format!(":{}", l)).unwrap_or_default();
    let edge_label_clause = edge_label.map(|t| format!(":{}", t)).unwrap_or_default();

    let node_prop_clauses: Vec<String> = match_props
        .keys()
        .enumerate()
        .map(|(i, key)| format!("{}: $n{}", key, i))
        .collect();

    let node_where = if node_prop_clauses.is_empty() {
        String::new()
    } else {
        format!(" {{ {} }}", node_prop_clauses.join(", "))
    };

    let (cypher, has_edges) = if include_edges {
        let edge_prop_clauses: Vec<String> = edge_properties
            .keys()
            .enumerate()
            .map(|(i, key)| format!("{}: $e{}", key, i))
            .collect();

        let edge_where = if edge_prop_clauses.is_empty() {
            String::new()
        } else {
            format!(" {{ {} }}", edge_prop_clauses.join(", "))
        };

        (
            format!(
                "MATCH (n{}{}) OPTIONAL MATCH (n)-[e{}{}]->(m) RETURN n, e, m",
                label_clause, node_where, edge_label_clause, edge_where
            ),
            true,
        )
    } else {
        (
            format!("MATCH (n{}{}) RETURN n", label_clause, node_where),
            false,
        )
    };

    let mut q = match_props
        .iter()
        .enumerate()
        .fold(query(&cypher), |q, (i, (_key, value))| {
            q.param(&format!("n{}", i), value.clone())
        });

    if has_edges {
        q = edge_properties
            .iter()
            .enumerate()
            .fold(q, |q, (i, (_key, value))| {
                q.param(&format!("e{}", i), value.clone())
            });
    }

    let mut result = graph.execute(q).await?;
    let mut nodes_map: HashMap<String, NodeData> = HashMap::new();
    let mut edges_vec: Vec<EdgeData> = Vec::new();

    while let Some(row) = result.next().await? {
        let neo_node: neo4rs::Node = row.get("n")?;
        let node: NodeData = (&neo_node).into();
        nodes_map.insert(node.id.clone(), node);

        if has_edges {
            if let Ok(neo_edge) = row.get::<neo4rs::Relation>("e") {
                if let Ok(neo_other) = row.get::<neo4rs::Node>("m") {
                    let edge: EdgeData = (&neo_edge).into();
                    let other: NodeData = (&neo_other).into();

                    edges_vec.push(edge);
                    nodes_map.insert(other.id.clone(), other);
                }
            }
        }
    }

    Ok(GraphData {
        nodes: nodes_map.into_values().collect(),
        edges: edges_vec,
    })
}
