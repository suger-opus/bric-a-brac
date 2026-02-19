use std::str::Utf8Error;

use crate::presentation::errors::TonicError;
use bric_a_brac_protos::{BaseGrpcClientError, GrpcServiceKind};
use tonic::Status;

#[derive(Debug, thiserror::Error)]
pub enum GrpcClientError {
    #[error(transparent)]
    Base(#[from] BaseGrpcClientError),

    #[error("gRPC service {service}: failed to deserialize response into {expected_struct}")]
    Deserialization {
        service: GrpcServiceKind,
        expected_struct: String,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum OpenRouterClientError {
    #[error("HTTP request failed: {message}")]
    Request {
        message: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("Failed to read OpenRouter API response: {message}")]
    ReadResponse {
        message: String,
        #[source]
        source: reqwest::Error,
    },

    #[error("OpenRouter API returned non-success status {status}: {body}")]
    NoSuccessResponse {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("OpenRouter API returned an invalid response {reason}")]
    Response { reason: String },

    #[error("Failed to deserialize OpenRouter API")]
    Deserialization {
        #[source]
        source: serde_json::Error,
    },
}

impl GrpcClientError {
    pub fn is_connection_error(&self) -> bool {
        match self {
            GrpcClientError::Base(base_err) => base_err.is_connection_error(),
            _ => false,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    GrpcClient(#[from] GrpcClientError),

    #[error(transparent)]
    OpenRouterClient(#[from] OpenRouterClientError),

    #[error("File parsing failed: {message}")]
    FileParsing {
        message: String,
        #[source]
        source: Utf8Error,
    },

    #[error("Failed to serialize schema: {message}")]
    JsonToString {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("Schema generation failed: {message}")]
    SchemaGeneration { message: String },
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        TonicError::from(err).into()
    }
}
