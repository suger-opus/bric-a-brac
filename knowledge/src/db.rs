use crate::config::Config;
use crate::models::{EdgeData, NodeData};
use neo4rs::{BoltType, ConfigBuilder, Graph};
use std::collections::HashMap;

// Helper trait for BoltType conversion
pub trait BoltTypeExt {
    fn to_json_value(&self) -> serde_json::Value;
}

impl BoltTypeExt for BoltType {
    fn to_json_value(&self) -> serde_json::Value {
        match self {
            BoltType::String(s) => serde_json::Value::String(s.to_string()),
            BoltType::Integer(i) => {
                if let Ok(val) = format!("{:?}", i).parse::<i64>() {
                    serde_json::Value::Number(val.into())
                } else {
                    serde_json::Value::Null
                }
            }
            BoltType::Float(f) => {
                if let Ok(val) = format!("{:?}", f).parse::<f64>() {
                    if let Some(num) = serde_json::Number::from_f64(val) {
                        serde_json::Value::Number(num)
                    } else {
                        serde_json::Value::Null
                    }
                } else {
                    serde_json::Value::Null
                }
            }
            BoltType::Boolean(b) => serde_json::Value::Bool(format!("{:?}", b) == "true"),
            BoltType::List(list) => {
                serde_json::Value::Array(list.iter().map(|item| item.to_json_value()).collect())
            }
            BoltType::Map(map) => {
                let mut obj = serde_json::Map::new();
                for (k, v) in map.value.iter() {
                    obj.insert(k.to_string(), v.to_json_value());
                }
                serde_json::Value::Object(obj)
            }
            BoltType::Null(_) => serde_json::Value::Null,
            _ => serde_json::Value::String(format!("{:?}", self)),
        }
    }
}

impl From<&neo4rs::Node> for NodeData {
    fn from(node: &neo4rs::Node) -> Self {
        let properties: HashMap<String, serde_json::Value> = node
            .keys()
            .iter()
            .filter_map(|key| {
                node.get::<BoltType>(key)
                    .ok()
                    .map(|val| (key.to_string(), val.to_json_value()))
            })
            .collect();

        let id = properties
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let label = node.labels().first().unwrap_or(&"").to_string();

        NodeData {
            id,
            label,
            properties,
        }
    }
}

impl From<&neo4rs::Relation> for EdgeData {
    fn from(edge: &neo4rs::Relation) -> Self {
        let properties: HashMap<String, serde_json::Value> = edge
            .keys()
            .iter()
            .filter_map(|key| {
                edge.get::<BoltType>(key)
                    .ok()
                    .map(|val| (key.to_string(), val.to_json_value()))
            })
            .collect();

        let id = properties
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        EdgeData {
            id,
            label: edge.typ().to_string(),
            from_id: edge.start_node_id().to_string(),
            to_id: edge.end_node_id().to_string(),
            properties,
        }
    }
}

pub async fn connect_to_database(cfg: &Config) -> anyhow::Result<Graph> {
    let uri = cfg.database_uri();

    let config = ConfigBuilder::default()
        .uri(&uri)
        .user(cfg.database_user.as_str())
        .password(cfg.database_password.as_str())
        .db(cfg.database_name.as_str())
        .fetch_size(500)
        .max_connections(10)
        .build()?;

    let graph = Graph::connect(config).await?;

    // Verify connection by running a simple query with timeout
    tokio::time::timeout(
        std::time::Duration::from_secs(5),
        graph.run(neo4rs::query("RETURN 1")),
    )
    .await
    .map_err(|_| anyhow::anyhow!("Database connection timeout - check credentials and network"))?
    .map_err(|e| anyhow::anyhow!("Failed to verify database connection: {}", e))?;

    Ok(graph)
}
