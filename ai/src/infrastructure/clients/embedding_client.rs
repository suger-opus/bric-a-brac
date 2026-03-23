use crate::infrastructure::{
    config::OpenRouterConfig, errors::OpenRouterClientError, http_retry::send_with_retry,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
struct EmbeddingRequest {
    model: String,
    input: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingResponse {
    data: Vec<EmbeddingData>,
}

#[derive(Debug, Deserialize)]
struct EmbeddingData {
    embedding: Vec<f32>,
}

#[derive(Clone)]
pub struct EmbeddingClient {
    api_key: SecretString,
    embedding_model: String,
    client: reqwest::Client,
}

impl EmbeddingClient {
    #[must_use] 
    pub fn new(config: &OpenRouterConfig) -> Self {
        Self {
            api_key: config.api_key().clone(),
            embedding_model: config.embedding_model().to_owned(),
            client: reqwest::Client::new(),
        }
    }

    #[tracing::instrument(
        level = "debug",
        name = "embedding_client.embed",
        skip(self, texts),
        err
    )]
    pub async fn embed(
        &self,
        texts: Vec<String>,
    ) -> Result<Vec<Vec<f32>>, OpenRouterClientError> {
        tracing::debug!(text_count = texts.len());

        let request = EmbeddingRequest {
            model: self.embedding_model.clone(),
            input: texts,
        };

        let response = send_with_retry("OpenRouter embed", || {
            self.client
                .post("https://openrouter.ai/api/v1/embeddings")
                .header(
                    "Authorization",
                    format!("Bearer {}", &self.api_key.expose_secret()),
                )
                .header("Content-Type", "application/json")
                .json(&request)
        })
        .await?;

        let status = response.status();
        let response_text =
            response
                .text()
                .await
                .map_err(|err| OpenRouterClientError::ReadResponse {
                    message: "Failed to read OpenRouter Embeddings API response".to_owned(),
                    source: err,
                })?;

        if !status.is_success() {
            return Err(OpenRouterClientError::NoSuccessResponse {
                status,
                body: response_text,
            });
        }

        let embedding_response: EmbeddingResponse =
            serde_json::from_str(&response_text).map_err(|err| {
                tracing::error!(body = %response_text, "Failed to deserialize embedding response");
                OpenRouterClientError::Deserialization {
                    message: "Failed to deserialize EmbeddingResponse".to_owned(),
                    source: err,
                }
            })?;

        Ok(embedding_response
            .data
            .into_iter()
            .map(|d| d.embedding)
            .collect())
    }

    /// Embed a single text and return the embedding vector
    #[tracing::instrument(
        level = "debug",
        name = "embedding_client.embed_one",
        skip(self, text),
        err
    )]
    pub async fn embed_one(
        &self,
        text: String,
    ) -> Result<Vec<f32>, OpenRouterClientError> {
        let mut results = self.embed(vec![text]).await?;
        results.pop().ok_or(OpenRouterClientError::ResponseFormat {
            message: "No embedding data in response".to_owned(),
        })
    }
}
