use crate::{
    config::KnowledgeServerConfig,
    error::{ApiError, ApiErrorContent},
    models::{
        self, EdgeDataId, GraphId, NewEdgeData, NewNodeData, NodeDataId, PropertiesData,
        PropertyData,
    },
};
use anyhow::Context;
use bric_a_brac_protos::knowledge::{
    knowledge_service_client::KnowledgeServiceClient, property_value, EdgeData, GraphData,
    InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest, NodeData, PropertyValue,
};
use std::{collections::HashMap, str::FromStr};
use tonic::Request;

#[derive(Clone)]
pub struct KnowledgeClient {
    client: KnowledgeServiceClient<tonic::transport::Channel>,
}

impl KnowledgeClient {
    pub async fn connect(config: &KnowledgeServerConfig) -> anyhow::Result<Self> {
        let client = KnowledgeServiceClient::connect(config.url().clone())
            .await
            .context("Failed to connect to Knowledge service")?;

        Ok(Self { client })
    }

    pub async fn insert_node(
        &self,
        graph_id: GraphId,
        formatted_label: String,
        new_node_data: NewNodeData,
    ) -> Result<models::NodeData, ApiError> {
        let request = Request::new(InsertNodeRequest {
            node_data_id: NodeDataId::new().to_string(),
            graph_id: graph_id.to_string(),
            formatted_label,
            properties: new_node_data.properties.into(),
        });
        let response = self.client.clone().insert_node(request).await?;

        response.into_inner().try_into()
    }

    pub async fn insert_edge(
        &self,
        formatted_label: String,
        new_edge_data: NewEdgeData,
    ) -> Result<models::EdgeData, ApiError> {
        let request = Request::new(InsertEdgeRequest {
            edge_data_id: EdgeDataId::new().to_string(),
            from_node_data_id: new_edge_data.from_node_data_id.to_string(),
            to_node_data_id: new_edge_data.to_node_data_id.to_string(),
            formatted_label,
            properties: new_edge_data.properties.into(),
        });
        let response = self.client.clone().insert_edge(request).await?;

        response.into_inner().try_into()
    }

    pub async fn load_graph(&self, graph_id: GraphId) -> Result<models::GraphData, ApiError> {
        let request = Request::new(LoadGraphRequest {
            graph_id: graph_id.to_string(),
        });
        let response = self.client.clone().load_graph(request).await?;

        response.into_inner().try_into()
    }
}

impl TryFrom<GraphData> for models::GraphData {
    type Error = ApiError;

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
    type Error = ApiError;

    fn try_from(node_data: NodeData) -> Result<Self, Self::Error> {
        Ok(Self {
            node_data_id: NodeDataId::from_str(&node_data.node_data_id)?,
            formatted_label: node_data.formatted_label,
            properties: node_data.properties.try_into()?,
        })
    }
}

impl TryFrom<EdgeData> for models::EdgeData {
    type Error = ApiError;

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
    type Error = ApiError;

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
    type Error = ApiError;

    fn try_from(value: PropertyValue) -> Result<Self, Self::Error> {
        match value.value {
            Some(property_value::Value::StringValue(s)) => Ok(PropertyData::String(s)),
            Some(property_value::Value::NumberValue(n)) => Ok(PropertyData::Number(n)),
            Some(property_value::Value::BoolValue(b)) => Ok(PropertyData::Boolean(b)),
            None => Err(ApiError::ConversionKnowledgeError(ApiErrorContent {
                message: "Property value is missing".to_string(),
                details: "The PropertyValue does not contain a value".to_string(),
            })),
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
