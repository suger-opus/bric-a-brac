use crate::DtosConversionError;
use bric_a_brac_protos::common::{property_value_proto, PropertyValueProto};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{PartialSchema, ToSchema};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug, Clone, Serialize, Deserialize, derive_more::Display)]
#[serde(untagged)]
pub enum PropertyValueDto {
    Bool(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertiesDataDto {
    pub values: HashMap<String, PropertyValueDto>,
}

impl Validate for PropertiesDataDto {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();
        for (label, value) in &self.values {
            if let PropertyValueDto::String(s) = value {
                if s.len() > 1000 {
                    let mut err = ValidationError::new("string_too_long");
                    err.message = Some(
                        format!("Property '{label}' string value exceeds maximum length of 1000")
                            .into(),
                    );
                    errors.add("values", err);
                }
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl PartialSchema for PropertiesDataDto {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        use utoipa::openapi::schema::{
            AdditionalProperties, ObjectBuilder, OneOfBuilder, SchemaType, Type,
        };

        let value_schema = OneOfBuilder::new()
            .item(
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(Type::String))
                    .max_length(Some(1000))
                    .build(),
            )
            .item(
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(Type::Number))
                    .build(),
            )
            .item(
                ObjectBuilder::new()
                    .schema_type(SchemaType::new(Type::Boolean))
                    .build(),
            )
            .build();

        ObjectBuilder::new()
            .additional_properties(Some(AdditionalProperties::RefOr(
                utoipa::openapi::schema::Schema::OneOf(value_schema).into(),
            )))
            .build()
            .into()
    }
}

impl ToSchema for PropertiesDataDto {
    fn name() -> std::borrow::Cow<'static, str> {
        std::borrow::Cow::Borrowed("PropertiesDataDto")
    }
}

impl TryFrom<HashMap<String, PropertyValueProto>> for PropertiesDataDto {
    type Error = DtosConversionError;

    fn try_from(properties: HashMap<String, PropertyValueProto>) -> Result<Self, Self::Error> {
        let values = properties
            .into_iter()
            .map(|(k, v)| {
                let value = match v.value {
                    Some(property_value_proto::Value::BoolValue(b)) => PropertyValueDto::Bool(b),
                    Some(property_value_proto::Value::NumberValue(n)) => {
                        PropertyValueDto::Number(n)
                    }
                    Some(property_value_proto::Value::StringValue(s)) => {
                        PropertyValueDto::String(s)
                    }
                    None => return Err(DtosConversionError::MissingPropertyValue { label: k }),
                };
                Ok((k, value))
            })
            .collect::<Result<_, _>>()?;
        Ok(Self { values })
    }
}

impl From<PropertiesDataDto> for HashMap<String, PropertyValueProto> {
    fn from(dto: PropertiesDataDto) -> Self {
        dto.values
            .into_iter()
            .map(|(k, v)| {
                let proto_value = match v {
                    PropertyValueDto::Bool(b) => property_value_proto::Value::BoolValue(b),
                    PropertyValueDto::Number(n) => property_value_proto::Value::NumberValue(n),
                    PropertyValueDto::String(s) => property_value_proto::Value::StringValue(s),
                };
                (
                    k,
                    PropertyValueProto {
                        value: Some(proto_value),
                    },
                )
            })
            .collect()
    }
}
