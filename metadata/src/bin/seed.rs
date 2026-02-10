use anyhow::Context;
use metadata::config::Config;
use metadata::database;
use metadata::dtos::{
    graph_dto::{
        PropertiesDto, ReqPostEdgeData, ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeData,
        ReqPostNodeSchema, ReqPostProperty, ResGraphMetadata,
    },
    user_dto::PostUser,
};
use metadata::models::{
    edge_data_model::EdgeDataId,
    edge_schema_model::{EdgeSchema, EdgeSchemaId},
    graph_model::GraphId,
    node_data_model::NodeDataId,
    node_schema_model::{NodeSchema, NodeSchemaId},
    property_model::{PropertyMetadata, PropertyType},
    user_model::User,
};
use metadata::setup_tracing;
use metadata::state::ApiState;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct NodeSchemaJson {
    label: String,
    formatted_label: String,
    color: String,
    #[serde(default)]
    properties: Vec<PropertyJson>,
}

#[derive(Debug, Deserialize)]
struct EdgeSchemaJson {
    label: String,
    formatted_label: String,
    color: String,
    #[serde(default)]
    properties: Vec<PropertyJson>,
}

#[derive(Debug, Deserialize)]
struct PropertyJson {
    label: String,
    formatted_label: String,
    property_type: String,
    #[serde(default)]
    metadata: PropertyMetadataJson,
}

impl TryFrom<&PropertyJson> for ReqPostProperty {
    type Error = anyhow::Error;

    fn try_from(property: &PropertyJson) -> Result<Self, Self::Error> {
        Ok(ReqPostProperty {
            label: property.label.clone(),
            formatted_label: property.formatted_label.clone(),
            property_type: PropertyType::try_from(property.property_type.as_str()).map_err(
                |err| {
                    anyhow::anyhow!(
                        "Invalid property type '{}': {}",
                        property.property_type,
                        err
                    )
                },
            )?,
            metadata: if property.metadata.options.is_empty() {
                PropertyMetadata::default()
            } else {
                PropertyMetadata {
                    options: Some(property.metadata.options.clone()),
                }
            },
        })
    }
}

#[derive(Debug, Deserialize, Default)]
struct PropertyMetadataJson {
    #[serde(default)]
    options: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    setup_tracing();

    tracing::info!("🌱 Starting database seed");
    let config = Config::load()?;
    tracing::info!("🔌 Connecting to database...");
    let pool = database::connect(&config.metadata_db).await?;
    tracing::info!("🗑️  Resetting database schema...");
    database::reset(&pool).await?;
    tracing::info!("⬆️  Running migrations...");
    database::migrate(&config.metadata_db, &pool).await?;
    tracing::info!("🌱 Seeding database...");
    pool.close().await;
    let state = ApiState::build(&config).await?;
    tracing::info!("Starting database seeding...");
    let user = create_user(&state).await?;
    tracing::info!("✓ Created user: admin");
    let graph = create_graph(&state, &user).await?;
    tracing::info!("✓ Created graph: European Royalty");
    let node_schemas = load_node_schemas().await?;
    tracing::info!("✓ Loaded {} node schemas", node_schemas.len());
    let node_schemas = create_node_schemas(&state, graph.graph.graph_id, node_schemas).await?;
    tracing::info!("✓ Created {} node schemas", node_schemas.len());
    let edge_schemas = load_edge_schemas().await?;
    tracing::info!("✓ Loaded {} edge schemas", edge_schemas.len());
    let edge_schemas = create_edge_schemas(&state, graph.graph.graph_id, edge_schemas).await?;
    tracing::info!("✓ Created {} edge schemas", edge_schemas.len());
    let node_data = create_node_data(&state, graph.graph.graph_id, &node_schemas).await?;
    tracing::info!("✓ Created {} nodes", node_data.len());
    let edge_data =
        create_edge_data(&state, graph.graph.graph_id, &edge_schemas, &node_data).await?;
    tracing::info!("✓ Created {} edges", edge_data.len());
    tracing::info!("🎉 Database seeding completed successfully!");
    tracing::info!("✅ All done!");

    Ok(())
}

async fn create_user(state: &ApiState) -> anyhow::Result<User> {
    state
        .user_service
        .post(PostUser {
            username: "admin".to_string(),
            email: "admin@example.com".to_string(),
        })
        .await
        .map_err(|err| anyhow::anyhow!("Failed to create user: {:?}", err))
}

