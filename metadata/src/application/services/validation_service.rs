use crate::{
    domain::models::{
        EdgeSchemaId, NodeSchemaId, PropertiesData, PropertyData, PropertySchema, PropertyType,
    },
    infrastructure::repositories::GraphRepository,
    presentation::error::{AppError, DomainError, InfraError, ResultExt},
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

    pub async fn validate_node_data(
        &self,
        node_schema_id: NodeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for node validation")?;
        let node_schema = self
            .repository
            .get_node_schema(&mut txn, node_schema_id)
            .await
            .map_err(|e| match e {
                AppError::Infra(InfraError::Database { source, .. })
                    if matches!(source, sqlx::Error::RowNotFound) =>
                {
                    DomainError::NotFound {
                        entity: "NodeSchema".to_string(),
                        identifier: node_schema_id.to_string(),
                    }
                    .into()
                }
                other => other,
            })?;
        txn.commit()
            .await
            .context("Failed to commit transaction after node schema fetch")?;

        self.validate_properties(properties, &node_schema.properties)?;
        Ok(node_schema.formatted_label)
    }

    pub async fn validate_edge_data(
        &self,
        edge_schema_id: EdgeSchemaId,
        properties: &PropertiesData,
    ) -> Result<String, AppError> {
        let mut txn = self
            .pool
            .begin()
            .await
            .context("Failed to start transaction for edge validation")?;
        let edge_schema = self
            .repository
            .get_edge_schema(&mut txn, edge_schema_id)
            .await
            .map_err(|e| match e {
                AppError::Infra(InfraError::Database { source, .. })
                    if matches!(source, sqlx::Error::RowNotFound) =>
                {
                    DomainError::NotFound {
                        entity: "EdgeSchema".to_string(),
                        identifier: edge_schema_id.to_string(),
                    }
                    .into()
                }
                other => other,
            })?;
        txn.commit()
            .await
            .context("Failed to commit transaction after edge schema fetch")?;

        self.validate_properties(properties, &edge_schema.properties)?;
        Ok(edge_schema.formatted_label)
    }

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
                    Err(DomainError::PropertyValidation {
                        property: property_data_formatted_label.clone(),
                        reason: "Property is not defined in schema".to_string(),
                    }
                    .into())
                }
            },
        )?;

        properties_schemas.iter().try_for_each(|property_schema| {
            if let Some(property_data) = properties_data.0.get(&property_schema.formatted_label) {
                self.validate_property(property_data, property_schema)
            } else {
                Err(DomainError::PropertyValidation {
                    property: property_schema.formatted_label.clone(),
                    reason: "Required property is missing".to_string(),
                }
                .into())
            }
        })
    }

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
                        return Err(DomainError::PropertyValidation {
                            property: property_schema.formatted_label.clone(),
                            reason: format!(
                                "Invalid value '{}', expected one of {:?}",
                                value, options
                            ),
                        }
                        .into());
                    }
                }
                Ok(())
            }
            _ => Err(DomainError::PropertyValidation {
                property: property_schema.formatted_label.clone(),
                reason: format!(
                    "Incorrect property type, expected {:?}, found {}",
                    property_schema.property_type, property_data_value
                ),
            }
            .into()),
        }
    }
}
