use crate::services::Service;
use bric_a_brac_protos::{
    common::{EdgeDataProto, GraphDataProto, NodeDataProto},
    knowledge::{
        knowledge_server::{Knowledge, KnowledgeServer as KnowledgeGrpcServer},
        InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest,
    },
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
    ) -> Result<Response<NodeDataProto>, Status> {
        self.service.insert_node(request).await
    }

    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<EdgeDataProto>, Status> {
        self.service.insert_edge(request).await
    }

    async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<GraphDataProto>, Status> {
        self.service.load_graph(request).await
    }
}
