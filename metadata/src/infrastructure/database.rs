use super::config::MetadataDatabaseConfig;
use anyhow::Context;
use secrecy::ExposeSecret;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(config: &MetadataDatabaseConfig) -> anyhow::Result<PgPool> {
    tracing::debug!("Establishing database connection");

    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections())
        .connect(config.url().expose_secret())
        .await
        .context("Failed to connect to metadata database")?;

    tracing::debug!("Database connection established");
    Ok(pool)
}

pub async fn migrate(config: &MetadataDatabaseConfig, db_pool: &PgPool) -> anyhow::Result<()> {
    if config.skip_migration() {
        tracing::warn!("Metadata database migrations skipped");
    } else {
        tracing::debug!("Running database migrations");
        sqlx::migrate!("./migrations")
            .run(db_pool)
            .await
            .context("Failed to run metadata database migrations")?;
        tracing::info!("Metadata database migrations applied");
    }

    Ok(())
}
