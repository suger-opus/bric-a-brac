use crate::infrastructure::InfraError;
use bric_a_brac_dtos::{GraphIdDto, SessionDocumentIdDto, SessionIdDto};
use std::str::Utf8Error;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    InfraError(#[from] InfraError),

    #[error(transparent)]
    AgentError(#[from] AgentError),

    #[error("File parsing failed: {message}")]
    FileParsing {
        message: String,
        #[source]
        source: Utf8Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum AgentError {
    #[error("Failed to load session '{session_id}'")]
    GetSession {
        session_id: SessionIdDto,
        #[source]
        source: InfraError,
    },

    #[error("Failed to load messages for session '{session_id}'")]
    GetSessionMessages {
        session_id: SessionIdDto,
        #[source]
        source: InfraError,
    },

    #[error("Failed to load schema for graph '{graph_id}'")]
    GetSchema {
        graph_id: GraphIdDto,
        #[source]
        source: InfraError,
    },

    #[error("Failed to load session document '{document_id}'")]
    GetSessionDocument {
        document_id: SessionDocumentIdDto,
        #[source]
        source: InfraError,
    },

    #[error("LLM call failed on iteration {iteration}")]
    LlmCall {
        iteration: usize,
        #[source]
        source: InfraError,
    },

    #[error("Internal agent error: {message}")]
    Internal {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
