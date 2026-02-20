use crate::domain::models::{PropertiesData, PropertyData};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{PartialSchema, ToSchema};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertiesDataDto {
    values: HashMap<String, serde_json::Value>,
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

impl From<PropertiesDataDto> for PropertiesData {
    fn from(properties: PropertiesDataDto) -> Self {
        PropertiesData(
            properties
                .values
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            serde_json::Value::String(s) => PropertyData::String(s),
                            serde_json::Value::Number(n) => {
                                PropertyData::Number(n.as_f64().unwrap_or(0.0))
                            } // todo: should return an error ?
                            serde_json::Value::Bool(b) => PropertyData::Boolean(b),
                            _ => PropertyData::String(v.to_string()), // todo: should return an error ?
                        },
                    )
                })
                .collect(),
        )
    }
}

impl From<PropertiesData> for PropertiesDataDto {
    fn from(data: PropertiesData) -> Self {
        Self {
            values: data
                .0
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            PropertyData::String(s) => serde_json::Value::String(s),
                            PropertyData::Number(n) => serde_json::Value::Number(
                                serde_json::Number::from_f64(n)
                                    .unwrap_or(serde_json::Number::from(0)), // todo: should return an error ?
                            ),
                            PropertyData::Boolean(b) => serde_json::Value::Bool(b),
                        },
                    )
                })
                .collect(),
        }
    }
}
