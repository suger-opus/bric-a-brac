use crate::error::ApiError;
use anyhow::Context;
use axum::http::Uri;
use bric_a_brac_protos::knowledge::{
    knowledge_service_client::KnowledgeServiceClient, InsertEdgeRequest, InsertNodeRequest,
    LoadGraphRequest, LoadGraphResponse, PropertyValue,
};
use std::collections::HashMap;
use tonic::Request;

#[derive(Clone)]
pub struct KnowledgeClient {
    client: KnowledgeServiceClient<tonic::transport::Channel>,
}

impl KnowledgeClient {
    pub async fn connect(addr: Uri) -> anyhow::Result<Self> {
        let client = KnowledgeServiceClient::connect(addr)
            .await
            .context("Failed to connect to KnowledgeServiceClient")?;
        Ok(Self { client })
    }

    // #[tracing::instrument(
    //     skip(self, properties),
    //     fields(
    //         grpc_method = "insert_node",
    //         graph_id = %graph_id,
    //         label = %label,
    //         property_count = properties.len()
    //     )
    // )]
    pub async fn insert_node(
        &self,
        graph_id: String,
        label: String,
        properties: HashMap<String, PropertyValue>,
    ) -> Result<String, ApiError> {
        tracing::debug!("Calling Knowledge service insert_node");

        let request = Request::new(InsertNodeRequest {
            graph_id,
            label,
            properties,
        });

        let response = self.client.clone().insert_node(request).await?;

        tracing::debug!("Knowledge service insert_node completed");
        Ok(response.into_inner().node_id)
    }

    pub async fn insert_edge(
        &self,
        from_id: String,
        to_id: String,
        label: String,
        properties: HashMap<String, PropertyValue>,
    ) -> Result<String, ApiError> {
        tracing::debug!("Calling Knowledge service insert_edge");

        let request = Request::new(InsertEdgeRequest {
            from_id,
            to_id,
            label,
            properties,
        });

        let response = self.client.clone().insert_edge(request).await?;

        tracing::debug!("Knowledge service insert_edge completed");
        Ok(response.into_inner().edge_id)
    }

    pub async fn load_graph(&self, graph_id: String) -> Result<LoadGraphResponse, ApiError> {
        tracing::debug!("Calling Knowledge service load_graph");

        let request = Request::new(LoadGraphRequest { graph_id });

        let response = self.client.clone().load_graph(request).await?;

        tracing::debug!("Knowledge service load_graph completed");
        Ok(response.into_inner())
    }
}
