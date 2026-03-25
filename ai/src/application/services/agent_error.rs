use crate::infrastructure::errors::{GrpcClientError, OpenRouterClientError};

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Failed to load session '{session_id}'")]
    SessionLoad {
        session_id: String,
        #[source]
        source: GrpcClientError,
    },

    #[error("Failed to load messages for session '{session_id}'")]
    MessagesLoad {
        session_id: String,
        #[source]
        source: GrpcClientError,
    },

    #[error("Failed to load schema for graph '{graph_id}'")]
    SchemaLoad {
        graph_id: String,
        #[source]
        source: GrpcClientError,
    },

    #[error("LLM call failed on iteration {iteration}")]
    LlmCall {
        iteration: usize,
        #[source]
        source: OpenRouterClientError,
    },

    #[error("Failed to refresh schema for graph '{graph_id}'")]
    SchemaRefresh {
        graph_id: String,
        #[source]
        source: GrpcClientError,
    },

    #[error("Internal agent error: {message}")]
    Internal {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
