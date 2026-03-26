use crate::application::{
    dtos::{CreateSessionMessageDto, SessionDocumentIdDto, SessionIdDto},
    services::{DocumentService, GraphService, SessionService},
};
use bric_a_brac_dtos::GraphIdDto;
use bric_a_brac_protos::common::{EdgeSchemaProto, GraphSchemaProto, NodeSchemaProto};
use bric_a_brac_protos::metadata::{
    metadata_server::Metadata, AppendSessionMessagesRequest, AppendSessionMessagesResponse,
    CloseSessionRequest, CreateEdgeSchemaRequest, CreateNodeSchemaRequest, GetSchemaRequest,
    GetSessionDocumentRequest, GetSessionMessagesRequest, GetSessionMessagesResponse,
    GetSessionRequest, SessionDocumentProto, SessionProto,
};
use tonic::{Request, Response, Status};

#[allow(clippy::struct_field_names)]
pub struct MetadataGrpcService {
    session_service: SessionService,
    graph_service: GraphService,
    document_service: DocumentService,
}

impl MetadataGrpcService {
    pub const fn new(
        session_service: SessionService,
        graph_service: GraphService,
        document_service: DocumentService,
    ) -> Self {
        Self {
            session_service,
            graph_service,
            document_service,
        }
    }
}

#[tonic::async_trait]
impl Metadata for MetadataGrpcService {
    #[tracing::instrument(level = "trace", name = "grpc.get_session", skip(self, request), err)]
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto = req
            .session_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid session_id: {err:?}")))?;

        let session = self
            .session_service
            .get_session(session_id)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.close_session", skip(self, request), err)]
    async fn close_session(
        &self,
        request: Request<CloseSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto = req
            .session_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid session_id: {err:?}")))?;

        let session = self
            .session_service
            .close_session(session_id, &req.status)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.get_session_messages",
        skip(self, request),
        err
    )]
    async fn get_session_messages(
        &self,
        request: Request<GetSessionMessagesRequest>,
    ) -> Result<Response<GetSessionMessagesResponse>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto = req
            .session_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid session_id: {err:?}")))?;

        let messages = self
            .session_service
            .get_messages(session_id)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(GetSessionMessagesResponse {
            messages: messages.into_iter().map(From::from).collect(),
        }))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.append_session_messages",
        skip(self, request),
        err
    )]
    async fn append_session_messages(
        &self,
        request: Request<AppendSessionMessagesRequest>,
    ) -> Result<Response<AppendSessionMessagesResponse>, Status> {
        let req = request.into_inner();
        let session_id: SessionIdDto = req
            .session_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid session_id: {err:?}")))?;

        let messages: Vec<CreateSessionMessageDto> = req
            .messages
            .into_iter()
            .map(|m| CreateSessionMessageDto {
                role: m.role,
                content: m.content,
                tool_calls: m.tool_calls,
                tool_call_id: m.tool_call_id,
                document_id: m.document_id,
                chunk_index: m.chunk_index,
            })
            .collect();

        self.session_service
            .append_messages(session_id, messages)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(AppendSessionMessagesResponse {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.create_node_schema",
        skip(self, request),
        err
    )]
    async fn create_node_schema(
        &self,
        request: Request<CreateNodeSchemaRequest>,
    ) -> Result<Response<NodeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req
            .graph_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid graph_id: {err:?}")))?;

        let schema = self
            .graph_service
            .create_node_schema(graph_id, req.label, req.description)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.create_edge_schema",
        skip(self, request),
        err
    )]
    async fn create_edge_schema(
        &self,
        request: Request<CreateEdgeSchemaRequest>,
    ) -> Result<Response<EdgeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req
            .graph_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid graph_id: {err:?}")))?;

        let schema = self
            .graph_service
            .create_edge_schema(graph_id, req.label, req.description)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_schema", skip(self, request), err)]
    async fn get_schema(
        &self,
        request: Request<GetSchemaRequest>,
    ) -> Result<Response<GraphSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id: GraphIdDto = req
            .graph_id
            .try_into()
            .map_err(|err| Status::invalid_argument(format!("Invalid graph_id: {err:?}")))?;

        let schema = self
            .graph_service
            .get_schema(graph_id)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.get_session_document",
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
            .map_err(|err| Status::invalid_argument(format!("Invalid document_id: {err:?}")))?;

        let document = self
            .document_service
            .get_document(document_id)
            .await
            .map_err(|err| Status::internal(format!("{err:?}")))?;

        Ok(Response::new(document.into()))
    }
}
