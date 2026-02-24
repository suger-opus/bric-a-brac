use crate::{
    domain::models::{
        self, CreateEdgeData, CreateNodeData, EdgeDataId, GraphId, NodeDataId, PropertiesData,
        PropertyData,
    },
    infrastructure::config::KnowledgeServerConfig,
    presentation::errors::GrpcClientError,
};
use axum::http::Uri;
use bric_a_brac_protos::{
    knowledge::{
        knowledge_client::KnowledgeClient as KnowledgeGrpcClient, property_value, EdgeData,
        GraphData, InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest, NodeData, PropertyValue,
    },
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use std::{
    collections::HashMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tonic::Request;

#[derive(Clone)]
pub struct KnowledgeClient {
    config: KnowledgeServerConfig,
    client: Arc<Mutex<Option<KnowledgeGrpcClient<tonic::transport::Channel>>>>,
}

#[tonic::async_trait]
impl GrpcClient for KnowledgeClient {
    type Client = KnowledgeGrpcClient<tonic::transport::Channel>;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>> {
        &self.client
    }

    fn service_kind(&self) -> GrpcServiceKind {
        GrpcServiceKind::Knowledge
    }

    fn url(&self) -> &Uri {
        self.config.url()
    }

    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error> {
        KnowledgeGrpcClient::connect(self.url().clone()).await
    }
}

impl KnowledgeClient {
    pub fn new(config: KnowledgeServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    #[tracing::instrument(level = "debug", skip(self, graph_id, create_node_data))]
    pub async fn insert_node(
        &self,
        graph_id: GraphId,
        create_node_data: CreateNodeData,
    ) -> Result<models::NodeData, GrpcClientError> {
        tracing::debug!(graph_id = ?graph_id, key = ?create_node_data.key);

        match self
            .try_insert_node(graph_id, create_node_data.clone())
            .await
        {
            Ok(node) => Ok(node),
            Err(e) => {
                if e.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_insert_node(graph_id, create_node_data).await
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self, create_edge_data))]
    pub async fn insert_edge(
        &self,
        create_edge_data: CreateEdgeData,
    ) -> Result<models::EdgeData, GrpcClientError> {
        tracing::debug!(key = ?create_edge_data.key, from_node_data_id = ?create_edge_data.from_node_data_id, to_node_data_id = ?create_edge_data.to_node_data_id);

        match self.try_insert_edge(create_edge_data.clone()).await {
            Ok(edge) => Ok(edge),
            Err(e) => {
                if e.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_insert_edge(create_edge_data).await
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tracing::instrument(level = "debug", skip(self, graph_id))]
    pub async fn load_graph(
        &self,
        graph_id: GraphId,
    ) -> Result<models::GraphData, GrpcClientError> {
        tracing::debug!(graph_id = ?graph_id);

        match self.try_load_graph(graph_id).await {
            Ok(graph) => Ok(graph),
            Err(e) => {
                if e.is_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_load_graph(graph_id).await
                } else {
                    Err(e)
                }
            }
        }
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id, create_node_data))]
    async fn try_insert_node(
        &self,
        graph_id: GraphId,
        create_node_data: CreateNodeData,
    ) -> Result<models::NodeData, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertNodeRequest {
            node_data_id: NodeDataId::new().to_string(),
            graph_id: graph_id.to_string(),
            key: create_node_data.key,
            properties: create_node_data.properties.into(),
        });

        let response =
            client
                .insert_node(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to insert node in Knowledge service".to_string(),
                    source: err,
                })?;

        models::NodeData::try_from(response.into_inner())
    }

    #[tracing::instrument(level = "trace", skip(self, create_edge_data))]
    async fn try_insert_edge(
        &self,
        create_edge_data: CreateEdgeData,
    ) -> Result<models::EdgeData, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertEdgeRequest {
            edge_data_id: EdgeDataId::new().to_string(),
            from_node_data_id: create_edge_data.from_node_data_id.to_string(),
            to_node_data_id: create_edge_data.to_node_data_id.to_string(),
            key: create_edge_data.key,
            properties: create_edge_data.properties.into(),
        });

        let response =
            client
                .insert_edge(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to insert edge in Knowledge service".to_string(),
                    source: err,
                })?;

        response.into_inner().try_into()
    }

    #[tracing::instrument(level = "trace", skip(self, graph_id))]
    async fn try_load_graph(
        &self,
        graph_id: GraphId,
    ) -> Result<models::GraphData, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(LoadGraphRequest {
            graph_id: graph_id.to_string(),
        });
        let response =
            client
                .load_graph(request)
                .await
                .map_err(|err| BaseGrpcClientError::Request {
                    service: GrpcServiceKind::Knowledge,
                    message: "Failed to load graph from Knowledge service".to_string(),
                    source: err,
                })?;

        response.into_inner().try_into()
    }
}

