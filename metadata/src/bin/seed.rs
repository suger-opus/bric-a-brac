use anyhow::Result;
use metadata::config::Config;
use metadata::dtos::{graph_dto::PostGraph, user_dto::PostUser};
use metadata::models::access_model::Role;
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
            PostGraph {
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
            PostGraph {
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
            PostGraph {
                name: "Knowledge Base".to_string(),
                description: "Personal knowledge management system".to_string(),
                is_public: false,
            },
        )
        .await
        .expect("Failed to create Knowledge Base graph");

    tracing::info!("✓ Created 3 graphs");

    access_service
        .post(alice.user_id, graph2.graph_id, Role::Viewer)
        .await
        .expect("Failed to grant alice access to Project Graph");

    access_service
        .post(bob.user_id, graph3.graph_id, Role::Admin)
        .await
        .expect("Failed to grant bob access to Research Notes");

    access_service
        .post(charlie.user_id, graph1.graph_id, Role::Editor)
        .await
        .expect("Failed to grant charlie access to Knowledge Base");

    tracing::info!("✓ Granted users access to some graphs");

    tracing::info!("🎉 Database seeding completed successfully!");
    Ok(())
}
