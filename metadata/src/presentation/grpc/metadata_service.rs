use crate::{
    application::{
        dtos::session_dto::{create_messages_from_proto, create_session_from_proto},
        services::{GraphService, SessionService},
    },
    domain::models::SessionStatusModel,
};
use bric_a_brac_dtos::GraphIdDto;
use bric_a_brac_protos::metadata::{
    metadata_server::Metadata, AppendSessionMessagesRequest, AppendSessionMessagesResponse,
    CloseSessionRequest, CreateEdgeSchemaRequest, CreateNodeSchemaRequest,
    CreateSessionRequest, GetSchemaRequest, GetSessionMessagesRequest,
    GetSessionMessagesResponse, GetSessionRequest, SessionProto,
};
use bric_a_brac_protos::common::{EdgeSchemaProto, GraphSchemaProto, NodeSchemaProto};
use std::str::FromStr;
use tonic::{Request, Response, Status};

pub struct MetadataGrpcService {
    session_service: SessionService,
    graph_service: GraphService,
}

impl MetadataGrpcService {
    pub fn new(session_service: SessionService, graph_service: GraphService) -> Self {
        Self {
            session_service,
            graph_service,
        }
    }
}

#[tonic::async_trait]
impl Metadata for MetadataGrpcService {
    #[tracing::instrument(level = "trace", name = "grpc.create_session", skip(self, request))]
    async fn create_session(
        &self,
        request: Request<CreateSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let create = create_session_from_proto(req.graph_id, req.user_id)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        let session = self
            .session_service
            .create_session(create)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_session", skip(self, request))]
    async fn get_session(
        &self,
        request: Request<GetSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id = req
            .session_id
            .parse()
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let session = self
            .session_service
            .get_session(session_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.close_session", skip(self, request))]
    async fn close_session(
        &self,
        request: Request<CloseSessionRequest>,
    ) -> Result<Response<SessionProto>, Status> {
        let req = request.into_inner();
        let session_id = req
            .session_id
            .parse()
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;
        let status: SessionStatusModel = req
            .status
            .parse()
            .map_err(|_| Status::invalid_argument("Invalid status, expected: completed or error"))?;

        let session = self
            .session_service
            .close_session(session_id, status)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(session.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.get_session_messages",
        skip(self, request)
    )]
    async fn get_session_messages(
        &self,
        request: Request<GetSessionMessagesRequest>,
    ) -> Result<Response<GetSessionMessagesResponse>, Status> {
        let req = request.into_inner();
        let session_id = req
            .session_id
            .parse()
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let messages = self
            .session_service
            .get_messages(session_id)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(GetSessionMessagesResponse {
            messages: messages.into_iter().map(From::from).collect(),
        }))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.append_session_messages",
        skip(self, request)
    )]
    async fn append_session_messages(
        &self,
        request: Request<AppendSessionMessagesRequest>,
    ) -> Result<Response<AppendSessionMessagesResponse>, Status> {
        let req = request.into_inner();
        let session_id = req
            .session_id
            .parse()
            .map_err(|_| Status::invalid_argument("Invalid session_id"))?;

        let messages = create_messages_from_proto(session_id, 0, req.messages)
            .map_err(|e| Status::invalid_argument(e.to_string()))?;

        self.session_service
            .append_messages(session_id, messages)
            .await
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(AppendSessionMessagesResponse {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.create_node_schema",
        skip(self, request)
    )]
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
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "grpc.create_edge_schema",
        skip(self, request)
    )]
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
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(schema.into()))
    }

    #[tracing::instrument(level = "trace", name = "grpc.get_schema", skip(self, request))]
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
            .map_err(|e| Status::internal(e.to_string()))?;

        Ok(Response::new(schema.into()))
    }
}
