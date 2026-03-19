use crate::{infrastructure::config::MetadataServerConfig, presentation::errors::GrpcClientError};
use bric_a_brac_protos::{
    common::{EdgeSchemaProto, GraphSchemaProto, NodeSchemaProto},
    metadata::{
        metadata_client::MetadataClient as MetadataGrpcClient, AppendSessionMessagesRequest,
        CloseSessionRequest, CreateEdgeSchemaRequest, CreateNodeSchemaRequest,
        CreateSessionRequest, GetSchemaRequest, GetSessionMessagesRequest, GetSessionRequest,
        NewSessionMessageProto, SessionMessageProto, SessionProto,
    },
    BaseGrpcClientError, GrpcClient, GrpcServiceKind,
};
use http::Uri;
use std::sync::{Arc, Mutex};
use tonic::Request;

#[derive(Clone)]
pub struct MetadataClient {
    config: MetadataServerConfig,
    client: Arc<Mutex<Option<MetadataGrpcClient<tonic::transport::Channel>>>>,
}

#[tonic::async_trait]
impl GrpcClient for MetadataClient {
    type Client = MetadataGrpcClient<tonic::transport::Channel>;

    fn client(&self) -> &Arc<Mutex<Option<Self::Client>>> {
        &self.client
    }

    fn service_kind(&self) -> GrpcServiceKind {
        GrpcServiceKind::Metadata
    }

    fn url(&self) -> &Uri {
        self.config.url()
    }

    async fn connect(&self) -> Result<Self::Client, tonic::transport::Error> {
        MetadataGrpcClient::connect(self.url().clone()).await
    }
}

impl MetadataClient {
    pub fn new(config: MetadataServerConfig) -> Self {
        Self {
            config,
            client: Arc::new(Mutex::new(None)),
        }
    }

    // --- Session RPCs ---

    pub async fn create_session(
        &self,
        graph_id: &str,
        user_id: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        match self.try_create_session(graph_id, user_id).await {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_create_session(graph_id, user_id).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_create_session(
        &self,
        graph_id: &str,
        user_id: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .create_session(Request::new(CreateSessionRequest {
                graph_id: graph_id.to_owned(),
                user_id: user_id.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to create session".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn get_session(
        &self,
        session_id: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        match self.try_get_session(session_id).await {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_get_session(session_id).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_get_session(
        &self,
        session_id: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .get_session(Request::new(GetSessionRequest {
                session_id: session_id.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to get session".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn close_session(
        &self,
        session_id: &str,
        status: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        match self.try_close_session(session_id, status).await {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_close_session(session_id, status).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_close_session(
        &self,
        session_id: &str,
        status: &str,
    ) -> Result<SessionProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .close_session(Request::new(CloseSessionRequest {
                session_id: session_id.to_owned(),
                status: status.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to close session".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn get_session_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<SessionMessageProto>, GrpcClientError> {
        match self.try_get_session_messages(session_id).await {
            Ok(m) => Ok(m),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_get_session_messages(session_id).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_get_session_messages(
        &self,
        session_id: &str,
    ) -> Result<Vec<SessionMessageProto>, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .get_session_messages(Request::new(GetSessionMessagesRequest {
                session_id: session_id.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to get session messages".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner().messages)
    }

    pub async fn append_session_messages(
        &self,
        session_id: &str,
        messages: Vec<NewSessionMessageProto>,
    ) -> Result<(), GrpcClientError> {
        match self
            .try_append_session_messages(session_id, messages.clone())
            .await
        {
            Ok(()) => Ok(()),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_append_session_messages(session_id, messages).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_append_session_messages(
        &self,
        session_id: &str,
        messages: Vec<NewSessionMessageProto>,
    ) -> Result<(), GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        client
            .append_session_messages(Request::new(AppendSessionMessagesRequest {
                session_id: session_id.to_owned(),
                messages,
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to append session messages".to_owned(),
                source: err,
            })?;
        Ok(())
    }

    // --- Schema RPCs ---

    pub async fn create_node_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<NodeSchemaProto, GrpcClientError> {
        match self
            .try_create_node_schema(graph_id, label, description)
            .await
        {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_create_node_schema(graph_id, label, description)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_create_node_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<NodeSchemaProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .create_node_schema(Request::new(CreateNodeSchemaRequest {
                graph_id: graph_id.to_owned(),
                label: label.to_owned(),
                description: description.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to create node schema".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn create_edge_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<EdgeSchemaProto, GrpcClientError> {
        match self
            .try_create_edge_schema(graph_id, label, description)
            .await
        {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_create_edge_schema(graph_id, label, description)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_create_edge_schema(
        &self,
        graph_id: &str,
        label: &str,
        description: &str,
    ) -> Result<EdgeSchemaProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .create_edge_schema(Request::new(CreateEdgeSchemaRequest {
                graph_id: graph_id.to_owned(),
                label: label.to_owned(),
                description: description.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to create edge schema".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }

    pub async fn get_schema(
        &self,
        graph_id: &str,
    ) -> Result<GraphSchemaProto, GrpcClientError> {
        match self.try_get_schema(graph_id).await {
            Ok(s) => Ok(s),
            Err(err) if err.is_connection_error() => {
                self.reset_connection();
                self.try_get_schema(graph_id).await
            }
            Err(err) => Err(err),
        }
    }

    async fn try_get_schema(
        &self,
        graph_id: &str,
    ) -> Result<GraphSchemaProto, GrpcClientError> {
        self.ensure_connection().await?;
        let mut client = self.clone_client()?;
        let response = client
            .get_schema(Request::new(GetSchemaRequest {
                graph_id: graph_id.to_owned(),
            }))
            .await
            .map_err(|err| BaseGrpcClientError::Request {
                service: GrpcServiceKind::Metadata,
                message: "Failed to get schema".to_owned(),
                source: err,
            })?;
        Ok(response.into_inner())
    }
}
