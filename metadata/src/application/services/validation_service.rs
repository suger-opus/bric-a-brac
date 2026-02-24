use crate::{
    domain::models::{
        CreateEdgeData, CreateNodeData, PropertiesData, PropertyData, PropertySchema, PropertyType,
    },
    infrastructure::repositories::GraphRepository,
    presentation::errors::{AppError, RequestError},
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

    #[tracing::instrument(level = "trace", skip(self, create_node_data))]
    pub async fn validate_create_node_data(
        &self,
        create_node_data: &CreateNodeData,
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

    #[tracing::instrument(level = "trace", skip(self, create_edge_data))]
    pub async fn validate_create_edge_data(
        &self,
        create_edge_data: &CreateEdgeData,
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

    #[tracing::instrument(level = "trace", skip(self, properties_data, properties_schemas))]
    fn validate_properties(
        &self,
        properties_data: &PropertiesData,
        properties_schemas: &[PropertySchema],
    ) -> Result<(), AppError> {
        properties_data
            .0
            .keys()
            .try_for_each(|property_data_key| -> Result<(), AppError> {
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
            })?;

        properties_schemas.iter().try_for_each(|property_schema| {
            if let Some(property_data) = properties_data.0.get(&property_schema.key) {
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

    #[tracing::instrument(level = "trace", skip(self, property_data_value, property_schema))]
    fn validate_property(
        &self,
        property_data_value: &PropertyData,
        property_schema: &PropertySchema,
    ) -> Result<(), AppError> {
        match (&property_schema.property_type, property_data_value) {
            (PropertyType::String, PropertyData::String(_)) => Ok(()),
            (PropertyType::Number, PropertyData::Number(_)) => Ok(()),
            (PropertyType::Boolean, PropertyData::Boolean(_)) => Ok(()),
            (PropertyType::Select, PropertyData::String(value)) => {
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
