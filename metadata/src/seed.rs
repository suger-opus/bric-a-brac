use crate::dtos::{graph_dto::PostGraph, user_dto::PostUser};
use crate::services::{graph_service::GraphService, user_service::UserService};
use anyhow::Result;

pub async fn seed(user_service: &UserService, graph_service: &GraphService) -> Result<()> {
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

    let _graph1 = graph_service
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

    let _graph2 = graph_service
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

    let _graph3 = graph_service
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

    tracing::info!("🎉 Database seeding completed successfully!");
    Ok(())
}
