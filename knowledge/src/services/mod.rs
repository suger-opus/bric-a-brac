use crate::{error::ApiError, repositories::Repository};
use bric_a_brac_protos::knowledge::{
    EdgeData, GraphData, InsertEdgeRequest, InsertNodeRequest, LoadGraphRequest, NodeData,
};
use neo4rs::Graph;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct Service {
    graph: Arc<Graph>,
    repository: Repository,
}

impl Service {
    pub async fn new(graph: Arc<Graph>, repository: Repository) -> anyhow::Result<Self> {
        Ok(Self { graph, repository })
    }

    pub async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<NodeData>, Status> {
        let req = request.into_inner();
        let mut txn = self
            .graph
            .start_txn()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;
        let node = self
            .repository
            .insert_node(
                &mut txn,
                req.graph_id,
                req.node_data_id,
                req.formatted_label,
                req.properties,
            )
            .await?;
        txn.commit()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;

        Ok(Response::new(node))
    }

    pub async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<EdgeData>, Status> {
        let req = request.into_inner();
        let mut txn = self
            .graph
            .start_txn()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;
        let edge = self
            .repository
            .insert_edge(
                &mut txn,
                req.edge_data_id,
                req.from_node_data_id,
                req.to_node_data_id,
                req.formatted_label,
                req.properties,
            )
            .await?;
        txn.commit()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;

        Ok(Response::new(edge))
    }

    pub async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<GraphData>, Status> {
        let req = request.into_inner();
        let mut txn = self
            .graph
            .start_txn()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;
        let graph = self.repository.load_graph(&mut txn, req.graph_id).await?;
        txn.commit()
            .await
            .map_err(|e| ApiError::UnkownDatabaseError(e))?;

        Ok(Response::new(graph))
    }
}