impl TryFrom<GraphData> for models::GraphData {
    type Error = GrpcClientError;

    fn try_from(graph_data: GraphData) -> Result<Self, Self::Error> {
        let nodes = graph_data
            .nodes
            .into_iter()
            .map(models::NodeData::try_from)
            .collect::<Result<_, Self::Error>>()?;
        let edges = graph_data
            .edges
            .into_iter()
            .map(models::EdgeData::try_from)
            .collect::<Result<_, Self::Error>>()?;

        Ok(Self { nodes, edges })
    }
}

impl TryFrom<NodeData> for models::NodeData {
    type Error = GrpcClientError;

    fn try_from(node_data: NodeData) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: NodeDataId::from_str(&node_data.node_data_id).map_err(|err| {
                GrpcClientError::DomainUuidConversion {
                    service: GrpcServiceKind::Knowledge,
                    source: err,
                }
            })?,
            key: node_data.key,
            properties: node_data.properties.try_into()?,
        })
    }
}

impl TryFrom<EdgeData> for models::EdgeData {
    type Error = GrpcClientError;

    fn try_from(edge_data: EdgeData) -> Result<Self, Self::Error> {
        Ok(Self {
            edge_data_id: EdgeDataId::from_str(&edge_data.edge_data_id).map_err(|err| {
                GrpcClientError::DomainUuidConversion {
                    service: GrpcServiceKind::Knowledge,
                    source: err,
                }
            })?,
            key: edge_data.key,
            from_node_data_id: NodeDataId::from_str(&edge_data.from_node_data_id).map_err(
                |err| GrpcClientError::DomainUuidConversion {
                    service: GrpcServiceKind::Knowledge,
                    source: err,
                },
            )?,
            to_node_data_id: NodeDataId::from_str(&edge_data.to_node_data_id).map_err(|err| {
                GrpcClientError::DomainUuidConversion {
                    service: GrpcServiceKind::Knowledge,
                    source: err,
                }
            })?,
            properties: edge_data.properties.try_into()?,
        })
    }
}

impl TryFrom<HashMap<String, PropertyValue>> for PropertiesData {
    type Error = GrpcClientError;

    fn try_from(values: HashMap<String, PropertyValue>) -> Result<Self, Self::Error> {
        let data = values
            .into_iter()
            .map(|(k, v)| Ok((k, v.try_into()?)))
            .collect::<Result<_, Self::Error>>()?;

        Ok(PropertiesData(data))
    }
}

impl From<PropertiesData> for HashMap<String, PropertyValue> {
    fn from(data: PropertiesData) -> Self {
        data.0
            .into_iter()
            .map(|(key, value)| (key, value.into()))
            .collect()
    }
}

impl TryFrom<PropertyValue> for PropertyData {
    type Error = GrpcClientError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value.value {
            Some(property_value::Value::StringValue(s)) => Ok(PropertyData::String(s)),
            Some(property_value::Value::NumberValue(n)) => Ok(PropertyData::Number(n)),
            Some(property_value::Value::BoolValue(b)) => Ok(PropertyData::Boolean(b)),
            None => Err(GrpcClientError::DomainConversion {
                service: GrpcServiceKind::Knowledge,
                reason: "Property value is missing - PropertyValue does not contain a value"
                    .to_string(),
            }),
        }
    }
}

impl From<PropertyData> for PropertyValue {
    fn from(data: PropertyData) -> Self {
        match data {
            PropertyData::String(s) => PropertyValue {
                value: Some(property_value::Value::StringValue(s)),
            },
            PropertyData::Number(n) => PropertyValue {
                value: Some(property_value::Value::NumberValue(n)),
            },
            PropertyData::Boolean(b) => PropertyValue {
                value: Some(property_value::Value::BoolValue(b)),
            },
        }
    }
}
