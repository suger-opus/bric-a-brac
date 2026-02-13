use crate::{
    error::{ApiError, ApiErrorContent, ValidationContext},
    models::{
        EdgeSchemaId, NodeSchemaId, PropertiesData, PropertyData, PropertySchema, PropertyType,
    },
    repositories::GraphRepository,
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

    pub async fn validate_graph_schema(&self) -> Result<(), ApiError> {
        // Placeholder for future graph schema validation logic
        Ok(())
    }

    pub async fn validate_node_data(
        &self,
        node_schema_id: NodeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, ApiError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .get_node_schema(&mut txn, node_schema_id)
            .await?;
        txn.commit().await?;

        self.validate_properties(properties, &node_schema.properties)?;
        Ok(node_schema.formatted_label)
    }

    pub async fn validate_edge_data(
        &self,
        edge_schema_id: EdgeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, ApiError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .get_edge_schema(&mut txn, edge_schema_id)
            .await?;
        txn.commit().await?;

        self.validate_properties(properties, &edge_schema.properties)?;
        Ok(edge_schema.formatted_label)
    }

    fn validate_properties(
        &self,
        properties_data: &PropertiesData,
        properties_schemas: &[PropertySchema],
    ) -> Result<(), ApiError> {
        properties_data
            .0
            .keys()
            .try_for_each(|property_data_formatted_label| {
                if let Some(_) = properties_schemas
                    .iter()
                    .find(|schema| schema.formatted_label == *property_data_formatted_label)
                {
                    Ok(())
                } else {
                    Err(ApiError::ValidationError(ApiErrorContent {
                        message: format!(
                            "Property '{}' is not expected",
                            property_data_formatted_label
                        ),
                        details: ValidationContext {
                            field: property_data_formatted_label.clone(),
                            issue: "Unexpected property".to_string(),
                        },
                    }))
                }
            })?;

        properties_schemas.iter().try_for_each(|property_schema| {
            if let Some(property_data) = properties_data.0.get(&property_schema.formatted_label) {
                self.validate_property(property_data, property_schema)
            } else {
                Err(ApiError::ValidationError(ApiErrorContent {
                    message: format!(
                        "Missing required property '{}'",
                        property_schema.formatted_label
                    ),
                    details: ValidationContext {
                        field: property_schema.formatted_label.clone(),
                        issue: "Missing required property".to_string(),
                    },
                }))
            }
        })
    }

    fn validate_property(
        &self,
        property_data_value: &PropertyData,
        property_schema: &PropertySchema,
    ) -> Result<(), ApiError> {
        match (&property_schema.property_type, property_data_value) {
            (PropertyType::String, PropertyData::String(_)) => Ok(()),
            (PropertyType::Number, PropertyData::Number(_)) => Ok(()),
            (PropertyType::Boolean, PropertyData::Boolean(_)) => Ok(()),
            (PropertyType::Select, PropertyData::String(value)) => {
                if let Some(options) = &property_schema.metadata.options {
                    if !options.contains(value) {
                        Err(ApiError::ValidationError(ApiErrorContent {
                            message: format!(
                                "Property '{}' has invalid value '{}'",
                                property_schema.formatted_label, value
                            ),
                            details: ValidationContext {
                                field: property_schema.formatted_label.clone(),
                                issue: format!(
                                    "Invalid value '{}', expected one of {:?}",
                                    value, options
                                ),
                            },
                        }))?
                    }
                }
                Ok(())
            }
            _ => Err(ApiError::ValidationError(ApiErrorContent {
                message: format!(
                    "Property '{}' has incorrect type, expected {:?}, found {}",
                    property_schema.formatted_label,
                    property_schema.property_type,
                    property_data_value
                ),
                details: ValidationContext {
                    field: property_schema.formatted_label.clone(),
                    issue: "Incorrect property type".to_string(),
                },
            })),
        }
    }
}
