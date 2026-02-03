use anyhow::Result;
use metadata::{config::Config, seed, setup_tracing, state::ApiState};

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
    seed::seed(&state.user_service, &state.graph_service).await?;
    tracing::info!("✅ All done!");

    Ok(())
}
