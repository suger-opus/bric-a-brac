mod error;
mod service;
mod tracing;

pub use error::PresentationError;
pub use service::KnowledgeService;
pub use tracing::setup as setup_tracing;
