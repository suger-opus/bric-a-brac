use std::collections::HashMap;

use crate::{domain::models::PropertiesDataModel, infrastructure::errors::DatabaseError};
use neo4rs::BoltType;

pub trait ExtendElement {
    fn get(&self, key: &str) -> Option<BoltType>;

    fn keys(&self) -> Vec<&str>;

    fn extract_id(&self, id_key: &str) -> Result<String, DatabaseError> {
        let bolt_id = self.get(id_key).ok_or_else(|| DatabaseError::MissingId {
            id: id_key.to_string(),
        })?;

        match bolt_id {
            BoltType::String(s) => Ok(s.to_string()),
            _ => Err(DatabaseError::WrongId {
                id: id_key.to_string(),
            }),
        }
    }

    fn collect_properties(&self) -> Result<PropertiesDataModel, DatabaseError> {
        let bolt_properties = self
            .keys()
            .into_iter()
            .filter(|key| {
                !matches!(
                    *key,
                    "node_data_id" | "graph_id" | "edge_data_id" | "embedding" | "session_id"
                )
            })
            .map(|key| {
                let value = self
                    .get(key)
                    .ok_or_else(|| DatabaseError::UnreachableProperty {
                        property_key: key.to_string(),
                    })?;
                Ok((key.to_string().into(), value))
            })
            .collect::<Result<HashMap<_, _>, DatabaseError>>()?;

        Ok(bolt_properties.try_into()?)
    }
}

impl ExtendElement for neo4rs::Node {
    fn get(&self, key: &str) -> Option<BoltType> {
        self.get::<BoltType>(key).ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.keys()
    }
}

impl ExtendElement for neo4rs::Relation {
    fn get(&self, key: &str) -> Option<BoltType> {
        self.get::<BoltType>(key).ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.keys()
    }
}

impl ExtendElement for neo4rs::UnboundedRelation {
    fn get(&self, key: &str) -> Option<BoltType> {
        self.get::<BoltType>(key).ok()
    }

    fn keys(&self) -> Vec<&str> {
        self.keys()
    }
}
