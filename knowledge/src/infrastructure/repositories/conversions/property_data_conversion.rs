use crate::{
    domain::{PropertiesDataModel, PropertyValueModel},
    infrastructure::DatabaseError,
};
use neo4rs::{BoltBoolean, BoltString, BoltType};
use std::collections::HashMap;

impl TryFrom<HashMap<BoltString, BoltType>> for PropertiesDataModel {
    type Error = DatabaseError;

    fn try_from(properties: HashMap<BoltString, BoltType>) -> Result<Self, Self::Error> {
        let mut values = HashMap::new();
        for (k, v) in properties {
            let value = match v {
                BoltType::String(s) => PropertyValueModel::String(s.to_string()),
                BoltType::Float(f) => PropertyValueModel::Number(f.value),
                BoltType::Boolean(b) => PropertyValueModel::Bool(b.value),
                _ => {
                    return Err(DatabaseError::CorruptedProperty {
                        label: k.to_string(),
                        value: v,
                    })
                }
            };
            values.insert(k.to_string(), value);
        }

        Ok(Self { values })
    }
}

impl From<PropertiesDataModel> for HashMap<BoltString, BoltType> {
    fn from(properties: PropertiesDataModel) -> Self {
        properties
            .values
            .into_iter()
            .map(|(k, v)| {
                let bolt_value = match v {
                    PropertyValueModel::String(s) => BoltType::String(s.into()),
                    PropertyValueModel::Number(f) => BoltType::Float(neo4rs::BoltFloat::new(f)),
                    PropertyValueModel::Bool(b) => BoltType::Boolean(BoltBoolean::new(b)),
                };
                (BoltString::from(k), bolt_value)
            })
            .collect()
    }
}
