use anyhow::Result;
use metadata::config::Config;
use metadata::dtos::graph_dto::{
    ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeSchema, ReqPostProperty,
};
use metadata::dtos::user_dto::PostUser;
use metadata::models::access_model::Role;
use metadata::models::property_model::{PropertyMetadata, PropertyType};
use metadata::services::{
    access_service::AccessService, graph_service::GraphService, user_service::UserService,
};
use metadata::setup_tracing;
use metadata::state::ApiState;

#[tokio::main]
async fn main() -> Result<()> {
    setup_tracing();

    tracing::info!("🌱 Starting database seed");
    let config = Config::load()?;
    tracing::info!("� Connecting to database...");
    let pool = config.metadata_db.connect().await?;
    tracing::info!("🗑️  Resetting database schema...");
    config.metadata_db.reset(&pool).await?;
    tracing::info!("⬆️  Running migrations...");
    config.metadata_db.migrate(&pool).await?;
    tracing::info!("🌱 Seeding database...");
    pool.close().await;
    let state = ApiState::from_config(&config).await?;
    seed(
        &state.user_service,
        &state.graph_service,
        &state.access_service,
    )
    .await?;
    tracing::info!("✅ All done!");

    Ok(())
}

async fn seed(
    user_service: &UserService,
    graph_service: &GraphService,
    access_service: &AccessService,
) -> Result<()> {
    tracing::info!("Starting database seeding...");

    let alice = user_service
        .post(&PostUser {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
        })
        .await
        .expect("Failed to create alice user");

    let bob = user_service
        .post(&PostUser {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
        })
        .await
        .expect("Failed to create bob user");

    let charlie = user_service
        .post(&PostUser {
            username: "charlie".to_string(),
            email: "charlie@example.com".to_string(),
        })
        .await
        .expect("Failed to create charlie user");

    tracing::info!("✓ Created 3 users: alice, bob and charlie");

    let graph1 = graph_service
        .post(
            alice.user_id,
            &ReqPostGraph {
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
            &ReqPostGraph {
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
            &ReqPostGraph {
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
    
    graph_service
        .post_node_schema(
            graph1.graph.graph_id,
            &ReqPostNodeSchema {
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

    graph_service
        .post_node_schema(
            graph1.graph.graph_id,
            &ReqPostNodeSchema {
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

    graph_service
        .post_edge_schema(
            graph1.graph.graph_id,
            &ReqPostEdgeSchema {
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

    graph_service
        .post_node_schema(
            graph2.graph.graph_id,
            &ReqPostNodeSchema {
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

    graph_service
        .post_node_schema(
            graph2.graph.graph_id,
            &ReqPostNodeSchema {
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

    graph_service
        .post_edge_schema(
            graph2.graph.graph_id,
            &ReqPostEdgeSchema {
                label: "Cites".to_string(),
                formatted_label: "Cites".to_string(),
                color: "#06B6D4".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Cites edge schema");

    graph_service
        .post_edge_schema(
            graph2.graph.graph_id,
            &ReqPostEdgeSchema {
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

    graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            &ReqPostNodeSchema {
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

    graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            &ReqPostNodeSchema {
                label: "Document".to_string(),
                formatted_label: "Document".to_string(),
                color: "#A855F7".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Document node schema");

    graph_service
        .post_node_schema(
            graph3.graph.graph_id,
            &ReqPostNodeSchema {
                label: "Tag".to_string(),
                formatted_label: "Tag".to_string(),
                color: "#F97316".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create Tag node schema");

    graph_service
        .post_edge_schema(
            graph3.graph.graph_id,
            &ReqPostEdgeSchema {
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

    graph_service
        .post_edge_schema(
            graph3.graph.graph_id,
            &ReqPostEdgeSchema {
                label: "TaggedWith".to_string(),
                formatted_label: "Tagged_With".to_string(),
                color: "#6366F1".to_string(),
                properties: vec![],
            },
        )
        .await
        .expect("Failed to create TaggedWith edge schema");

    tracing::info!("✓ Added schemas to Knowledge Base");

    tracing::info!("🎉 Database seeding completed successfully!");
    Ok(())
}
