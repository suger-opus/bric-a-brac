use crate::DtosConversionError;
use bric_a_brac_protos::common::{property_value_proto, PropertyValueProto};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{PartialSchema, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
// TODO: validation (length of string, ...)
pub struct PropertiesDataDto {
    pub values: HashMap<String, serde_json::Value>,
}

impl PartialSchema for PropertiesDataDto {
    fn schema() -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
        utoipa::openapi::schema::Schema::default().into()
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
        let mut values = HashMap::new();
        for (k, v) in properties {
            let json_value = match v.value {
                Some(property_value_proto::Value::BoolValue(b)) => serde_json::Value::Bool(b),
                Some(property_value_proto::Value::NumberValue(n)) => {
                    serde_json::Value::Number(serde_json::Number::from_f64(n).ok_or_else(|| {
                        DtosConversionError::Number {
                            property_name: k.clone(),
                            value: n,
                        }
                    })?)
                }
                Some(property_value_proto::Value::StringValue(s)) => serde_json::Value::String(s),
                None => Err(DtosConversionError::NoPropertyValue {
                    property_name: k.clone(),
                })?,
            };
            values.insert(k, json_value);
        }
        Ok(PropertiesDataDto { values })
    }
}

impl From<PropertiesDataDto> for HashMap<String, PropertyValueProto> {
    fn from(dto: PropertiesDataDto) -> Self {
        dto.values
            .into_iter()
            .map(|(k, v)| {
                (
                    k,
                    match v {
                        serde_json::Value::Bool(b) => PropertyValueProto {
                            value: Some(property_value_proto::Value::BoolValue(b)),
                        },
                        serde_json::Value::Number(n) => PropertyValueProto {
                            value: Some(property_value_proto::Value::NumberValue(
                                n.as_f64().unwrap_or(0.0), // TODO: return error ? (problem: chain try_from)
                            )),
                        },
                        serde_json::Value::String(s) => PropertyValueProto {
                            value: Some(property_value_proto::Value::StringValue(s)),
                        },
                        _ => PropertyValueProto { value: None }, // TODO: should definitely return an error instead of silently putting None
                    },
                )
            })
            .collect()
    }
}
