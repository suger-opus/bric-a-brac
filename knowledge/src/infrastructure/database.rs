use super::config::KnowledgeDatabaseConfig;
use neo4rs::{ConfigBuilder, Graph};
use std::sync::Arc;

pub async fn connect(config: &KnowledgeDatabaseConfig) -> anyhow::Result<Arc<Graph>> {
    let uri = config.url();

    let builder = ConfigBuilder::default()
        .uri(&uri)
        .user(config.user())
        .password(config.password())
        .db(config.name())
        .fetch_size(config.fetch_size())
        .max_connections(config.max_connections())
        .build()?;

    let graph = Graph::connect(builder).await?;
    test_connection(&graph).await?;

    Ok(Arc::new(graph))
}

async fn test_connection(graph: &Graph) -> anyhow::Result<()> {
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        graph.run(neo4rs::query("RETURN 1")),
    )
    .await
    .map_err(|err| {
        anyhow::anyhow!(
            "Knowledge database connection timeout - check credentials and network: {err}"
        )
    })?
    .map_err(|err| anyhow::anyhow!("Failed to verify knowledge database connection: {err}"))?;

    Ok(())
}
