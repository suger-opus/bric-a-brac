use crate::config::KnowledgeServerConfig;
use crate::dtos::graph_dto::{PropertiesDto, ResEdgeData, ResGraphData, ResNodeData};
use crate::error::ApiError;
use crate::models::{
    edge_data_model::EdgeDataId, graph_model::GraphId, node_data_model::NodeDataId,
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
        properties: PropertiesDto,
    ) -> Result<NodeData, ApiError> {
        let request = Request::new(InsertNodeRequest {
            node_data_id: NodeDataId::new().to_string(),
            graph_id: graph_id.to_string(),
            formatted_label,
            properties: properties.into(),
        });
        let response = self.client.clone().insert_node(request).await?;

        Ok(response.into_inner())
    }

    pub async fn insert_edge(
        &self,
        from_node_data_id: NodeDataId,
        to_node_data_id: NodeDataId,
        formatted_label: String,
        properties: PropertiesDto,
    ) -> Result<EdgeData, ApiError> {
        let request = Request::new(InsertEdgeRequest {
            edge_data_id: EdgeDataId::new().to_string(),
            from_node_data_id: from_node_data_id.to_string(),
            to_node_data_id: to_node_data_id.to_string(),
            formatted_label,
            properties: properties.into(),
        });
        let response = self.client.clone().insert_edge(request).await?;

        Ok(response.into_inner())
    }

    pub async fn load_graph(&self, graph_id: GraphId) -> Result<GraphData, ApiError> {
        let request = Request::new(LoadGraphRequest {
            graph_id: graph_id.to_string(),
        });
        let response = self.client.clone().load_graph(request).await?;

        Ok(response.into_inner())
    }
}

impl From<GraphData> for ResGraphData {
    fn from(graph_data: GraphData) -> Self {
        let nodes = graph_data.nodes.into_iter().map(NodeData::into).collect();
        let edges = graph_data.edges.into_iter().map(EdgeData::into).collect();

        Self { nodes, edges }
    }
}

impl From<NodeData> for ResNodeData {
    fn from(node_data: NodeData) -> Self {
        Self {
            node_data_id: NodeDataId::from_str(&node_data.node_data_id).unwrap_or_default(),
            formatted_label: node_data.formatted_label,
            properties: node_data.properties.into(),
        }
    }
}

impl From<EdgeData> for ResEdgeData {
    fn from(node_data: EdgeData) -> Self {
        Self {
            edge_data_id: EdgeDataId::from_str(&node_data.edge_data_id).unwrap_or_default(),
            formatted_label: node_data.formatted_label,
            from_node_data_id: NodeDataId::from_str(&node_data.from_node_data_id)
                .unwrap_or_default(),
            to_node_data_id: NodeDataId::from_str(&node_data.to_node_data_id).unwrap_or_default(),
            properties: node_data.properties.into(),
        }
    }
}

impl From<HashMap<String, PropertyValue>> for PropertiesDto {
    fn from(properties: HashMap<String, PropertyValue>) -> Self {
        let json_properties = properties
            .into_iter()
            .filter_map(|(k, v)| {
                v.value.map(|val| {
                    let json_val = match val {
                        property_value::Value::StringValue(s) => serde_json::Value::String(s),
                        property_value::Value::NumberValue(n) => serde_json::Number::from_f64(n)
                            .map(serde_json::Value::Number)
                            .unwrap_or_default(),
                        property_value::Value::BoolValue(b) => serde_json::Value::Bool(b),
                    };
                    (k, json_val)
                })
            })
            .collect();

        PropertiesDto(json_properties)
    }
}

impl From<PropertiesDto> for HashMap<String, PropertyValue> {
    fn from(dto: PropertiesDto) -> HashMap<String, PropertyValue> {
        dto.0
            .into_iter()
            .map(|(formatted_label, value)| {
                let proto_value = match value {
                    serde_json::Value::String(s) => PropertyValue {
                        value: Some(property_value::Value::StringValue(s)),
                    },
                    serde_json::Value::Number(n) => PropertyValue {
                        value: Some(property_value::Value::NumberValue(
                            n.as_f64().unwrap_or_default(),
                        )),
                    },
                    serde_json::Value::Bool(b) => PropertyValue {
                        value: Some(property_value::Value::BoolValue(b)),
                    },
                    _ => PropertyValue { value: None },
                };
                (formatted_label, proto_value)
            })
            .collect()
    }
}
