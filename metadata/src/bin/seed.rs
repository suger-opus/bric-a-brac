use anyhow::Result;
use metadata::config::Config;
use metadata::database;
use metadata::dtos::{
    graph_dto::{
        PropertiesDto, ReqPostEdgeData, ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeData,
        ReqPostNodeSchema, ReqPostProperty,
    },
    user_dto::PostUser,
};
use metadata::models::{
    access_model::Role,
    property_model::{PropertyMetadata, PropertyType},
};
use metadata::services::{
    access_service::AccessService, graph_service::GraphService, user_service::UserService,
};
use metadata::setup_tracing;
use metadata::state::ApiState;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();

    tracing::info!("🌱 Starting database seed");
    let config = Config::load()?;
    tracing::info!("� Connecting to database...");
    let pool = database::connect(&config.metadata_db).await?;
    tracing::info!("🗑️  Resetting database schema...");
    database::reset(&pool).await?;
    tracing::info!("⬆️  Running migrations...");
    database::migrate(&config.metadata_db, &pool).await?;
    tracing::info!("🌱 Seeding database...");
    pool.close().await;
    let state = ApiState::build(&config).await?;
    seed(
        state.user_service,
        state.graph_service,
        state.access_service,
    )
    .await?;
    tracing::info!("✅ All done!");

    Ok(())
}

