use crate::application::{
    dtos::{
        CreateSessionDto, CreateSessionMessageDto, SessionDocumentIdDto,
        SessionIdDto, UserIdDto,
    },
    services::{DocumentService, GraphService, SessionService},
};
use bric_a_brac_dtos::GraphIdDto;
use bric_a_brac_protos::common::{EdgeSchemaProto, GraphSchemaProto, NodeSchemaProto};
use bric_a_brac_protos::metadata::{
    metadata_server::Metadata, AppendSessionMessagesRequest, AppendSessionMessagesResponse,
    CloseSessionRequest, CreateEdgeSchemaRequest, CreateNodeSchemaRequest,
    CreateSessionRequest, GetSchemaRequest,
    GetSessionDocumentRequest, GetSessionMessagesRequest, GetSessionMessagesResponse,
    GetSessionRequest, SessionDocumentProto, SessionProto,
};
use std::str::FromStr;
use tonic::{Request, Response, Status};

pub struct MetadataGrpcService {
    session_service: SessionService,
    graph_service: GraphService,
    document_service: DocumentService,
}

impl MetadataGrpcService {
    pub fn new(
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
    #[tracing::instrument(level = "trace", name = "grpc.create_session", skip(self, request), err)]
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let graph_id = GraphIdDto::from_str(&req.graph_id)
            .map_err(|_| Status::invalid_argument("Invalid graph_id"))?;
        let user_id = UserIdDto::from_str(&req.user_id)
            .map_err(|_| Status::invalid_argument("Invalid user_id"))?;

        let session = self
            .session_service
            .create_session(CreateSessionDto { graph_id }, user_id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_session", skip(self, request), err)]
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id = SessionIdDto::from_str(&req.session_id)
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let session = self
            .session_service
            .get_session(session_id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.close_session", skip(self, request), err)]
    async fn close_session(
        &self,
        request: Request<CloseSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id = SessionIdDto::from_str(&req.session_id)
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let session = self
            .session_service
            .close_session(session_id, &req.status)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

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
        let session_id = SessionIdDto::from_str(&req.session_id)
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let messages = self
            .session_service
            .get_messages(session_id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

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
        let session_id = SessionIdDto::from_str(&req.session_id)
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

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
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(AppendSessionMessagesResponse {}))
    }

    #[tracing::instrument(level = "trace", name = "grpc.create_node_schema", skip(self, request), err)]
    async fn create_node_schema(
        &self,
        request: Request<CreateNodeSchemaRequest>,
    ) -> Result<Response<NodeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id = GraphIdDto::from_str(&req.graph_id)
            .map_err(|_| Status::invalid_argument("Invalid graph_id"))?;

        let schema = self
            .graph_service
            .create_node_schema(graph_id, req.label, req.description)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.create_edge_schema", skip(self, request), err)]
    async fn create_edge_schema(
        &self,
        request: Request<CreateEdgeSchemaRequest>,
    ) -> Result<Response<EdgeSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id = GraphIdDto::from_str(&req.graph_id)
            .map_err(|_| Status::invalid_argument("Invalid graph_id"))?;

        let schema = self
            .graph_service
            .create_edge_schema(graph_id, req.label, req.description)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_schema", skip(self, request), err)]
    async fn get_schema(
        &self,
        request: Request<GetSchemaRequest>,
    ) -> Result<Response<GraphSchemaProto>, Status> {
        let req = request.into_inner();
        let graph_id = GraphIdDto::from_str(&req.graph_id)
            .map_err(|_| Status::invalid_argument("Invalid graph_id"))?;

        let schema = self
            .graph_service
            .get_schema(graph_id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_session_document", skip(self, request), err)]
    async fn get_session_document(
        &self,
        request: Request<GetSessionDocumentRequest>,
    ) -> Result<Response<SessionDocumentProto>, Status> {
        let req = request.into_inner();
        let document_id = SessionDocumentIdDto::from_str(&req.document_id)
            .map_err(|_| Status::invalid_argument("Invalid document_id"))?;

        let document = self
            .document_service
            .get_document(document_id)
            .await
            .map_err(|e| Status::internal(format!("{e:?}")))?;

        Ok(Response::new(document.into()))
    }
}
