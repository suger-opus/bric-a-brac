use crate::{
    domain::models::{
        EdgeSchemaId, NodeSchemaId, PropertiesData, PropertyData, PropertySchema, PropertyType,
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

    #[tracing::instrument(level = "trace", skip(self, node_schema_id, properties))]
    pub async fn validate_node_data(
        &self,
        node_schema_id: NodeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, AppError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .get_node_schema(&mut txn, node_schema_id)
            .await?;
        txn.commit().await?;

        self.validate_properties(properties, &node_schema.properties)?;
        Ok(node_schema.formatted_label)
    }

    #[tracing::instrument(level = "trace", skip(self, edge_schema_id, properties))]
    pub async fn validate_edge_data(
        &self,
        edge_schema_id: EdgeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, AppError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .get_edge_schema(&mut txn, edge_schema_id)
            .await?;
        txn.commit().await?;

        self.validate_properties(properties, &edge_schema.properties)?;
        Ok(edge_schema.formatted_label)
    }

    #[tracing::instrument(level = "trace", skip(self, properties_data, properties_schemas))]
    fn validate_properties(
        &self,
        properties_data: &PropertiesData,
        properties_schemas: &[PropertySchema],
    ) -> Result<(), AppError> {
        properties_data.0.keys().try_for_each(
            |property_data_formatted_label| -> Result<(), AppError> {
                if let Some(_) = properties_schemas
                    .iter()
                    .find(|schema| schema.formatted_label == *property_data_formatted_label)
                {
                    Ok(())
                } else {
                    Err(RequestError::InvalidInput {
                        field: format!("Property {property_data_formatted_label}"),
                        issue: "Property is not defined in schema".to_string(),
                    }
                    .into())
                }
            },
        )?;

        properties_schemas.iter().try_for_each(|property_schema| {
            if let Some(property_data) = properties_data.0.get(&property_schema.formatted_label) {
                self.validate_property(property_data, property_schema)
            } else {
                Err(RequestError::InvalidInput {
                    field: format!("Property {}", property_schema.formatted_label),
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
                            field: format!("Property {}", property_schema.formatted_label),
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
                field: format!("Property {}", property_schema.formatted_label),
                issue: format!(
                    "Incorrect property type, expected {:?}, found {}",
                    property_schema.property_type, property_data_value
                ),
            }
            .into()),
        }
    }
}
