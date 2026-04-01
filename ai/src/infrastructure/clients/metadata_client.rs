use crate::infrastructure::{InfraError, MetadataServerConfig};
use bric_a_brac_dtos::{
    CreateSessionMessageDto, DescriptionDto, EdgeSchemaDto, GraphIdDto, GraphSchemaDto, LabelDto, NodeSchemaDto, SessionDocumentDto, SessionDocumentIdDto, SessionDto, SessionIdDto, SessionMessageDto
};
use bric_a_brac_protos::{
    metadata::{
        metadata_client::MetadataClient as MetadataGrpcClient, AppendSessionMessagesRequest,
        CreateEdgeSchemaRequest, CreateNodeSchemaRequest, GetSchemaRequest,
        GetSessionDocumentRequest, GetSessionMessagesRequest, GetSessionRequest,
    },
    with_retry,
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

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session",
        skip(self, session_id),
        err
    )]
    pub async fn get_session(&self, session_id: SessionIdDto) -> Result<SessionDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetSessionRequest {
                session_id: session_id.to_string(),
            });
            async move { c.get_session(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    // #[tracing::instrument(
    //     level = "debug",
    //     name = "metadata_client.close_session",
    //     skip(self, session_id, status),
    //     err
    // )]
    // pub async fn close_session(
    //     &self,
    //     session_id: SessionIdDto,
    //     status: SessionStatusDto,
    // ) -> Result<SessionDto, InfraError> {
    //     let data = with_retry(|| {
    //         let mut c = self.client.clone();
    //         let req = Request::new(CloseSessionRequest {
    //             session_id: session_id.to_string(),
    //             status: status.into(),
    //         });
    //         async move { c.close_session(req).await }
    //     })
    //     .await?;

    //     Ok(data.try_into()?)
    // }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session_messages",
        skip(self, session_id),
        err
    )]
    pub async fn get_session_messages(
        &self,
        session_id: SessionIdDto,
    ) -> Result<Vec<SessionMessageDto>, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetSessionMessagesRequest {
                session_id: session_id.to_string(),
            });
            async move { c.get_session_messages(req).await }
        })
        .await?;

        Ok(data
            .messages
            .into_iter()
            .map(SessionMessageDto::try_from)
            .collect::<Result<_, _>>()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.append_session_messages",
        skip(self, session_id, messages),
        err
    )]
    pub async fn append_session_messages(
        &self,
        session_id: SessionIdDto,
        messages: Vec<CreateSessionMessageDto>,
    ) -> Result<(), InfraError> {
        with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(AppendSessionMessagesRequest {
                session_id: session_id.to_string(),
                messages: messages
                    .clone()
                    .into_iter()
                    .map(CreateSessionMessageDto::into)
                    .collect(),
            });
            async move { c.append_session_messages(req).await }
        })
        .await?;

        Ok(())
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_session_document",
        skip(self, document_id),
        err
    )]
    pub async fn get_session_document(
        &self,
        document_id: SessionDocumentIdDto,
    ) -> Result<SessionDocumentDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetSessionDocumentRequest {
                document_id: document_id.to_string(),
            });
            async move { c.get_session_document(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.create_node_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_node_schema(
        &self,
        graph_id: GraphIdDto,
        label: LabelDto,
        description: DescriptionDto,
    ) -> Result<NodeSchemaDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(CreateNodeSchemaRequest {
                graph_id: graph_id.to_string(),
                label: label.to_string(),
                description: description.to_string(),
            });
            async move { c.create_node_schema(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.create_edge_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_edge_schema(
        &self,
        graph_id: GraphIdDto,
        label: LabelDto,
        description: DescriptionDto,
    ) -> Result<EdgeSchemaDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(CreateEdgeSchemaRequest {
                graph_id: graph_id.to_string(),
                label: label.to_string(),
                description: description.to_string(),
            });
            async move { c.create_edge_schema(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }

    #[tracing::instrument(
        level = "debug",
        name = "metadata_client.get_schema",
        skip(self, graph_id),
        err
    )]
    pub async fn get_schema(&self, graph_id: GraphIdDto) -> Result<GraphSchemaDto, InfraError> {
        let data = with_retry(|| {
            let mut c = self.client.clone();
            let req = Request::new(GetSchemaRequest {
                graph_id: graph_id.to_string(),
            });
            async move { c.get_schema(req).await }
        })
        .await?;

        Ok(data.try_into()?)
    }
}
