use crate::{
    domain::models::{
        self, CreateEdgeData, CreateNodeData, EdgeDataId, GraphId, NodeDataId, PropertiesData,
        PropertyData,
    },
    infrastructure::config::KnowledgeServerConfig,
    presentation::error::{AppError, DomainError, InfraError, ResultExt},
};
use bric_a_brac_protos::knowledge::{
    knowledge_client::KnowledgeClient as KnowledgeGrpcClient, property_value, EdgeData, GraphData,
    InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest, NodeData, PropertyValue,
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

impl KnowledgeClient {
    pub fn new(config: KnowledgeServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn insert_node(
        &self,
        graph_id: GraphId,
        formatted_label: String,
        create_node_data: CreateNodeData,
    ) -> Result<models::NodeData, AppError> {
        match self
            .try_insert_node(graph_id, formatted_label.clone(), create_node_data.clone())
            .await
        {
            Ok(node) => Ok(node),
            Err(e) => {
                if e.is_grpc_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_insert_node(graph_id, formatted_label, create_node_data)
                        .await
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn insert_edge(
        &self,
        formatted_label: String,
        create_edge_data: CreateEdgeData,
    ) -> Result<models::EdgeData, AppError> {
        match self
            .try_insert_edge(formatted_label.clone(), create_edge_data.clone())
            .await
        {
            Ok(edge) => Ok(edge),
            Err(e) => {
                if e.is_grpc_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_insert_edge(formatted_label, create_edge_data)
                        .await
                } else {
                    Err(e)
                }
            }
        }
    }

    pub async fn load_graph(&self, graph_id: GraphId) -> Result<models::GraphData, AppError> {
        match self.try_load_graph(graph_id).await {
            Ok(graph) => Ok(graph),
            Err(e) => {
                if e.is_grpc_connection_error() {
                    tracing::warn!("Connection error detected, reconnecting: {}", e);
                    self.reset_connection();
                    self.try_load_graph(graph_id).await
                } else {
                    Err(e)
                }
            }
        }
    }

    async fn try_insert_node(
        &self,
        graph_id: GraphId,
        formatted_label: String,
        create_node_data: CreateNodeData,
    ) -> Result<models::NodeData, AppError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertNodeRequest {
            node_data_id: NodeDataId::new().to_string(),
            graph_id: graph_id.to_string(),
            formatted_label,
            properties: create_node_data.properties.into(),
        });

        let response = client
            .insert_node(request)
            .await
            .context("Failed to insert node in Knowledge service")?;

        response.into_inner().try_into()
    }

    async fn try_insert_edge(
        &self,
        formatted_label: String,
        create_edge_data: CreateEdgeData,
    ) -> Result<models::EdgeData, AppError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(InsertEdgeRequest {
            edge_data_id: EdgeDataId::new().to_string(),
            from_node_data_id: create_edge_data.from_node_data_id.to_string(),
            to_node_data_id: create_edge_data.to_node_data_id.to_string(),
            formatted_label,
            properties: create_edge_data.properties.into(),
        });

        let response = client
            .insert_edge(request)
            .await
            .context("Failed to insert edge in Knowledge service")?;

        response.into_inner().try_into()
    }

    async fn try_load_graph(&self, graph_id: GraphId) -> Result<models::GraphData, AppError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;

        let request = Request::new(LoadGraphRequest {
            graph_id: graph_id.to_string(),
        });
        let response = client
            .load_graph(request)
            .await
            .context("Failed to load graph from Knowledge service")?;

        response.into_inner().try_into()
    }

    fn reset_connection(&self) {
        if let Ok(mut client_lock) = self.client.lock() {
            *client_lock = None;
            tracing::info!("Reset Knowledge service connection");
        }
    }

    async fn ensure_connection(&self) -> Result<(), AppError> {
        {
            let client_lock = self.client.lock().map_err(|e| {
                AppError::Infra(InfraError::MutexPoisoned {
                    message: format!("Failed to acquire lock: {}", e),
                })
            })?;

            if client_lock.is_some() {
                return Ok(());
            }
        } // Lock dropped here - Holding mutex guards across .await is an anti-pattern

        tracing::info!("Connecting to Knowledge service at {}", self.config.url());

        let client = KnowledgeGrpcClient::connect(self.config.url().clone())
            .await
            .map_err(|e| {
                AppError::Infra(InfraError::GrpcService {
                    service: "knowledge".to_string(),
                    message: format!("Failed to connect to Knowledge service: {}", e),
                    source: None,
                })
            })?;

        {
            let mut client_lock = self.client.lock().map_err(|e| {
                AppError::Infra(InfraError::MutexPoisoned {
                    message: format!("Failed to acquire lock: {}", e),
                })
            })?;

            *client_lock = Some(client);
        }

        tracing::info!(
            "Successfully connected to Knowledge service at {}",
            self.config.url()
        );

        Ok(())
    }

    fn clone_client(&self) -> Result<KnowledgeGrpcClient<tonic::transport::Channel>, AppError> {
        let client_lock = self.client.lock().map_err(|e| {
            AppError::Infra(InfraError::MutexPoisoned {
                message: format!("Failed to acquire lock: {}", e),
            })
        })?;

        client_lock
            .as_ref()
            .ok_or_else(|| {
                AppError::Infra(InfraError::ClientNotConnected {
                    message: "Client not connected".to_string(),
                })
            })
            .map(|client| client.clone()) // Clone the client to avoid holding the lock across await
    }
}

impl TryFrom<GraphData> for models::GraphData {
    type Error = AppError;

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
    type Error = AppError;

    fn try_from(node_data: NodeData) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: NodeDataId::from_str(&node_data.node_data_id)?,
            formatted_label: node_data.formatted_label,
            properties: node_data.properties.try_into()?,
        })
    }
}

impl TryFrom<EdgeData> for models::EdgeData {
    type Error = AppError;

    fn try_from(edge_data: EdgeData) -> Result<Self, Self::Error> {
        Ok(Self {
            edge_data_id: EdgeDataId::from_str(&edge_data.edge_data_id)?,
            formatted_label: edge_data.formatted_label,
            from_node_data_id: NodeDataId::from_str(&edge_data.from_node_data_id)?,
            to_node_data_id: NodeDataId::from_str(&edge_data.to_node_data_id)?,
            properties: edge_data.properties.try_into()?,
        })
    }
}

impl TryFrom<HashMap<String, PropertyValue>> for PropertiesData {
    type Error = AppError;

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
            .map(|(formatted_label, value)| (formatted_label, value.into()))
            .collect()
    }
}

impl TryFrom<PropertyValue> for PropertyData {
    type Error = AppError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value.value {
            Some(property_value::Value::StringValue(s)) => Ok(PropertyData::String(s)),
            Some(property_value::Value::NumberValue(n)) => Ok(PropertyData::Number(n)),
            Some(property_value::Value::BoolValue(b)) => Ok(PropertyData::Boolean(b)),
            None => Err(DomainError::InvalidSchema {
                reason: "Property value is missing - PropertyValue does not contain a value"
                    .to_string(),
            }
            .into()),
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
