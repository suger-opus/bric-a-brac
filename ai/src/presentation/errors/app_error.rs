use crate::presentation::errors::TonicError;
use bric_a_brac_dtos::DtosConversionError;
use bric_a_brac_protos::BaseGrpcClientError;
use std::str::Utf8Error;
use tonic::Status;

#[derive(Debug, thiserror::Error)]
pub enum GrpcClientError {
    #[error(transparent)]
    Base(#[from] BaseGrpcClientError),

    #[error(transparent)]
    Conversion(#[from] DtosConversionError),
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

    #[error("OpenRouter API returned an invalid response format: {message}")]
    ResponseFormat { message: String },

    #[error("Failed to convert OpenRouter API response: {message}: {source}")]
    ResponseConversion {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("OpenRouter API response validation failed: {source}")]
    ResponseValidation {
        #[source]
        source: validator::ValidationErrors,
    },

    #[error("OpenRouter API returned non-success status {status}: {body}")]
    NoSuccessResponse {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to deserialize OpenRouter API response: {message}: {source}")]
    Deserialization {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("AI failed to generate valid data after multiple attempts: {message}")]
    DataGeneration { message: String },
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
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Self {
        TonicError::from(err).into()
    }
}

impl From<DtosConversionError> for AppError {
    fn from(err: DtosConversionError) -> Self {
        AppError::GrpcClient(err.into())
    }
}
