use crate::{domain::PropertiesDataModel, infrastructure::DatabaseError};
use neo4rs::BoltType;
use std::collections::HashMap;
use uuid::Uuid;

pub trait ExtendElement {
    fn get(&self, key: &str) -> Option<BoltType>;

    fn keys(&self) -> Vec<&str>;

    fn extract_id(&self, id_key: &str) -> Result<Uuid, DatabaseError> {
        let bolt_id = self
            .get(id_key)
            .ok_or_else(|| DatabaseError::CorruptedIdState {
                label: id_key.to_owned(),
            })?;

        match bolt_id {
            BoltType::String(s) => {
                Uuid::parse_str(&s.to_string()).map_err(|err| DatabaseError::CorruptedId {
                    label: id_key.to_owned(),
                    value: s.to_string(),
                    source: Some(Box::new(err)),
                })
            }
            other => Err(DatabaseError::CorruptedId {
                label: id_key.to_owned(),
                value: format!("{other:?}"),
                source: None,
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
                    "node_data_id" | "graph_id" | "edge_data_id" | "embedding"
                )
            })
            .map(|key| {
                let value = self
                    .get(key)
                    .ok_or_else(|| DatabaseError::CorruptedPropertyState {
                        label: key.to_owned(),
                    })?;
                Ok((key.to_owned().into(), value))
            })
            .collect::<Result<HashMap<_, _>, DatabaseError>>()?;

        bolt_properties.try_into()
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
