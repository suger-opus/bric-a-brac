use crate::infrastructure::{config::MetadataServerConfig, errors::GrpcClientError};
use bric_a_brac_protos::{
    common::{EdgeSchemaProto, GraphSchemaProto, NodeSchemaProto},
    metadata::{
        metadata_client::MetadataClient as MetadataGrpcClient, AppendSessionMessagesRequest,
        CloseSessionRequest, CreateEdgeSchemaRequest, CreateNodeSchemaRequest, GetSchemaRequest,
        GetSessionDocumentRequest, GetSessionMessagesRequest, GetSessionRequest,
        NewSessionMessageProto, SessionDocumentProto, SessionMessageProto, SessionProto,
    },
    with_retry, GrpcServiceKind,
};
use tonic::transport::Channel;
use tonic::Request;

#[derive(Clone)]
pub struct MetadataClient {
    client: MetadataGrpcClient<Channel>,
}

impl MetadataClient {
    pub fn new(config: &MetadataServerConfig) -> anyhow::Result<Self> {
        let channel =
            tonic::transport::Endpoint::from_shared(config.url().to_string())?.connect_lazy();
        Ok(Self {
            client: MetadataGrpcClient::new(channel),
        })
    }

    // --- Session RPCs ---
    // TODO: might need a session DTO
    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session",
        skip(self, session_id),
        err
    )]
    pub async fn get_session(&self, session_id: &str) -> Result<SessionProto, GrpcClientError> {
        let client = self.client.clone();
        let session_id = session_id.to_owned();
        with_retry(GrpcServiceKind::Metadata, "Failed to get session", || {
            let mut c = client.clone();
            let req = Request::new(GetSessionRequest {
                session_id: session_id.clone(),
            });
            async move { c.get_session(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.close_session",
        skip(self, session_id, status),
        err
    )]
    pub async fn close_session(
        &self,
        session_id: &str,
        status: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        let client = self.client.clone();
        let session_id = session_id.to_owned();
        let status = status.to_owned();
        with_retry(GrpcServiceKind::Metadata, "Failed to close session", || {
            let mut c = client.clone();
            let req = Request::new(CloseSessionRequest {
                session_id: session_id.clone(),
                status: status.clone(),
            });
            async move { c.close_session(req).await }
        })
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session_messages",
        skip(self, session_id),
        err
    )]
    pub async fn get_session_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<SessionMessageProto>, GrpcClientError> {
        let client = self.client.clone();
        let session_id = session_id.to_owned();
        let response = with_retry(
            GrpcServiceKind::Metadata,
            "Failed to get session messages",
            || {
                let mut c = client.clone();
                let req = Request::new(GetSessionMessagesRequest {
                    session_id: session_id.clone(),
                });
                async move { c.get_session_messages(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(response.messages)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.append_session_messages",
        skip(self, session_id, messages),
        err
    )]
    pub async fn append_session_messages(
        &self,
        session_id: &str,
        messages: Vec<NewSessionMessageProto>,
    ) -> Result<(), GrpcClientError> {
        let client = self.client.clone();
        let session_id = session_id.to_owned();
        let messages = messages.clone();
        with_retry(
            GrpcServiceKind::Metadata,
            "Failed to append session messages",
            || {
                let mut c = client.clone();
                let req = Request::new(AppendSessionMessagesRequest {
                    session_id: session_id.clone(),
                    messages: messages.clone(),
                });
                async move { c.append_session_messages(req).await }
            },
        )
        .await
        .map_err(GrpcClientError::from)?;
        Ok(())
    }

    // --- Document RPCs ---

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session_document",
        skip(self, document_id),
        err
    )]
    pub async fn get_session_document(
        &self,
        document_id: &str,
    ) -> Result<SessionDocumentProto, GrpcClientError> {
        let client = self.client.clone();
        let document_id = document_id.to_owned();
        with_retry(
            GrpcServiceKind::Metadata,
            "Failed to get session document",
            || {
                let mut c = client.clone();
                let req = Request::new(GetSessionDocumentRequest {
                    document_id: document_id.clone(),
                });
                async move { c.get_session_document(req).await }
            },
        )
        .await
        .map_err(Into::into)
    }

    // --- Schema RPCs ---

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.create_node_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_node_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<NodeSchemaProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let label = label.to_owned();
        let description = description.to_owned();
        with_retry(
            GrpcServiceKind::Metadata,
            "Failed to create node schema",
            || {
                let mut c = client.clone();
                let req = Request::new(CreateNodeSchemaRequest {
                    graph_id: graph_id.clone(),
                    label: label.clone(),
                    description: description.clone(),
                });
                async move { c.create_node_schema(req).await }
            },
        )
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.create_edge_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_edge_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<EdgeSchemaProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        let label = label.to_owned();
        let description = description.to_owned();
        with_retry(
            GrpcServiceKind::Metadata,
            "Failed to create edge schema",
            || {
                let mut c = client.clone();
                let req = Request::new(CreateEdgeSchemaRequest {
                    graph_id: graph_id.clone(),
                    label: label.clone(),
                    description: description.clone(),
                });
                async move { c.create_edge_schema(req).await }
            },
        )
        .await
        .map_err(Into::into)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_schema",
        skip(self, graph_id),
        err
    )]
    pub async fn get_schema(&self, graph_id: &str) -> Result<GraphSchemaProto, GrpcClientError> {
        let client = self.client.clone();
        let graph_id = graph_id.to_owned();
        with_retry(GrpcServiceKind::Metadata, "Failed to get schema", || {
            let mut c = client.clone();
            let req = Request::new(GetSchemaRequest {
                graph_id: graph_id.clone(),
            });
            async move { c.get_schema(req).await }
        })
        .await
        .map_err(Into::into)
    }
}
