use crate::models::{PropertyMetadata, PropertyType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyMetadataDto {
    pub options: Option<Vec<String>>,
}

impl From<PropertyMetadataDto> for PropertyMetadata {
    fn from(metadata: PropertyMetadataDto) -> Self {
        Self {
            options: metadata.options,
        }
    }
}

impl From<PropertyMetadata> for PropertyMetadataDto {
    fn from(metadata: PropertyMetadata) -> Self {
        Self {
            options: metadata.options,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyTypeDto {
    Number,
    String,
    Boolean,
    Select,
}

impl From<PropertyTypeDto> for PropertyType {
    fn from(property_type: PropertyTypeDto) -> Self {
        match property_type {
            PropertyTypeDto::Number => PropertyType::Number,
            PropertyTypeDto::String => PropertyType::String,
            PropertyTypeDto::Boolean => PropertyType::Boolean,
            PropertyTypeDto::Select => PropertyType::Select,
        }
    }
}

impl From<PropertyType> for PropertyTypeDto {
    fn from(property_type: PropertyType) -> Self {
        match property_type {
            PropertyType::Number => PropertyTypeDto::Number,
            PropertyType::String => PropertyTypeDto::String,
            PropertyType::Boolean => PropertyTypeDto::Boolean,
            PropertyType::Select => PropertyTypeDto::Select,
        }
    }
}
