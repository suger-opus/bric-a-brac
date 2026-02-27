use crate::{
    domain::models::{PropertySchemaModel, PropertyTypeModel},
    infrastructure::repositories::GraphRepository,
    presentation::errors::{AppError, RequestError},
};
use bric_a_brac_dtos::{CreateEdgeDataDto, CreateNodeDataDto, PropertiesDataDto};
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
        name = "validation_service.validate_create_node_data",
        skip(self, create_node_data)
    )]
    pub async fn validate_create_node_data(
        &self,
        create_node_data: &CreateNodeDataDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .get_node_schema_by_key(&mut txn, create_node_data.key.clone())
            .await?;
        txn.commit().await?;

        self.validate_properties(&create_node_data.properties, &node_schema.properties)?;
        Ok(())
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_create_edge_data",
        skip(self, create_edge_data)
    )]
    pub async fn validate_create_edge_data(
        &self,
        create_edge_data: &CreateEdgeDataDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .get_edge_schema_by_key(&mut txn, create_edge_data.key.clone())
            .await?;
        txn.commit().await?;

        self.validate_properties(&create_edge_data.properties, &edge_schema.properties)?;
        Ok(())
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
