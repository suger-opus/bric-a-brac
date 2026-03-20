use crate::{
    application::services::{MutateService, QueryService},
    application::errors::AppError,
};
use bric_a_brac_dtos::{
    EdgeDataDto, InsertEdgeDataDto, InsertNodeDataDto, KeyDto, NodeDataDto,
    UpdateNodeDataDto,
};
use bric_a_brac_protos::{
    common::{EdgeDataProto, GraphDataProto, NodeDataProto, PathProto, SubgraphProto},
    knowledge::{
        knowledge_server::Knowledge, DeleteEdgeRequest, DeleteEdgeResponse, DeleteNodeRequest,
        DeleteNodeResponse, FindPathsRequest,
        FindPathsResponse, GetNeighborsRequest, GetNeighborsResponse, GetNodeRequest,
        InitializeSchemaRequest, InitializeSchemaResponse, InsertEdgeRequest, InsertNodeRequest,
        LoadGraphRequest, SearchNodesRequest, SearchNodesResponse, UpdateNodeRequest,
    },
};
use tonic::{Request, Response, Status};
use validator::Validate;

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

    async fn initialize_schema(
        &self,
        request: Request<InitializeSchemaRequest>,
    ) -> Result<Response<InitializeSchemaResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_keys = ?req.node_keys);

        for key_str in &req.node_keys {
            let key: KeyDto = key_str.clone().into();
            key.validate().map_err(|e| AppError::InvalidInput {
                reason: format!("Invalid node key '{}': {}", key_str, e),
            })?;
        }

        self.mutate_service
            .initialize_schema(req.node_keys)
            .await?;

        Ok(Response::new(InitializeSchemaResponse {}))
    }

    async fn insert_node(
        &self,
        request: Request<InsertNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let node_dto: InsertNodeDataDto = req
            .node
            .ok_or_else(|| AppError::InvalidInput {
                reason: "Missing node field".to_string(),
            })?
            .try_into()
            .map_err(AppError::from)?;

        node_dto.validate().map_err(|e| AppError::InvalidInput {
            reason: e.to_string(),
        })?;

        let node = self
            .mutate_service
            .insert_node(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                node_dto.into(),
            )
            .await?;

        let dto: NodeDataDto = node.into();
        Ok(Response::new(dto.into()))
    }

    async fn update_node(
        &self,
        request: Request<UpdateNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let node_dto: UpdateNodeDataDto = req
            .node
            .ok_or_else(|| AppError::InvalidInput {
                reason: "Missing node field".to_string(),
            })?
            .try_into()
            .map_err(AppError::from)?;

        let node = self
            .mutate_service
            .update_node(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                node_dto.into(),
            )
            .await?;

        let dto: NodeDataDto = node.into();
        Ok(Response::new(dto.into()))
    }

    async fn delete_node(
        &self,
        request: Request<DeleteNodeRequest>,
    ) -> Result<Response<DeleteNodeResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id);

        self.mutate_service
            .delete_node(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.node_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        Ok(Response::new(DeleteNodeResponse {}))
    }

    async fn delete_edge(
        &self,
        request: Request<DeleteEdgeRequest>,
    ) -> Result<Response<DeleteEdgeResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, edge_data_id = %req.edge_data_id);

        self.mutate_service
            .delete_edge(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.edge_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        Ok(Response::new(DeleteEdgeResponse {}))
    }

    async fn insert_edge(
        &self,
        request: Request<InsertEdgeRequest>,
    ) -> Result<Response<EdgeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let edge_dto: InsertEdgeDataDto = req
            .edge
            .ok_or_else(|| AppError::InvalidInput {
                reason: "Missing edge field".to_string(),
            })?
            .try_into()
            .map_err(AppError::from)?;

        edge_dto.validate().map_err(|e| AppError::InvalidInput {
            reason: e.to_string(),
        })?;

        let edge = self
            .mutate_service
            .insert_edge(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                edge_dto.into(),
            )
            .await?;

        let dto: EdgeDataDto = edge.into();
        Ok(Response::new(dto.into()))
    }

    async fn search_nodes(
        &self,
        request: Request<SearchNodesRequest>,
    ) -> Result<Response<SearchNodesResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_key = ?req.node_key, limit = req.limit);

        if let Some(ref nk) = req.node_key {
            let key: KeyDto = nk.clone().into();
            key.validate().map_err(|e| AppError::InvalidInput {
                reason: format!("Invalid node key '{}': {}", nk, e),
            })?;
        }

        let results = self
            .query_service
            .search_nodes(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.node_key,
                req.query_embedding,
                req.limit,
            )
            .await?;

        Ok(Response::new(SearchNodesResponse {
            nodes: results.into_iter().map(Into::into).collect(),
        }))
    }

    async fn get_node(
        &self,
        request: Request<GetNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id);

        let node = self
            .query_service
            .get_node(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.node_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
            )
            .await?;

        let dto: NodeDataDto = node.into();
        Ok(Response::new(dto.into()))
    }

    async fn get_neighbors(
        &self,
        request: Request<GetNeighborsRequest>,
    ) -> Result<Response<GetNeighborsResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id, depth = req.depth);

        if let Some(ref ek) = req.edge_key {
            let key: KeyDto = ek.clone().into();
            key.validate().map_err(|e| AppError::InvalidInput {
                reason: format!("Invalid edge key '{}': {}", ek, e),
            })?;
        }

        let subgraph = self
            .query_service
            .get_neighbors(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.node_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.edge_key,
                req.depth,
            )
            .await?;

        let proto: SubgraphProto = subgraph.into();
        Ok(Response::new(GetNeighborsResponse {
            subgraph: Some(proto),
        }))
    }

    async fn find_paths(
        &self,
        request: Request<FindPathsRequest>,
    ) -> Result<Response<FindPathsResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(
            graph_id = %req.graph_id,
            from = %req.from_node_data_id,
            to = %req.to_node_data_id,
            max_depth = req.max_depth
        );

        let paths = self
            .query_service
            .find_paths(
                req.graph_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.from_node_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.to_node_data_id
                    .try_into()
                    .map_err(|err| AppError::from(err))?,
                req.max_depth,
            )
            .await?;

        Ok(Response::new(FindPathsResponse {
            paths: paths.into_iter().map(PathProto::from).collect(),
        }))
    }
}
