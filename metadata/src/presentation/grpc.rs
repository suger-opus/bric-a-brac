use crate::{
    application::{GraphService, SessionService},
    presentation::error::PresentationError,
};
use bric_a_brac_dtos::{
    CreateSessionMessageDto, DescriptionDto, DtosConversionError, GraphIdDto, LabelDto,
    SessionDocumentIdDto, SessionIdDto,
};
use bric_a_brac_protos::{
    common::{
        EdgeSchemaProto, Empty, GraphSchemaProto, NodeSchemaProto, SessionDocumentProto,
        SessionProto,
    },
    metadata::{
        metadata_server::Metadata, AppendSessionMessagesRequest, CreateEdgeSchemaRequest,
        CreateNodeSchemaRequest, GetSchemaRequest, GetSessionDocumentRequest,
        GetSessionMessagesRequest, GetSessionMessagesResponse, GetSessionRequest,
    },
};
use tonic::{Request, Response, Status};
use validator::Validate;

#[allow(clippy::struct_field_names)]
pub struct MetadataGrpcService {
    session_service: SessionService,
    graph_service: GraphService,
}

impl MetadataGrpcService {
    pub const fn new(session_service: SessionService, graph_service: GraphService) -> Self {
        Self {
            session_service,
            graph_service,
        }
    }
}

#[tonic::async_trait]
impl Metadata for MetadataGrpcService {
    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.get_session",
        skip(self, request),
        err
    )]
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session = self
            .session_service
            .get_session_internal(req.session_id.try_into().map_err(PresentationError::from)?)
            .await?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.get_session_messages",
        skip(self, request),
        err
    )]
    async fn get_session_messages(
        &self,
        request: Request<GetSessionMessagesRequest>,
    ) -> Result<Response<GetSessionMessagesResponse>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto =
            req.session_id.try_into().map_err(PresentationError::from)?;
        let messages = self
            .session_service
            .get_messages_internal(session_id)
            .await?;

        Ok(Response::new(GetSessionMessagesResponse {
            messages: messages.into_iter().map(From::from).collect(),
        }))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.append_session_messages",
        skip(self, request),
        err
    )]
    async fn append_session_messages(
        &self,
        request: Request<AppendSessionMessagesRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto =
            req.session_id.try_into().map_err(PresentationError::from)?;
        let messages: Vec<CreateSessionMessageDto> = req
            .messages
            .into_iter()
            .map(|m| {
                Ok(CreateSessionMessageDto {
                    role: m.role.try_into()?,
                    content: m.content,
                    tool_calls: m.tool_calls,
                    tool_call_id: m.tool_call_id,
                    document_id: m.document_id.map(TryFrom::try_from).transpose()?,
                    document_name: m.document_name,
                    chunk_index: m.chunk_index,
                })
            })
            .collect::<Result<Vec<_>, DtosConversionError>>()
            .map_err(PresentationError::from)?;

        self.session_service
            .append_messages_internal(session_id, messages)
            .await?;

        Ok(Response::new(Empty {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.create_node_schema",
        skip(self, request),
        err
    )]
    async fn create_node_schema(
        &self,
        request: Request<CreateNodeSchemaRequest>,
    ) -> Result<Response<NodeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req.graph_id.try_into().map_err(PresentationError::from)?;
        let label: LabelDto = req.label.into();
        let description: DescriptionDto = req.description.into();
        label.validate().map_err(PresentationError::from)?;
        description.validate().map_err(PresentationError::from)?;
        let schema = self
            .graph_service
            .create_node_schema(graph_id, label, description)
            .await?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.create_edge_schema",
        skip(self, request),
        err
    )]
    async fn create_edge_schema(
        &self,
        request: Request<CreateEdgeSchemaRequest>,
    ) -> Result<Response<EdgeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req.graph_id.try_into().map_err(PresentationError::from)?;
        let label: LabelDto = req.label.into();
        let description: DescriptionDto = req.description.into();
        label.validate().map_err(PresentationError::from)?;
        description.validate().map_err(PresentationError::from)?;
        let schema = self
            .graph_service
            .create_edge_schema(graph_id, label, description)
            .await?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.get_schema",
        skip(self, request),
        err
    )]
    async fn get_schema(
        &self,
        request: Request<GetSchemaRequest>,
    ) -> Result<Response<GraphSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req.graph_id.try_into().map_err(PresentationError::from)?;
        let schema = self.graph_service.get_schema_internal(graph_id).await?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "metadata_service.get_session_document",
        skip(self, request),
        err
    )]
    async fn get_session_document(
        &self,
        request: Request<GetSessionDocumentRequest>,
    ) -> Result<Response<SessionDocumentProto>, Status> {
        let req = request.into_inner();
        let document_id: SessionDocumentIdDto = req
            .document_id
            .try_into()
            .map_err(PresentationError::from)?;
        let document = self.session_service.get_document_internal(document_id).await?;

        Ok(Response::new(document.into()))
    }
}
