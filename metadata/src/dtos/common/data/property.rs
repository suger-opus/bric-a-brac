use crate::models::PropertiesData;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PropertiesDto {
    values: HashMap<String, serde_json::Value>,
}

impl From<PropertiesDto> for PropertiesData {
    fn from(properties: PropertiesDto) -> Self {
        PropertiesData(
            properties
                .values
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            serde_json::Value::String(s) => crate::models::PropertyData::String(s),
                            serde_json::Value::Number(n) => {
                                // todo: should return an error ?
                                crate::models::PropertyData::Number(n.as_f64().unwrap_or(0.0))
                            }
                            serde_json::Value::Bool(b) => crate::models::PropertyData::Boolean(b),
                            _ => crate::models::PropertyData::String(v.to_string()), // todo: should return an error ?
                        },
                    )
                })
                .collect(),
        )
    }
}

impl From<PropertiesData> for PropertiesDto {
    fn from(data: PropertiesData) -> Self {
        Self {
            values: data
                .0
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        match v {
                            crate::models::PropertyData::String(s) => serde_json::Value::String(s),
                            crate::models::PropertyData::Number(n) => serde_json::Value::Number(
                                serde_json::Number::from_f64(n)
                                    .unwrap_or(serde_json::Number::from(0)), // todo: should return an error ?
                            ),
                            crate::models::PropertyData::Boolean(b) => serde_json::Value::Bool(b),
                        },
                    )
                })
                .collect(),
        }
    }
}
