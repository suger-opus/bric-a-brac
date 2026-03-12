use std::collections::HashMap;

use crate::{
    domain::models::{EdgeSchemaModel, NodeSchemaModel, PropertySchemaModel, PropertyTypeModel},
    infrastructure::repositories::GraphRepository,
    presentation::errors::{AppError, RequestError},
};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateGraphDataDto, CreateNodeDataDto, GraphIdDto, NodeDataIdDto,
    PropertiesDataDto,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ValidationService {
    pool: PgPool,
    repository: GraphRepository,
}

impl ValidationService {
    pub fn new(pool: PgPool, repository: GraphRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_create_graph_data",
        skip(self, create_graph_data)
    )]
    pub async fn validate_create_graph_data(
        &self,
        graph_id: GraphIdDto,
        create_graph_data: &mut CreateGraphDataDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.begin().await?;
        let graph_schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;

        let id_map: HashMap<NodeDataIdDto, NodeDataIdDto> = create_graph_data
            .nodes
            .iter()
            .map(|n| (n.node_data_id, NodeDataIdDto::new()))
            .collect();

        create_graph_data
            .nodes
            .iter_mut()
            .try_for_each(|node_data| {
                self.validate_create_node_data(node_data, &graph_schema.nodes, &id_map)
            })?;

        create_graph_data
            .edges
            .iter_mut()
            .try_for_each(|edge_data| {
                self.validate_create_edge_data(edge_data, &graph_schema.edges, &id_map)
            })?;

        Ok(())
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_create_node_data",
        skip(self, node_data, node_schemas, id_map)
    )]
    fn validate_create_node_data(
        &self,
        node_data: &mut CreateNodeDataDto,
        node_schemas: &[NodeSchemaModel],
        id_map: &HashMap<NodeDataIdDto, NodeDataIdDto>,
    ) -> Result<(), AppError> {
        let schema = node_schemas
            .iter()
            .find(|s| s.key == node_data.key)
            .ok_or_else(|| RequestError::InvalidInput {
                field: format!("Node key '{}'", node_data.key),
                issue: "Node type is not defined in schema".to_string(),
            })?;

        let id = id_map
            .get(&node_data.node_data_id)
            .ok_or_else(|| RequestError::InvalidInput {
                field: format!("Node data ID '{}'", node_data.node_data_id),
                issue: "Node data ID is not defined in nodes list".to_string(),
            })?;
        node_data.node_data_id = *id;

        self.validate_properties(&node_data.properties, &schema.properties)
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_create_edge_data",
        skip(self, edge_data, edge_schemas, id_map)
    )]
    fn validate_create_edge_data(
        &self,
        edge_data: &mut CreateEdgeDataDto,
        edge_schemas: &[EdgeSchemaModel],
        id_map: &HashMap<NodeDataIdDto, NodeDataIdDto>,
    ) -> Result<(), AppError> {
        let schema = edge_schemas
            .iter()
            .find(|s| s.key == edge_data.key)
            .ok_or_else(|| RequestError::InvalidInput {
                field: format!("Edge key '{}'", edge_data.key),
                issue: "Edge type is not defined in schema".to_string(),
            })?;

        let from_id =
            id_map
                .get(&edge_data.from_node_data_id)
                .ok_or_else(|| RequestError::InvalidInput {
                    field: format!("From node data ID '{}'", edge_data.from_node_data_id),
                    issue: "From node data ID is not defined in nodes list".to_string(),
                })?;
        edge_data.from_node_data_id = *from_id;

        let to_id =
            id_map
                .get(&edge_data.to_node_data_id)
                .ok_or_else(|| RequestError::InvalidInput {
                    field: format!("To node data ID '{}'", edge_data.to_node_data_id),
                    issue: "To node data ID is not defined in nodes list".to_string(),
                })?;
        edge_data.to_node_data_id = *to_id;

        self.validate_properties(&edge_data.properties, &schema.properties)
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_properties",
        skip(self, properties_data, properties_schemas)
    )]
    fn validate_properties(
        &self,
        properties_data: &PropertiesDataDto,
        properties_schemas: &[PropertySchemaModel],
    ) -> Result<(), AppError> {
        properties_data.values.keys().try_for_each(
            |property_data_key| -> Result<(), AppError> {
                if let Some(_) = properties_schemas
                    .iter()
                    .find(|schema| schema.key == *property_data_key)
                {
                    Ok(())
                } else {
                    Err(RequestError::InvalidInput {
                        field: format!("Property {}", property_data_key),
                        issue: "Property is not defined in schema".to_string(),
                    }
                    .into())
                }
            },
        )?;

        properties_schemas.iter().try_for_each(|property_schema| {
            if let Some(property_data) = properties_data.values.get(&property_schema.key) {
                self.validate_property(property_data, property_schema)
            } else {
                Err(RequestError::InvalidInput {
                    field: format!(
                        "Property {} ({})",
                        property_schema.label, property_schema.key
                    ),
                    issue: "Required property is missing".to_string(),
                }
                .into())
            }
        })
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_property",
        skip(self, property_data_value, property_schema)
    )]
    fn validate_property(
        &self,
        property_data_value: &serde_json::Value,
        property_schema: &PropertySchemaModel,
    ) -> Result<(), AppError> {
        match (&property_schema.property_type, property_data_value) {
            (PropertyTypeModel::String, serde_json::Value::String(_)) => Ok(()),
            (PropertyTypeModel::Number, serde_json::Value::Number(_)) => Ok(()),
            (PropertyTypeModel::Boolean, serde_json::Value::Bool(_)) => Ok(()),
            (PropertyTypeModel::Select, serde_json::Value::String(value)) => {
                if let Some(options) = &property_schema.metadata.options {
                    if !options.contains(value) {
                        return Err(RequestError::InvalidInput {
                            field: format!(
                                "Property {} ({})",
                                property_schema.label, property_schema.key
                            ),
                            issue: format!(
                                "Invalid value '{}', expected one of {:?}",
                                value, options
                            ),
                        }
                        .into());
                    }
                }
                Ok(())
            }
            _ => Err(RequestError::InvalidInput {
                field: format!(
                    "Property {} ({})",
                    property_schema.label, property_schema.key
                ),
                issue: format!(
                    "Incorrect property type, expected {:?}, found {}",
                    property_schema.property_type, property_data_value
                ),
            }
            .into()),
        }
    }
}
