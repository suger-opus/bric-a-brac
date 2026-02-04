use anyhow::Context;
use secrecy::{ExposeSecret, SecretString};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(clap::Args, derive_more::Debug)]
pub struct MetadataDatabaseConfig {
    /// Metadata database URL
    #[arg(long, env = "METADATA_DB_URL", required = true)]
    metadata_db_url: SecretString,

    /// Metadata database maximum connections
    #[arg(long, env = "METADATA_DB_MAX_CONNECTIONS", default_value_t = 10)]
    metadata_db_max_connections: u32,

    /// Skip metadata database migrations
    #[arg(long, env = "METADATA_DB_SKIP_MIGRATION", default_value_t = false)]
    metadata_db_skip_migration: bool,
}

impl MetadataDatabaseConfig {
    // #[tracing::instrument(skip(self), fields(max_connections = self.metadata_db_max_connections))]
    pub async fn connect(&self) -> anyhow::Result<PgPool> {
        tracing::debug!("Establishing database connection");

        let pool = PgPoolOptions::new()
            .max_connections(self.metadata_db_max_connections)
            .connect(&self.metadata_db_url.expose_secret())
            .await
            .context("Failed to connect to metadata database")?;

        tracing::debug!("Database connection established");
        Ok(pool)
    }

    pub async fn reset(&self, pool: &PgPool) -> anyhow::Result<()> {
        tracing::debug!("Resetting database schema");

        sqlx::query("DROP SCHEMA public CASCADE")
            .execute(pool)
            .await
            .context("Failed to drop schema")?;

        sqlx::query("CREATE SCHEMA public")
            .execute(pool)
            .await
            .context("Failed to create schema")?;

        tracing::debug!("Database schema reset");
        Ok(())
    }

    pub async fn migrate(&self, db_pool: &PgPool) -> anyhow::Result<()> {
        if self.metadata_db_skip_migration {
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
}
