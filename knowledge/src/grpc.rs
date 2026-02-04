use crate::config::Config;
use crate::models::{EdgeData, NodeData};
use crate::service;
use neo4rs::{BoltType, Graph};
use std::collections::HashMap;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use bric_a_brac_protos::knowledge::knowledge_service_server::{
    KnowledgeService, KnowledgeServiceServer,
};
use bric_a_brac_protos::knowledge::{
    property_value, Edge as ProtoEdge, InsertEdgeRequest, InsertEdgeResponse, InsertNodeRequest,
    InsertNodeResponse, LoadGraphRequest, LoadGraphResponse, Node as ProtoNode, PropertyValue,
};

pub struct KnowledgeServer {
    graph: Arc<Graph>,
}

impl KnowledgeServer {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let graph = config.knowledge_db.connect().await?;
        Ok(Self {
            graph: Arc::new(graph),
        })
    }

    pub fn into_service(self) -> KnowledgeServiceServer<Self> {
        KnowledgeServiceServer::new(self)
    }
}

// Helper type wrappers for conversions
struct BoltProperties(HashMap<String, BoltType>);
struct PropertyValueWrapper<'a>(&'a PropertyValue);

impl<'a> From<PropertyValueWrapper<'a>> for Option<BoltType> {
    fn from(wrapper: PropertyValueWrapper<'a>) -> Self {
        wrapper.0.value.as_ref().map(|v| match v {
            bric_a_brac_protos::knowledge::property_value::Value::StringValue(s) => {
                BoltType::String(s.clone().into())
            }
            bric_a_brac_protos::knowledge::property_value::Value::NumberValue(n) => {
                BoltType::Float(neo4rs::BoltFloat::new(*n))
            }
            bric_a_brac_protos::knowledge::property_value::Value::BoolValue(b) => {
                BoltType::Boolean(neo4rs::BoltBoolean::new(*b))
            }
        })
    }
}

impl From<HashMap<String, PropertyValue>> for BoltProperties {
    fn from(props: HashMap<String, PropertyValue>) -> Self {
        BoltProperties(
            props
                .into_iter()
                .filter_map(|(k, v)| {
                    Option::<BoltType>::from(PropertyValueWrapper(&v)).map(|bolt| (k, bolt))
                })
                .collect(),
        )
    }
}

impl From<BoltProperties> for HashMap<String, BoltType> {
    fn from(props: BoltProperties) -> Self {
        props.0
    }
}

impl From<NodeData> for ProtoNode {
    fn from(node: NodeData) -> Self {
        ProtoNode {
            id: node.id,
            label: node.label,
            properties: node
                .properties
                .into_iter()
                .filter_map(|(k, v)| {
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
                        _ => return None,
                    };
                    Some((k, pv))
                })
                .collect(),
        }
    }
}

impl From<EdgeData> for ProtoEdge {
    fn from(edge: EdgeData) -> Self {
        ProtoEdge {
            id: edge.id,
            label: edge.label,
            from_id: edge.from_id,
            to_id: edge.to_id,
            properties: edge
                .properties
                .into_iter()
                .filter_map(|(k, v)| {
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
                        _ => return None,
                    };
                    Some((k, pv))
                })
                .collect(),
        }
    }
}

#[tonic::async_trait]
impl KnowledgeService for KnowledgeServer {
    async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<InsertNodeResponse>, Status> {
        let req = request.into_inner();
        let graph_id = Uuid::parse_str(&req.graph_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid graph_id: {}", e)))?;

        let properties: HashMap<String, BoltType> = BoltProperties::from(req.properties).into();

        let node_id = service::insert_node(&self.graph, graph_id, &req.label, properties)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert node: {}", e)))?;

        Ok(Response::new(InsertNodeResponse {
            node_id: node_id.to_string(),
        }))
    }

    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<InsertEdgeResponse>, Status> {
        let req = request.into_inner();
        let from_id = Uuid::parse_str(&req.from_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid from_id: {}", e)))?;
        let to_id = Uuid::parse_str(&req.to_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid to_id: {}", e)))?;

        let properties: HashMap<String, BoltType> = BoltProperties::from(req.properties).into();

        let edge_id = service::insert_edge(&self.graph, from_id, to_id, &req.label, properties)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert edge: {}", e)))?;

        Ok(Response::new(InsertEdgeResponse {
            edge_id: edge_id.to_string(),
        }))
    }

    async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<LoadGraphResponse>, Status> {
        let req = request.into_inner();
        let graph_id = Uuid::parse_str(&req.graph_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid graph_id: {}", e)))?;

        let graph_data = service::load_graph(&self.graph, graph_id)
            .await
            .map_err(|e| Status::internal(format!("Failed to load graph: {}", e)))?;

        Ok(Response::new(LoadGraphResponse {
            nodes: graph_data.nodes.into_iter().map(ProtoNode::from).collect(),
            edges: graph_data.edges.into_iter().map(ProtoEdge::from).collect(),
        }))
    }
}
