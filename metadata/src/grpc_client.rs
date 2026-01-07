use std::collections::HashMap;
use tonic::Request;

use bric_a_brac_protos::knowledge::knowledge_service_client::KnowledgeServiceClient;
use bric_a_brac_protos::knowledge::{
    GraphDataResponse, InsertEdgeRequest, InsertNodeRequest, PropertyValue, SearchRequest,
};

#[derive(Clone)]
pub struct KnowledgeClient {
    client: KnowledgeServiceClient<tonic::transport::Channel>,
}

impl KnowledgeClient {
    pub async fn connect(addr: String) -> anyhow::Result<Self> {
        let client = KnowledgeServiceClient::connect(addr).await?;
        Ok(Self { client })
    }

    pub async fn insert_node(
        &mut self,
        graph_id: String,
        label: String,
        properties: HashMap<String, PropertyValue>,
    ) -> anyhow::Result<GraphDataResponse> {
        let request = Request::new(InsertNodeRequest {
            graph_id,
            label,
            properties,
        });

        let response = self.client.insert_node(request).await?;
        Ok(response.into_inner())
    }

    pub async fn insert_edge(
        &mut self,
        from_id: String,
        to_id: String,
        label: String,
        properties: HashMap<String, PropertyValue>,
    ) -> anyhow::Result<GraphDataResponse> {
        let request = Request::new(InsertEdgeRequest {
            from_id,
            to_id,
            label,
            properties,
        });

        let response = self.client.insert_edge(request).await?;
        Ok(response.into_inner())
    }

    pub async fn search(
        &mut self,
        graph_id: Option<String>,
        node_label: Option<String>,
        node_properties: HashMap<String, PropertyValue>,
        edge_label: Option<String>,
        edge_properties: HashMap<String, PropertyValue>,
        include_edges: bool,
    ) -> anyhow::Result<GraphDataResponse> {
        let request = Request::new(SearchRequest {
            graph_id,
            node_label,
            node_properties,
            edge_label,
            edge_properties,
            include_edges,
        });

        let response = self.client.search(request).await?;
        Ok(response.into_inner())
    }
}
