use neo4rs::{query, BoltType, ConfigBuilder, Graph};
use std::collections::HashMap;
use uuid::Uuid;

async fn insert_node(
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
    let cypher = format!("CREATE (n:{} {{ {} }})", label, prop_keys.join(", "));

    let q = properties
        .iter()
        .enumerate()
        .fold(query(&cypher), |q, (i, (_key, value))| {
            q.param(&format!("p{}", i), value.clone())
        });

    graph.run(q).await?;
    Ok(node_id)
}

async fn insert_edge(
    graph: &Graph,
    from_id: Uuid,
    to_id: Uuid,
    edge_type: &str,
    mut properties: HashMap<String, BoltType>,
) -> anyhow::Result<Uuid> {
    let edge_id = Uuid::new_v4();
    properties.insert("id".to_string(), edge_id.to_string().into());

    let prop_keys: Vec<String> = properties
        .keys()
        .enumerate()
        .map(|(i, key)| format!("{}: $p{}", key, i))
        .collect();
    let edge_props = format!(" {{ {} }}", prop_keys.join(", "));
    let cypher = format!(
        "MATCH (a {{ id: $from_id }}), (b {{ id: $to_id }}) CREATE (a)-[e:{}{}]->(b)",
        edge_type, edge_props
    );

    let q = properties.iter().enumerate().fold(
        query(&cypher)
            .param("from_id", from_id.to_string())
            .param("to_id", to_id.to_string()),
        |q, (i, (_key, value))| q.param(&format!("p{}", i), value.clone()),
    );

    graph.run(q).await?;
    Ok(edge_id)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Configuring connection...");

    let config = ConfigBuilder::default()
        .uri("bolt://localhost:7687")
        .user("")
        .password("")
        .db("memgraph")
        .fetch_size(500)
        .max_connections(10)
        .build()?;

    println!("Connecting to Memgraph...");
    let graph = Graph::connect(config).await?;
    println!("✓ Connected successfully!");

    // Generate a graph_id for this user's graph
    let graph_id = Uuid::new_v4();
    println!("\nGraph ID: {}", graph_id);

    // Test the generic insert function
    println!("\nInserting nodes...");

    // Insert a Person
    let mut person_props = HashMap::new();
    person_props.insert("name".to_string(), "Alice".into());
    person_props.insert("age".to_string(), 30.into());
    person_props.insert("active".to_string(), true.into());

    let alice_id = insert_node(&graph, graph_id, "Person", person_props).await?;
    println!("✓ Person node created with ID: {}", alice_id);

    // Insert a Company with different properties
    let mut company_props = HashMap::new();
    company_props.insert("name".to_string(), BoltType::String("TechCorp".into()));
    company_props.insert("founded".to_string(), BoltType::Integer(2020.into()));

    let techcorp_id = insert_node(&graph, graph_id, "Company", company_props).await?;
    println!("✓ Company node created with ID: {}", techcorp_id);

    // Insert an edge between Alice and TechCorp
    println!("\nInserting edge...");
    let mut edge_props = HashMap::new();
    edge_props.insert("since".to_string(), 2020.into());
    edge_props.insert("role".to_string(), "Engineer".into());

    let edge_id = insert_edge(&graph, alice_id, techcorp_id, "WORKS_AT", edge_props).await?;
    println!("✓ Edge created with ID: {}", edge_id);

    println!("\n✓ All done!");
    Ok(())
}
