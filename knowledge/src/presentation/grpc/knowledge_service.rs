use crate::{
    application::services::{MutateService, QueryService},
    presentation::errors::AppError,
};
use bric_a_brac_protos::{
    common::GraphDataProto,
    knowledge::{knowledge_server::Knowledge, InsertGraphRequest, LoadGraphRequest},
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
    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.load_graph",
        skip(self, request)
    )]
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

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.insert_graph",
        skip(self, request)
    )]
    async fn insert_graph(
        &self,
        request: Request<InsertGraphRequest>,
    ) -> Result<Response<GraphDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let data = self
            .mutate_service
            .insert_graph(
                req.graph_id
                    .clone()
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.graph_data
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        Ok(Response::new(data.into()))
    }
}
