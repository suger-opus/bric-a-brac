mod config;
pub mod database;
mod error;
mod repositories;

pub use config::Config;
pub use error::DatabaseError;
pub use repositories::{MutateRepository, QueryRepository};

use config::KnowledgeDatabaseConfig;
