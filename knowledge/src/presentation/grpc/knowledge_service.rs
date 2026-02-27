use crate::{
    application::services::{MutateService, QueryService},
    presentation::errors::AppError,
};
use bric_a_brac_protos::{
    common::{EdgeDataProto, GraphDataProto, NodeDataProto},
    knowledge::{
        knowledge_server::Knowledge, InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest,
    },
};
use tonic::{Request, Response, Status};

pub struct KnowledgeService {
    query_service: QueryService,
    mutate_service: MutateService,
}

impl KnowledgeService {
    pub fn new(query_service: QueryService, mutate_service: MutateService) -> Self {
        Self {
            query_service,
            mutate_service,
        }
    }
}

#[tonic::async_trait]
impl Knowledge for KnowledgeService {
    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<GraphDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let data = self
            .query_service
            .load_graph(req.graph_id.try_into().map_err(|err| AppError::from(err))?)
            .await?;

        Ok(Response::new(data.into()))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let node = self
            .mutate_service
            .insert_node(
                req.graph_id.try_into().map_err(|err| AppError::from(err))?,
                req.node_data
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        Ok(Response::new(node.into()))
    }

    #[tracing::instrument(level = "trace", skip(self, request))]
    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<EdgeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let edge = self
            .mutate_service
            .insert_edge(
                req.graph_id.try_into().map_err(|err| AppError::from(err))?,
                req.edge_data
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        Ok(Response::new(edge.into()))
    }
}
