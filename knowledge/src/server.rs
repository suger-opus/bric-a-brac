use crate::services::Service;
use bric_a_brac_protos::knowledge::{
    knowledge_server::{Knowledge, KnowledgeServer as KnowledgeGrpcServer}, EdgeData, GraphData,
    InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest, NodeData,
};
use tonic::{Request, Response, Status};

pub struct KnowledgeServer {
    service: Service,
}

impl KnowledgeServer {
    pub async fn new(service: Service) -> anyhow::Result<Self> {
        Ok(Self { service })
    }

    pub fn into_service(self) -> KnowledgeGrpcServer<Self> {
        KnowledgeGrpcServer::new(self)
    }
}

#[tonic::async_trait]
impl Knowledge for KnowledgeServer {
    async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<NodeData>, Status> {
        self.service.insert_node(request).await
    }

    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<EdgeData>, Status> {
        self.service.insert_edge(request).await
    }

    async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<GraphData>, Status> {
        self.service.load_graph(request).await
    }
}