async fn seed(
    user_service: UserService,
    graph_service: GraphService,
    access_service: AccessService,
) -> Result<()> {
    tracing::info!("Starting database seeding...");

    let alice = user_service
        .post(PostUser {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        })
        .await
        .expect("Failed to create alice user");

    let bob = user_service
        .post(PostUser {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
        })
        .await
        .expect("Failed to create bob user");

    let charlie = user_service
        .post(PostUser {
            username: "charlie".to_string(),
            email: "charlie@example.com".to_string(),
        })
        .await
        .expect("Failed to create charlie user");

    tracing::info!("✓ Created 3 users: alice, bob and charlie");

    let graph1 = graph_service
        .post(
            alice.user_id,
            ReqPostGraph {
                name: "Project Graph".to_string(),
                description: "Software project dependencies".to_string(),
                is_public: false,
            },
        )
        .await
        .expect("Failed to create Project Graph");

    let graph2 = graph_service
        .post(
            bob.user_id,
            ReqPostGraph {
                name: "Research Notes".to_string(),
                description: "Academic research graph".to_string(),
                is_public: false,
            },
        )
        .await
        .expect("Failed to create Research Notes graph");

    let graph3 = graph_service
        .post(
            charlie.user_id,
            ReqPostGraph {
                name: "Knowledge Base".to_string(),
                description: "Personal knowledge management system".to_string(),
                is_public: false,
            },
        )
        .await
        .expect("Failed to create Knowledge Base graph");

    tracing::info!("✓ Created 3 graphs");

    access_service
        .post(alice.user_id, graph2.graph.graph_id, Role::Viewer)
        .await
        .expect("Failed to grant alice access to Project Graph");

    access_service
        .post(bob.user_id, graph3.graph.graph_id, Role::Admin)
        .await
        .expect("Failed to grant bob access to Research Notes");

    access_service
        .post(charlie.user_id, graph1.graph.graph_id, Role::Editor)
        .await
        .expect("Failed to grant charlie access to Knowledge Base");

    tracing::info!("✓ Granted users access to some graphs");

    tracing::info!("Adding schemas to Project Graph...");

    let package_schema = graph_service
        .post_node_schema(
            graph1.graph.graph_id,
            ReqPostNodeSchema {
                label: "Package".to_string(),
                formatted_label: "Package".to_string(),
                color: "#3B82F6".to_string(),
                properties: vec![
                    ReqPostProperty {
                        label: "name".to_string(),
                        formatted_label: "Name".to_string(),
                        property_type: PropertyType::String,
                        metadata: PropertyMetadata::default(),
                    },
                    ReqPostProperty {
                        label: "version".to_string(),
                        formatted_label: "Version".to_string(),
                        property_type: PropertyType::String,
                        metadata: PropertyMetadata::default(),
                    },
                ],
            },
        )
        .await
        .expect("Failed to create Package node schema");

    let module_schema = graph_service
        .post_node_schema(
            graph1.graph.graph_id,
            ReqPostNodeSchema {
                label: "Module".to_string(),
                formatted_label: "Module".to_string(),
                color: "#10B981".to_string(),
                properties: vec![ReqPostProperty {
                    label: "path".to_string(),
                    formatted_label: "Path".to_string(),
                    property_type: PropertyType::String,
                    metadata: PropertyMetadata::default(),
                }],
            },
        )
        .await
        .expect("Failed to create Module node schema");

    let depends_on_schema = graph_service
        .post_edge_schema(
            graph1.graph.graph_id,
            ReqPostEdgeSchema {
                label: "DependsOn".to_string(),
                formatted_label: "Depends_On".to_string(),
                color: "#EF4444".to_string(),
                properties: vec![ReqPostProperty {
                    label: "constraint".to_string(),
                    formatted_label: "Version_Constraint".to_string(),
                    property_type: PropertyType::String,
                    metadata: PropertyMetadata::default(),
                }],
            },
        )
        .await
        .expect("Failed to create DependsOn edge schema");

    tracing::info!("✓ Added schemas to Project Graph");

    tracing::info!("Adding schemas to Research Notes...");

    let paper_schema = graph_service
        .post_node_schema(
            graph2.graph.graph_id,
            ReqPostNodeSchema {
                label: "Paper".to_string(),
                formatted_label: "Paper".to_string(),
                color: "#8B5CF6".to_string(),
                properties: vec![
                    ReqPostProperty {
                        label: "title".to_string(),
                        formatted_label: "Title".to_string(),
                        property_type: PropertyType::String,
                        metadata: PropertyMetadata::default(),
                    },
                    ReqPostProperty {
                        label: "year".to_string(),
                        formatted_label: "Publication_Year".to_string(),
                        property_type: PropertyType::Number,
                        metadata: PropertyMetadata::default(),
                    },
                ],
            },
        )
        .await
        .expect("Failed to create Paper node schema");

    let author_schema = graph_service
        .post_node_schema(
            graph2.graph.graph_id,
            ReqPostNodeSchema {
                label: "Author".to_string(),
                formatted_label: "Author".to_string(),
                color: "#F59E0B".to_string(),
                properties: vec![ReqPostProperty {
                    label: "full_name".to_string(),
                    formatted_label: "Full_Name".to_string(),
                    property_type: PropertyType::String,
                    metadata: PropertyMetadata::default(),
                }],
            },
        )
        .await
        .expect("Failed to create Author node schema");

    let cites_schema = graph_service
        .post_edge_schema(
            graph2.graph.graph_id,
            ReqPostEdgeSchema {
                label: "Cites".to_string(),
                formatted_label: "Cites".to_string(),
                color: "#06B6D4".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Cites edge schema");

    let authored_by_schema = graph_service
        .post_edge_schema(
            graph2.graph.graph_id,
            ReqPostEdgeSchema {
                label: "AuthoredBy".to_string(),
                formatted_label: "Authored_By".to_string(),
                color: "#EC4899".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create AuthoredBy edge schema");

    tracing::info!("✓ Added schemas to Research Notes");

    tracing::info!("Adding schemas to Knowledge Base...");

    let topic_schema = graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            ReqPostNodeSchema {
                label: "Topic".to_string(),
                formatted_label: "Topic".to_string(),
                color: "#14B8A6".to_string(),
                properties: vec![
                    ReqPostProperty {
                        label: "name".to_string(),
                        formatted_label: "Name".to_string(),
                        property_type: PropertyType::String,
                        metadata: PropertyMetadata::default(),
                    },
                    ReqPostProperty {
                        label: "importance".to_string(),
                        formatted_label: "Importance_Level".to_string(),
                        property_type: PropertyType::Number,
                        metadata: PropertyMetadata::default(),
                    },
                ],
            },
        )
        .await
        .expect("Failed to create Topic node schema");

    let document_schema = graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            ReqPostNodeSchema {
                label: "Document".to_string(),
                formatted_label: "Document".to_string(),
                color: "#A855F7".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Document node schema");

    let tag_schema = graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            ReqPostNodeSchema {
                label: "Tag".to_string(),
                formatted_label: "Tag".to_string(),
                color: "#F97316".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Tag node schema");

    let contains_schema = graph_service
        .post_edge_schema(
            graph3.graph.graph_id,
            ReqPostEdgeSchema {
                label: "Contains".to_string(),
                formatted_label: "Contains".to_string(),
                color: "#84CC16".to_string(),
                properties: vec![ReqPostProperty {
                    label: "order".to_string(),
                    formatted_label: "Order".to_string(),
                    property_type: PropertyType::Number,
                    metadata: PropertyMetadata::default(),
                }],
            },
        )
        .await
        .expect("Failed to create Contains edge schema");

    let tagged_with_schema = graph_service
        .post_edge_schema(
            graph3.graph.graph_id,
            ReqPostEdgeSchema {
                label: "TaggedWith".to_string(),
                formatted_label: "Tagged_With".to_string(),
                color: "#6366F1".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create TaggedWith edge schema");

    tracing::info!("✓ Added schemas to Knowledge Base");

    tracing::info!("Adding nodes and edges to Project Graph...");

    let mut props1 = HashMap::new();
    props1.insert("Name".to_string(), json!("tokio"));
    props1.insert("Version".to_string(), json!("1.41.0"));
    let tokio_node = graph_service
        .post_node_data(
            graph1.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: package_schema.node_schema.node_schema_id,
                formatted_label: "Package".to_string(),
                properties: PropertiesDto(props1),
            },
        )
        .await
        .expect("Failed to insert tokio node");

    let mut props2 = HashMap::new();
    props2.insert("Name".to_string(), json!("axum"));
    props2.insert("Version".to_string(), json!("0.7.0"));
    let axum_node = graph_service
        .post_node_data(
            graph1.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: package_schema.node_schema.node_schema_id,
                formatted_label: "Package".to_string(),
                properties: PropertiesDto(props2),
            },
        )
        .await
        .expect("Failed to insert axum node");

    let mut props3 = HashMap::new();
    props3.insert("Path".to_string(), json!("src/main.rs"));
    let main_node = graph_service
        .post_node_data(
            graph1.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: module_schema.node_schema.node_schema_id,
                formatted_label: "Module".to_string(),
                properties: PropertiesDto(props3),
            },
        )
        .await
        .expect("Failed to insert main module node");

    let mut edge_props1 = HashMap::new();
    edge_props1.insert("Version_Constraint".to_string(), json!("^1.0"));
    graph_service
        .post_edge_data(
            graph1.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: depends_on_schema.edge_schema.edge_schema_id,
                from_node_data_id: axum_node.node_data_id,
                to_node_data_id: tokio_node.node_data_id,
                formatted_label: "Depends_On".to_string(),
                properties: PropertiesDto(edge_props1),
            },
        )
        .await
        .expect("Failed to insert DependsOn edge");

    let mut edge_props2 = HashMap::new();
    edge_props2.insert("Version_Constraint".to_string(), json!("^0.7"));
    graph_service
        .post_edge_data(
            graph1.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: depends_on_schema.edge_schema.edge_schema_id,
                from_node_data_id: main_node.node_data_id,
                to_node_data_id: axum_node.node_data_id,
                formatted_label: "Depends_On".to_string(),
                properties: PropertiesDto(edge_props2),
            },
        )
        .await
        .expect("Failed to insert DependsOn edge");

    tracing::info!("✓ Added data to Project Graph (3 nodes, 2 edges)");

    tracing::info!("Adding nodes and edges to Research Notes...");

    let mut props4 = HashMap::new();
    props4.insert("Title".to_string(), json!("Attention Is All You Need"));
    props4.insert("Publication_Year".to_string(), json!(2017));
    let paper1_node = graph_service
        .post_node_data(
            graph2.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: paper_schema.node_schema.node_schema_id,
                formatted_label: "Paper".to_string(),
                properties: PropertiesDto(props4),
            },
        )
        .await
        .expect("Failed to insert paper1 node");

    let mut props5 = HashMap::new();
    props5.insert(
        "Title".to_string(),
        json!("BERT: Pre-training of Deep Bidirectional Transformers"),
    );
    props5.insert("Publication_Year".to_string(), json!(2018));
    let paper2_node = graph_service
        .post_node_data(
            graph2.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: paper_schema.node_schema.node_schema_id,
                formatted_label: "Paper".to_string(),
                properties: PropertiesDto(props5),
            },
        )
        .await
        .expect("Failed to insert paper2 node");

    let mut props6 = HashMap::new();
    props6.insert("Full_Name".to_string(), json!("Ashish Vaswani"));
    let author_node = graph_service
        .post_node_data(
            graph2.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: author_schema.node_schema.node_schema_id,
                formatted_label: "Author".to_string(),
                properties: PropertiesDto(props6),
            },
        )
        .await
        .expect("Failed to insert author node");

    graph_service
        .post_edge_data(
            graph2.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: cites_schema.edge_schema.edge_schema_id,
                from_node_data_id: paper2_node.node_data_id,
                to_node_data_id: paper1_node.node_data_id,
                formatted_label: "Cites".to_string(),
                properties: PropertiesDto(HashMap::new()),
            },
        )
        .await
        .expect("Failed to insert Cites edge");

    graph_service
        .post_edge_data(
            graph2.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: authored_by_schema.edge_schema.edge_schema_id,
                from_node_data_id: paper1_node.node_data_id,
                to_node_data_id: author_node.node_data_id,
                formatted_label: "Authored_By".to_string(),
                properties: PropertiesDto(HashMap::new()),
            },
        )
        .await
        .expect("Failed to insert AuthoredBy edge");

    tracing::info!("✓ Added data to Research Notes (3 nodes, 2 edges)");

    tracing::info!("Adding nodes and edges to Knowledge Base...");

    let mut props7 = HashMap::new();
    props7.insert("Name".to_string(), json!("Rust Programming"));
    props7.insert("Importance_Level".to_string(), json!(10));
    let topic1_node = graph_service
        .post_node_data(
            graph3.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: topic_schema.node_schema.node_schema_id,
                formatted_label: "Topic".to_string(),
                properties: PropertiesDto(props7),
            },
        )
        .await
        .expect("Failed to insert topic1 node");

    let mut props8 = HashMap::new();
    props8.insert("Name".to_string(), json!("Async Programming"));
    props8.insert("Importance_Level".to_string(), json!(8));
    let topic2_node = graph_service
        .post_node_data(
            graph3.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: topic_schema.node_schema.node_schema_id,
                formatted_label: "Topic".to_string(),
                properties: PropertiesDto(props8),
            },
        )
        .await
        .expect("Failed to insert topic2 node");

    let doc_node = graph_service
        .post_node_data(
            graph3.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: document_schema.node_schema.node_schema_id,
                formatted_label: "Document".to_string(),
                properties: PropertiesDto(HashMap::new()),
            },
        )
        .await
        .expect("Failed to insert document node");

    let tag_node = graph_service
        .post_node_data(
            graph3.graph.graph_id,
            ReqPostNodeData {
                node_schema_id: tag_schema.node_schema.node_schema_id,
                formatted_label: "Tag".to_string(),
                properties: PropertiesDto(HashMap::new()),
            },
        )
        .await
        .expect("Failed to insert tag node");

    let mut edge_props3 = HashMap::new();
    edge_props3.insert("Order".to_string(), json!(1));
    graph_service
        .post_edge_data(
            graph3.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: contains_schema.edge_schema.edge_schema_id,
                from_node_data_id: topic1_node.node_data_id,
                to_node_data_id: topic2_node.node_data_id,
                formatted_label: "Contains".to_string(),
                properties: PropertiesDto(edge_props3),
            },
        )
        .await
        .expect("Failed to insert Contains edge");

    graph_service
        .post_edge_data(
            graph3.graph.graph_id,
            ReqPostEdgeData {
                edge_schema_id: tagged_with_schema.edge_schema.edge_schema_id,
                from_node_data_id: doc_node.node_data_id,
                to_node_data_id: tag_node.node_data_id,
                formatted_label: "Tagged_With".to_string(),
                properties: PropertiesDto(HashMap::new()),
            },
        )
        .await
        .expect("Failed to insert TaggedWith edge");

    tracing::info!("✓ Added data to Knowledge Base (4 nodes, 2 edges)");

    tracing::info!("🎉 Database seeding completed successfully!");
    Ok(())
}
