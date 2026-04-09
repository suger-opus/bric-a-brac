use secrecy::SecretString;

#[derive(clap::Args, derive_more::Debug)]
#[allow(clippy::struct_field_names)]
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
    pub const fn url(&self) -> &SecretString {
        &self.metadata_db_url
    }

    pub const fn max_connections(&self) -> u32 {
        self.metadata_db_max_connections
    }

    pub const fn skip_migration(&self) -> bool {
        self.metadata_db_skip_migration
    }
}
