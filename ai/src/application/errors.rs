use crate::infrastructure::errors::{GrpcClientError, OpenRouterClientError};
use bric_a_brac_dtos::DtosConversionError;
use std::str::Utf8Error;

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

impl From<DtosConversionError> for AppError {
    fn from(err: DtosConversionError) -> Self {
        Self::GrpcClient(err.into())
    }
}
