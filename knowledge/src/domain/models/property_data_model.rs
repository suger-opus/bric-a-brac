use crate::infrastructure::errors::DatabaseError;
use neo4rs::{BoltBoolean, BoltString, BoltType};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize)]
pub struct PropertiesDataModel {
    pub values: HashMap<String, serde_json::Value>,
}

impl TryFrom<HashMap<BoltString, BoltType>> for PropertiesDataModel {
    type Error = DatabaseError;

    fn try_from(properties: HashMap<BoltString, BoltType>) -> Result<Self, Self::Error> {
        let mut values = HashMap::new();
        for (k, v) in properties {
            let value = match v {
                BoltType::String(s) => serde_json::Value::String(s.to_string()),
                BoltType::Float(f) => {
                    serde_json::Value::Number(serde_json::Number::from_f64(f.value).ok_or_else(
                        || DatabaseError::NumberConversion {
                            property_name: k.to_string(),
                            value: f.value.to_string(),
                        },
                    )?)
                }
                BoltType::Boolean(b) => serde_json::Value::Bool(b.value),
                _ => return Err(DatabaseError::UnsupportedBoltType { bolt_type: v }),
            };
            values.insert(k.to_string(), value);
        }

        Ok(PropertiesDataModel { values })
    }
}

impl TryFrom<PropertiesDataModel> for HashMap<BoltString, BoltType> {
    type Error = DatabaseError;

    fn try_from(properties: PropertiesDataModel) -> Result<Self, Self::Error> {
        properties
            .values
            .into_iter()
            .map(|(k, v)| {
                let bolt_value = match v {
                    serde_json::Value::String(s) => Ok(BoltType::String(s.clone().into())),
                    serde_json::Value::Number(n) => {
                        let f = n.as_f64().ok_or_else(|| DatabaseError::NumberConversion {
                            property_name: k.clone(),
                            value: n.to_string(),
                        })?;
                        Ok(BoltType::Float(neo4rs::BoltFloat::new(f)))
                    }
                    serde_json::Value::Bool(b) => {
                        Ok(BoltType::Boolean(BoltBoolean::new(b.clone())))
                    }
                    _ => Err(DatabaseError::UnsupportedPropertyValue {
                        value: v.to_string(),
                    }),
                }?;
                Ok((BoltString::from(k), bolt_value))
            })
            .collect()
    }
}
