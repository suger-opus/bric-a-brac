use bric_a_brac_dtos::DtosConversionError;
use bric_a_brac_protos::GrpcRequestError;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
pub enum InfraError {
    #[error(transparent)]
    GrpcRequestError(#[from] GrpcRequestError),

    #[error(transparent)]
    OpenRouterClientError(#[from] OpenRouterClientError),

    #[error(transparent)]
    DtosConversionError(#[from] DtosConversionError),
}

#[derive(Debug, thiserror::Error)]
pub enum OpenRouterClientError {
    #[error(transparent)]
    HttpRequestError(#[from] HttpRequestError),

    #[error("Failed to read OpenRouter API response")]
    ReadResponse {
        #[source]
        source: reqwest::Error,
    },

    #[error("OpenRouter API returned an invalid response format: {message}")]
    ResponseFormat { message: String },

    #[error("OpenRouter API returned non-success status {status}: {body}")]
    NoSuccessResponse {
        status: reqwest::StatusCode,
        body: String,
    },

    #[error("Failed to deserialize OpenRouter API response")]
    Deserialization {
        body: String,
        #[source]
        source: serde_json::Error,
    },
}

#[derive(Debug, thiserror::Error)]
#[error("HTTP request failed")]
pub struct HttpRequestError {
    #[source]
    pub source: reqwest::Error,
}
