mod config;
mod grpc_client;

use bric_a_brac_protos::knowledge::{property_value::Value, PropertyValue};
use config::Config;
use grpc_client::KnowledgeClient;
use std::collections::HashMap;
use uuid::Uuid;

// todo: logging

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::load();
    // todo: do not show sensitive info
    println!("{:?}", config);

    println!("\nMetadata service starting...");
    println!("Connecting to Knowledge service...");
    let mut knowledge_client = KnowledgeClient::connect(config.knowledge_uri()).await?;
    println!("✓ Connected to Knowledge service");

    let graph_id = Uuid::new_v4().to_string();
    println!("\nTesting Knowledge service from Metadata service");
    println!("Graph ID: {}", graph_id);

    // Test insert_node
    println!("\nInserting Person node...");
    let mut props = HashMap::new();
    props.insert(
        "name".to_string(),
        PropertyValue {
            value: Some(Value::StringValue("Bob".to_string())),
        },
    );
    props.insert(
        "age".to_string(),
        PropertyValue {
            value: Some(Value::IntValue(25)),
        },
    );

    let result = knowledge_client
        .insert_node(graph_id.clone(), "Person".to_string(), props)
        .await?;
    println!("✓ Node created: {} nodes returned", result.nodes.len());
    let person_id = result.nodes[0].id.clone();

    // Test insert another node
    println!("\nInserting Company node...");
    let mut company_props = HashMap::new();
    company_props.insert(
        "name".to_string(),
        PropertyValue {
            value: Some(Value::StringValue("StartupCo".to_string())),
        },
    );

    let company_result = knowledge_client
        .insert_node(graph_id.clone(), "Company".to_string(), company_props)
        .await?;
    println!(
        "✓ Company created: {} nodes returned",
        company_result.nodes.len()
    );
    let company_id = company_result.nodes[0].id.clone();

    // Test insert_edge
    println!("\nInserting WORKS_AT edge...");
    let mut edge_props = HashMap::new();
    edge_props.insert(
        "role".to_string(),
        PropertyValue {
            value: Some(Value::StringValue("Developer".to_string())),
        },
    );

    let edge_result = knowledge_client
        .insert_edge(person_id, company_id, "WORKS_AT".to_string(), edge_props)
        .await?;
    println!("✓ Edge created: {} edges returned", edge_result.edges.len());

    // Test search
    println!("\nSearching for Person nodes...");
    let search_result = knowledge_client
        .search(
            Some(graph_id.clone()),
            Some("Person".to_string()),
            HashMap::new(),
            None,
            HashMap::new(),
            false,
        )
        .await?;
    println!("✓ Found {} Person nodes", search_result.nodes.len());

    println!("\n✓ All tests passed!");
    Ok(())
}
