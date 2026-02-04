use crate::clients::knowledge_client::KnowledgeClient;
use crate::dtos::graph_dto::{
    ReqPostEdgeSchema, ReqPostGraph, ReqPostNodeSchema, ResEdgeSchema, ResGraphData,
    ResGraphMetadata, ResGraphSchema, ResNodeSchema,
};
use crate::error::ApiError;
use crate::models::{access_model::Role, graph_model::GraphId, user_model::UserId};
use crate::repositories::{access_repository::AccessRepository, graph_repository::GraphRepository};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Clone)]
pub struct GraphService {
    pool: PgPool,
    repository: GraphRepository,
    access_repository: AccessRepository,
    knowledge_client: KnowledgeClient,
}

impl GraphService {
    pub fn new(
        pool: &PgPool,
        repository: &GraphRepository,
        access_repository: &AccessRepository,
        knowledge_client: &KnowledgeClient,
    ) -> Self {
        GraphService {
            pool: pool.clone(),
            repository: repository.clone(),
            access_repository: access_repository.clone(),
            knowledge_client: knowledge_client.clone(),
        }
    }

    pub async fn get_all_metadata(
        &self,
        user_id: UserId,
    ) -> Result<Vec<ResGraphMetadata>, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self.repository.get_all_metadata(&mut txn, user_id).await?;
        txn.commit().await?;
        Ok(graphs)
    }

    pub async fn get_metadata(
        &self,
        user_id: UserId,
        graph_id: GraphId,
    ) -> Result<ResGraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph_id)
            .await?;
        txn.commit().await?;
        Ok(graph)
    }

    pub async fn get_schema(&self, graph_id: GraphId) -> Result<ResGraphSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let schema = self.repository.get_schema(&mut txn, graph_id).await?;
        txn.commit().await?;
        Ok(schema)
    }

    pub async fn post(
        &self,
        user_id: UserId,
        new_graph: &ReqPostGraph,
    ) -> Result<ResGraphMetadata, ApiError> {
        let mut txn = self.pool.begin().await?;
        let graph = self.repository.post(&mut txn, new_graph).await?;
        self.access_repository
            .post(&mut txn, graph.graph_id, user_id, Role::Owner)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, user_id, graph.graph_id)
            .await?;
        txn.commit().await?;

        Ok(graph)
    }

    pub async fn post_node_schema(
        &self,
        graph_id: GraphId,
        new_node_schema: &ReqPostNodeSchema,
    ) -> Result<ResNodeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let node_schema = self
            .repository
            .post_node_schema(&mut txn, graph_id, new_node_schema)
            .await?;
        let properties = self
            .repository
            .post_properties(
                &mut txn,
                Some(node_schema.node_schema_id),
                None,
                &new_node_schema.properties,
            )
            .await?;
        txn.commit().await?;
        Ok(ResNodeSchema {
            node_schema,
            properties,
        })
    }

    pub async fn post_edge_schema(
        &self,
        graph_id: GraphId,
        new_edge_schema: &ReqPostEdgeSchema,
    ) -> Result<ResEdgeSchema, ApiError> {
        let mut txn = self.pool.begin().await?;
        let edge_schema = self
            .repository
            .post_edge_schema(&mut txn, graph_id, new_edge_schema)
            .await?;
        let properties = self
            .repository
            .post_properties(
                &mut txn,
                None,
                Some(edge_schema.edge_schema_id),
                &new_edge_schema.properties,
            )
            .await?;
        txn.commit().await?;
        Ok(ResEdgeSchema {
            edge_schema,
            properties,
        })
    }

    pub async fn get_data(&self, graph_id: GraphId) -> Result<ResGraphData, ApiError> {
        let response = self
            .knowledge_client
            .load_graph(graph_id.to_string())
            .await?;
        
        use bric_a_brac_protos::knowledge::property_value::Value;
        
        let convert_properties = |props: std::collections::HashMap<String, bric_a_brac_protos::knowledge::PropertyValue>| {
            props.into_iter().filter_map(|(k, v)| {
                v.value.map(|val| {
                    let json_val = match val {
                        Value::StringValue(s) => serde_json::Value::String(s),
                        Value::NumberValue(n) => serde_json::json!(n),
                        Value::BoolValue(b) => serde_json::Value::Bool(b),
                    };
                    (k, json_val)
                })
            }).collect()
        };
        
        Ok(ResGraphData {
            nodes: response.nodes.into_iter().map(|n| crate::dtos::graph_dto::ResNode {
                id: n.id,
                label: n.label,
                properties: convert_properties(n.properties),
            }).collect(),
            edges: response.edges.into_iter().map(|e| crate::dtos::graph_dto::ResEdge {
                id: e.id,
                label: e.label,
                from_id: e.from_id,
                to_id: e.to_id,
                properties: convert_properties(e.properties),
            }).collect(),
        })
    }

    pub async fn post_node_data(
        &self,
        graph_id: GraphId,
        label: String,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<String, ApiError> {
        use bric_a_brac_protos::knowledge::{PropertyValue, property_value};
        
        let proto_properties: HashMap<String, PropertyValue> = properties
            .into_iter()
            .map(|(k, v)| {
                let pv = match v {
                    serde_json::Value::String(s) => PropertyValue {
                        value: Some(property_value::Value::StringValue(s)),
                    },
                    serde_json::Value::Number(n) => PropertyValue {
                        value: Some(property_value::Value::NumberValue(
                            n.as_f64().unwrap_or(0.0),
                        )),
                    },
                    serde_json::Value::Bool(b) => PropertyValue {
                        value: Some(property_value::Value::BoolValue(b)),
                    },
                    _ => PropertyValue { value: None },
                };
                (k, pv)
            })
            .collect();

        let node_id = self
            .knowledge_client
            .insert_node(graph_id.to_string(), label, proto_properties)
            .await?;
        Ok(node_id)
    }

    pub async fn post_edge_data(
        &self,
        from_id: String,
        to_id: String,
        label: String,
        properties: HashMap<String, serde_json::Value>,
    ) -> Result<String, ApiError> {
        use bric_a_brac_protos::knowledge::{PropertyValue, property_value};
        
        let proto_properties: HashMap<String, PropertyValue> = properties
            .into_iter()
            .map(|(k, v)| {
                let pv = match v {
                    serde_json::Value::String(s) => PropertyValue {
                        value: Some(property_value::Value::StringValue(s)),
                    },
                    serde_json::Value::Number(n) => PropertyValue {
                        value: Some(property_value::Value::NumberValue(
                            n.as_f64().unwrap_or(0.0),
                        )),
                    },
                    serde_json::Value::Bool(b) => PropertyValue {
                        value: Some(property_value::Value::BoolValue(b)),
                    },
                    _ => PropertyValue { value: None },
                };
                (k, pv)
            })
            .collect();

        let edge_id = self
            .knowledge_client
            .insert_edge(from_id, to_id, label, proto_properties)
            .await?;
        Ok(edge_id)
    }
}