async fn create_graph(state: &ApiState, user: &User) -> anyhow::Result<ResGraphMetadata> {
    state
        .graph_service
        .post(
            user.user_id,
            ReqPostGraph {
                name: "European Royalty".to_string(),
                description: "Historical graph of European monarchies, dynasties, and conflicts"
                    .to_string(),
                is_public: true,
            },
        )
        .await
        .map_err(|err| anyhow::anyhow!("Failed to create graph: {:?}", err))
}

async fn load_node_schemas() -> anyhow::Result<Vec<NodeSchemaJson>> {
    let node_schemas = std::fs::read_to_string("src/bin/dataset/nodes/schemas.json")
        .context("Failed to read node schemas.json")?;
    let node_schemas: Vec<NodeSchemaJson> =
        serde_json::from_str(&node_schemas).context("Failed to parse node schemas")?;

    Ok(node_schemas)
}

async fn load_edge_schemas() -> anyhow::Result<Vec<EdgeSchemaJson>> {
    let edge_schemas = std::fs::read_to_string("src/bin/dataset/edges/schemas.json")
        .context("Failed to read edge schemas.json")?;
    let edge_schemas: Vec<EdgeSchemaJson> =
        serde_json::from_str(&edge_schemas).context("Failed to parse edge schemas")?;

    Ok(edge_schemas)
}

async fn create_node_schemas(
    state: &ApiState,
    graph_id: GraphId,
    node_schemas: Vec<NodeSchemaJson>,
) -> anyhow::Result<HashMap<NodeSchemaId, NodeSchema>> {
    let mut node_schema_map = HashMap::new();

    for schema in &node_schemas {
        let properties: Vec<ReqPostProperty> = schema
            .properties
            .iter()
            .map(|property| property.try_into())
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("Invalid properties in node schema {}", schema.label))?;

        let response = state
            .graph_service
            .post_node_schema(
                graph_id,
                ReqPostNodeSchema {
                    label: schema.label.clone(),
                    formatted_label: schema.formatted_label.clone(),
                    color: schema.color.clone(),
                    properties,
                },
            )
            .await
            .map_err(|err| {
                anyhow::anyhow!("Failed to create node schema {}: {:?}", schema.label, err)
            })?;

        node_schema_map.insert(
            response.node_schema.node_schema_id.clone(),
            response.node_schema,
        );
    }

    Ok(node_schema_map)
}

async fn create_edge_schemas(
    state: &ApiState,
    graph_id: GraphId,
    edge_schemas: Vec<EdgeSchemaJson>,
) -> anyhow::Result<HashMap<EdgeSchemaId, EdgeSchema>> {
    let mut edge_schema_map = HashMap::new();

    for schema in &edge_schemas {
        let properties: Vec<ReqPostProperty> = schema
            .properties
            .iter()
            .map(|property| property.try_into())
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| format!("Invalid properties in edge schema {}", schema.label))?;

        let response = state
            .graph_service
            .post_edge_schema(
                graph_id,
                ReqPostEdgeSchema {
                    label: schema.label.clone(),
                    formatted_label: schema.formatted_label.clone(),
                    color: schema.color.clone(),
                    properties,
                },
            )
            .await
            .map_err(|err| {
                anyhow::anyhow!("Failed to create edge schema {}: {:?}", schema.label, err)
            })?;

        edge_schema_map.insert(
            response.edge_schema.edge_schema_id.clone(),
            response.edge_schema,
        );
    }

    Ok(edge_schema_map)
}

trait ExtendCsvRecord {
    fn extract_properties(
        &self,
        headers: &csv::StringRecord,
        skip: usize,
    ) -> anyhow::Result<HashMap<String, serde_json::Value>>;
}

impl ExtendCsvRecord for csv::StringRecord {
    fn extract_properties(
        &self,
        headers: &csv::StringRecord,
        skip: usize,
    ) -> anyhow::Result<HashMap<String, serde_json::Value>> {
        let mut properties = HashMap::new();

        for (i, value) in self.iter().enumerate().skip(skip) {
            if !value.is_empty() {
                let header = headers
                    .get(i)
                    .with_context(|| format!("Unable to read header at index {}", i))?;
                if let Ok(num) = value.parse::<i32>() {
                    properties.insert(header.to_string(), json!(num));
                } else if let Ok(bool) = value.parse::<bool>() {
                    properties.insert(header.to_string(), json!(bool));
                } else {
                    properties.insert(header.to_string(), json!(value));
                }
            }
        }

        Ok(properties)
    }
}

