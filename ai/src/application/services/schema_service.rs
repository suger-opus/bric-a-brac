use crate::infrastructure::clients::{MetadataClient, OpenRouterClient};
use anyhow::Context;

pub struct SchemaService {
    openrouter_client: OpenRouterClient,
    metadata_client: MetadataClient,
}

impl SchemaService {
    pub fn new(openrouter_client: OpenRouterClient, metadata_client: MetadataClient) -> Self {
        Self {
            openrouter_client,
            metadata_client,
        }
    }

    pub async fn generate_schema(&self, file_content: &[u8]) -> anyhow::Result<serde_json::Value> {
        let openapi_spec = self
            .metadata_client
            .clone()
            .get_openapi_spec()
            .await
            .context("Failed to fetch OpenAPI spec")?;
        let schema = OpenRouterClient::openai_to_structured_output_schema(&openapi_spec)
            .context("Failed to transform schema")?;

        let parsed_data = parse_file(file_content).context("Failed to parse file")?;
        let system_prompt = build_system_prompt();
        let user_prompt = build_user_prompt(&parsed_data);

        let mut previous_errors = None;
        for attempt in 1..=3 {
            tracing::info!(attempt, "Generating schema (attempt {})", attempt);

            let generated_schema = self
                .openrouter_client
                .chat(
                    &system_prompt,
                    &user_prompt,
                    schema.clone(),
                    previous_errors.as_deref(),
                )
                .await
                .context(format!("Schema generation failed on attempt {}", attempt))?;

            match self.validate_schema(&generated_schema).await {
                Ok(()) => {
                    tracing::info!("Schema validation successful on attempt {}", attempt);
                    return Ok(generated_schema);
                }
                Err(validation_errors) => {
                    tracing::warn!(
                        attempt,
                        errors = %validation_errors,
                        "Schema validation failed"
                    );

                    previous_errors = Some(validation_errors);

                    if attempt == 3 {
                        return Err(anyhow::anyhow!(
                            "Schema generation failed after 3 attempts. Last errors: {}",
                            previous_errors.unwrap()
                        ));
                    }
                }
            }
        }

        Err(anyhow::anyhow!("Schema generation failed after 3 attempts"))
    }

    async fn validate_schema(&self, schema: &serde_json::Value) -> Result<(), String> {
        let schema = serde_json::to_string(schema)
            .map_err(|e| format!("Failed to serialize schema: {}", e))?;

        let response = self
            .metadata_client
            .clone()
            .validate_schema(schema)
            .await
            .map_err(|e| format!("gRPC call failed: {}", e))?;

        if response.is_valid {
            Ok(())
        } else {
            let errors = response
                .errors
                .iter()
                .map(|e| format!("  - {}: {}", e.field, e.message))
                .collect::<Vec<_>>()
                .join("\n");
            Err(errors)
        }
    }
}

fn parse_file(content: &[u8]) -> anyhow::Result<&str> {
    std::str::from_utf8(content).context("File content is not valid UTF-8")
}

fn build_system_prompt() -> String {
    r##"You are a graph schema generator assistant. Your task is to analyze data and generate appropriate graph schemas consisting of nodes, edges, and properties.

A graph schema consists of:
- Nodes: Entities/objects (e.g., Person, Company, Product)
- Edges: Relationships between nodes (e.g., WORKS_AT, PURCHASED, KNOWS)
- Properties: Attributes of nodes and edges (e.g., name, age, since_date)

Follow these rules:
1. Use clear, descriptive labels for nodes and edges
2. Node labels should be singular nouns (Person, not People)
3. Edge labels should be verbs or relationship names in UPPER_SNAKE_CASE
4. formatted_label must use lowercase with underscores (e.g., "first_name")
5. Color must be a valid hex color code (e.g., "#3B82F6")
6. Properties must specify the correct type: Number, String, Boolean, or Select
7. If type is Select, provide options in metadata.options array
8. Identify meaningful relationships between entities in the data

Generate schemas that best represent the structure and relationships in the provided data."##.to_string()
}

fn build_user_prompt(data: &str) -> String {
    format!(
        r##"Analyze the following data and generate a graph schema:

{}

Generate a complete graph schema with nodes, edges, and their properties. Include relationships that make sense for this data."##,
        data
    )
}
