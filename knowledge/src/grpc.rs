use crate::config::Config;
use crate::db::connect_to_database;
use crate::models::{EdgeData, GraphData, NodeData};
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
    Edge as ProtoEdge, GraphDataResponse, InsertEdgeRequest, InsertNodeRequest, Node as ProtoNode,
    PropertyValue, SearchRequest,
};

pub struct KnowledgeServer {
    graph: Arc<Graph>,
}

impl KnowledgeServer {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let graph = connect_to_database(&config).await?;
        Ok(Self {
            graph: Arc::new(graph),
        })
    }

    pub fn into_service(self) -> KnowledgeServiceServer<Self> {
        KnowledgeServiceServer::new(self)
    }
}

// Helper type wrapper for property conversion
// Helper type wrappers for conversions (satisfies orphan rule)
struct BoltProperties(HashMap<String, BoltType>);
struct PropertyValueWrapper<'a>(&'a PropertyValue);

// Trait implementations for type conversions
impl<'a> From<PropertyValueWrapper<'a>> for Option<BoltType> {
    fn from(wrapper: PropertyValueWrapper<'a>) -> Self {
        wrapper.0.value.as_ref().map(|v| match v {
            bric_a_brac_protos::knowledge::property_value::Value::StringValue(s) => {
                BoltType::String(s.clone().into())
            }
            bric_a_brac_protos::knowledge::property_value::Value::IntValue(i) => {
                BoltType::Integer((*i).into())
            }
            bric_a_brac_protos::knowledge::property_value::Value::FloatValue(f) => {
                BoltType::Float(neo4rs::BoltFloat::new(*f))
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
            properties_json: serde_json::to_string(&node.properties).unwrap_or_default(),
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
            properties_json: serde_json::to_string(&edge.properties).unwrap_or_default(),
        }
    }
}

impl From<GraphData> for GraphDataResponse {
    fn from(data: GraphData) -> Self {
        GraphDataResponse {
            nodes: data.nodes.into_iter().map(ProtoNode::from).collect(),
            edges: data.edges.into_iter().map(ProtoEdge::from).collect(),
        }
    }
}

#[tonic::async_trait]
impl KnowledgeService for KnowledgeServer {
    async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<GraphDataResponse>, Status> {
        let req = request.into_inner();
        let graph_id = Uuid::parse_str(&req.graph_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid graph_id: {}", e)))?;

        let properties: HashMap<String, BoltType> = BoltProperties::from(req.properties).into();

        let result = service::insert_node(&self.graph, graph_id, &req.label, properties)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert node: {}", e)))?;

        Ok(Response::new(result.into()))
    }

    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<GraphDataResponse>, Status> {
        let req = request.into_inner();
        let from_id = Uuid::parse_str(&req.from_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid from_id: {}", e)))?;
        let to_id = Uuid::parse_str(&req.to_id)
            .map_err(|e| Status::invalid_argument(format!("Invalid to_id: {}", e)))?;

        let properties: HashMap<String, BoltType> = BoltProperties::from(req.properties).into();

        let result = service::insert_edge(&self.graph, from_id, to_id, &req.label, properties)
            .await
            .map_err(|e| Status::internal(format!("Failed to insert edge: {}", e)))?;

        Ok(Response::new(result.into()))
    }

    async fn search(
        &self,
        request: Request<SearchRequest>,
    ) -> Result<Response<GraphDataResponse>, Status> {
        let req = request.into_inner();

        let graph_id = req.graph_id.and_then(|id| Uuid::parse_str(&id).ok());

        let node_properties: HashMap<String, BoltType> =
            BoltProperties::from(req.node_properties).into();
        let edge_properties: HashMap<String, BoltType> =
            BoltProperties::from(req.edge_properties).into();

        let result = service::search(
            &self.graph,
            graph_id,
            req.node_label.as_deref(),
            node_properties,
            req.edge_label.as_deref(),
            edge_properties,
            req.include_edges,
        )
        .await
        .map_err(|e| Status::internal(format!("Search failed: {}", e)))?;

        Ok(Response::new(result.into()))
    }
}