fn read_csv(formatted_label: &str, is_node: bool) -> anyhow::Result<(String, String)> {
    let folder = if is_node { "nodes" } else { "edges" };
    let filename = formatted_label.to_lowercase() + ".csv";
    let csv_path = format!("src/bin/dataset/{}/{}", folder, filename);
    let content = std::fs::read_to_string(&csv_path)
        .with_context(|| format!("Unable to read file {}", &csv_path))?;

    Ok((csv_path, content))
}

async fn create_node_data(
    state: &ApiState,
    graph_id: GraphId,
    node_schemas: &HashMap<NodeSchemaId, NodeSchema>,
) -> anyhow::Result<HashMap<String, NodeDataId>> {
    let mut node_data: HashMap<String, NodeDataId> = HashMap::new();

    for (node_schema_id, node_schema) in node_schemas {
        let (csv_path, content) = read_csv(&node_schema.formatted_label, true)?;
        let mut csv_reader = csv::Reader::from_reader(content.as_bytes());
        let headers = csv_reader
            .headers()
            .with_context(|| format!("Failed to read CSV headers for file {}", csv_path))?
            .clone();

        for result in csv_reader.records() {
            let record = result
                .with_context(|| format!("Failed to read CSV record for file {}", csv_path))?;
            let node_id = record
                .get(0)
                .with_context(|| format!("Missing record ID in file {}", csv_path))?
                .to_string();
            let properties = record.extract_properties(&headers, 1).with_context(|| {
                format!(
                    "Failed to extract properties for node {} in file {}",
                    node_id, csv_path
                )
            })?;

            let response = state
                .graph_service
                .post_node_data(
                    graph_id,
                    ReqPostNodeData {
                        node_schema_id: *node_schema_id,
                        formatted_label: node_schema.formatted_label.clone(),
                        properties: PropertiesDto(properties),
                    },
                )
                .await
                .map_err(|err| {
                    anyhow::anyhow!(
                        "Failed to create node {} in file {}: {:?}",
                        node_id,
                        csv_path,
                        err
                    )
                })?;

            node_data.insert(node_id, response.node_data_id);
        }
    }

    Ok(node_data)
}

async fn create_edge_data(
    state: &ApiState,
    graph_id: GraphId,
    edge_schemas: &HashMap<EdgeSchemaId, EdgeSchema>,
    node_data_map: &HashMap<String, NodeDataId>,
) -> anyhow::Result<HashMap<String, EdgeDataId>> {
    let mut edge_data: HashMap<String, EdgeDataId> = HashMap::new();

    for (edge_schema_id, edge_schema) in edge_schemas {
        let (csv_path, content) = read_csv(&edge_schema.formatted_label, false)?;
        let mut csv_reader = csv::Reader::from_reader(content.as_bytes());
        let headers = csv_reader
            .headers()
            .with_context(|| format!("Failed to read CSV headers for file {}", csv_path))?
            .clone();

        for result in csv_reader.records() {
            let record = result
                .with_context(|| format!("Failed to read CSV record for file {}", csv_path))?;
            let edge_id = record
                .get(0)
                .with_context(|| format!("Missing record ID in file {}", csv_path))?
                .to_string();
            let from_node_id = record
                .get(1)
                .with_context(|| format!("Failed to read from node ID in file {}", csv_path))?;
            let to_node_id = record
                .get(2)
                .with_context(|| format!("Failed to read to node ID in file {}", csv_path))?;
            let properties = record.extract_properties(&headers, 3).with_context(|| {
                format!(
                    "Failed to extract properties for edge {} -> {} in file {}",
                    from_node_id, to_node_id, csv_path
                )
            })?;
            let from_node_data_id = node_data_map
                .get(from_node_id)
                .with_context(|| format!("Node not found: {}", from_node_id))?;
            let to_node_data_id = node_data_map
                .get(to_node_id)
                .with_context(|| format!("Node not found: {}", to_node_id))?;

            let response = state
                .graph_service
                .post_edge_data(
                    graph_id,
                    ReqPostEdgeData {
                        edge_schema_id: *edge_schema_id,
                        from_node_data_id: *from_node_data_id,
                        to_node_data_id: *to_node_data_id,
                        formatted_label: edge_schema.formatted_label.clone(),
                        properties: PropertiesDto(properties),
                    },
                )
                .await
                .map_err(|err| {
                    anyhow::anyhow!(
                        "Failed to create edge {} ({} -> {}) in file {}: {:?}",
                        edge_schema.formatted_label,
                        from_node_id,
                        to_node_id,
                        csv_path,
                        err
                    )
                })?;

            edge_data.insert(edge_id, response.edge_data_id);
        }
    }

    Ok(edge_data)
}
