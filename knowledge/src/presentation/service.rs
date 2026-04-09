use super::PresentationError;
use crate::application::{MutateService, QueryService};
use bric_a_brac_dtos::{
    CreateEdgeDataDto, CreateNodeDataDto, KeyDto, UpdateEdgeDataDto, UpdateNodeDataDto,
};
use bric_a_brac_protos::{
    common::{EdgeDataProto, Empty, GraphDataProto, NodeDataProto},
    knowledge::{
        knowledge_server::Knowledge, CreateEdgeRequest, CreateNodeRequest, DeleteEdgeRequest,
        DeleteGraphRequest, DeleteNodeRequest, FindPathsRequest, FindPathsResponse,
        GetNeighborsRequest, GetNodeRequest, InitializeSchemaRequest, LoadGraphRequest,
        SearchNodesRequest, SearchNodesResponse, UpdateEdgeRequest, UpdateNodeRequest,
    },
};
use tonic::{Request, Response, Status};
use validator::Validate;

pub struct KnowledgeService {
    query_service: QueryService,
    mutate_service: MutateService,
}

impl KnowledgeService {
    pub const fn new(query_service: QueryService, mutate_service: MutateService) -> Self {
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
        skip(self, request),
        err
    )]
    async fn load_graph(
        &self,
        request: Request<LoadGraphRequest>,
    ) -> Result<Response<GraphDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let data = self
            .query_service
            .load_graph(req.graph_id.try_into().map_err(PresentationError::from)?)
            .await?;

        Ok(Response::new(data.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.initialize_schema",
        skip(self, request),
        err
    )]
    async fn initialize_schema(
        &self,
        request: Request<InitializeSchemaRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_keys = ?req.node_keys);

        for key_str in &req.node_keys {
            let key: KeyDto = key_str.clone().into();
            key.validate().map_err(PresentationError::from)?;
        }
        self.mutate_service.initialize_schema(req.node_keys).await?;

        Ok(Response::new(Empty {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.create_node",
        skip(self, request),
        err
    )]
    async fn create_node(
        &self,
        request: Request<CreateNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let node_dto: CreateNodeDataDto = req
            .node
            .ok_or_else(|| PresentationError::MissingField("node".to_owned()))?
            .try_into()
            .map_err(PresentationError::from)?;

        node_dto
            .validate()
            .map_err(PresentationError::ValidationErrors)?;

        let node = self
            .mutate_service
            .create_node(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                node_dto,
            )
            .await?;

        Ok(Response::new(node.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.update_node",
        skip(self, request),
        err
    )]
    async fn update_node(
        &self,
        request: Request<UpdateNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let node: UpdateNodeDataDto = req
            .node
            .ok_or_else(|| PresentationError::MissingField("node".to_owned()))?
            .try_into()
            .map_err(PresentationError::from)?;

        node.validate()
            .map_err(PresentationError::ValidationErrors)?;

        let node = self
            .mutate_service
            .update_node(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                node,
            )
            .await?;

        Ok(Response::new(node.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.delete_node",
        skip(self, request),
        err
    )]
    async fn delete_node(
        &self,
        request: Request<DeleteNodeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id);

        self.mutate_service
            .delete_node(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.node_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
            )
            .await?;

        Ok(Response::new(Empty {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.delete_edge",
        skip(self, request),
        err
    )]
    async fn delete_edge(
        &self,
        request: Request<DeleteEdgeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, edge_data_id = %req.edge_data_id);

        self.mutate_service
            .delete_edge(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.edge_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
            )
            .await?;

        Ok(Response::new(Empty {}))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.create_edge",
        skip(self, request),
        err
    )]
    async fn create_edge(
        &self,
        request: Request<CreateEdgeRequest>,
    ) -> Result<Response<EdgeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let edge: CreateEdgeDataDto = req
            .edge
            .ok_or_else(|| PresentationError::MissingField("edge".to_owned()))?
            .try_into()
            .map_err(PresentationError::from)?;

        edge.validate()
            .map_err(PresentationError::ValidationErrors)?;

        let edge = self
            .mutate_service
            .create_edge(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                edge,
            )
            .await?;

        Ok(Response::new(edge.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.update_edge",
        skip(self, request),
        err
    )]
    async fn update_edge(
        &self,
        request: Request<UpdateEdgeRequest>,
    ) -> Result<Response<EdgeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id);

        let edge: UpdateEdgeDataDto = req
            .edge
            .ok_or_else(|| PresentationError::MissingField("edge".to_owned()))?
            .try_into()
            .map_err(PresentationError::from)?;

        edge.validate()
            .map_err(PresentationError::ValidationErrors)?;

        let edge = self
            .mutate_service
            .update_edge(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                edge,
            )
            .await?;

        Ok(Response::new(edge.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.search_nodes",
        skip(self, request),
        err
    )]
    async fn search_nodes(
        &self,
        request: Request<SearchNodesRequest>,
    ) -> Result<Response<SearchNodesResponse>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_key = ?req.node_key, limit = req.limit);

        if let Some(ref nk) = req.node_key {
            let key: KeyDto = nk.clone().into();
            key.validate()
                .map_err(PresentationError::ValidationErrors)?;
        }

        #[allow(clippy::cast_sign_loss)]
        let results = self
            .query_service
            .search_nodes(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.node_key,
                req.query_embedding,
                req.limit as u32,
            )
            .await?;

        Ok(Response::new(SearchNodesResponse {
            nodes: results.into_iter().map(Into::into).collect(),
        }))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.get_node",
        skip(self, request),
        err
    )]
    async fn get_node(
        &self,
        request: Request<GetNodeRequest>,
    ) -> Result<Response<NodeDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id);

        let node = self
            .query_service
            .get_node(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.node_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
            )
            .await?;

        Ok(Response::new(node.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.get_neighbors",
        skip(self, request),
        err
    )]
    async fn get_neighbors(
        &self,
        request: Request<GetNeighborsRequest>,
    ) -> Result<Response<GraphDataProto>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_data_id = %req.node_data_id, depth = req.depth);

        if let Some(ref ek) = req.edge_key {
            let key: KeyDto = ek.clone().into();
            key.validate()
                .map_err(PresentationError::ValidationErrors)?;
        }

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let depth = if (1..=10).contains(&req.depth) {
            req.depth as u8
        } else {
            return Err(PresentationError::DepthOutOfRange { value: req.depth }.into());
        };

        let subgraph = self
            .query_service
            .get_neighbors(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.node_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
                req.edge_key,
                depth,
            )
            .await?;

        Ok(Response::new(subgraph.into()))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.find_paths",
        skip(self, request),
        err
    )]
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

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        let max_depth = if (1..=10).contains(&req.max_depth) {
            req.max_depth as u8
        } else {
            return Err(PresentationError::DepthOutOfRange {
                value: req.max_depth,
            }
            .into());
        };

        let paths = self
            .query_service
            .find_paths(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.from_node_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
                req.to_node_data_id
                    .try_into()
                    .map_err(PresentationError::from)?,
                max_depth,
            )
            .await?;

        Ok(Response::new(FindPathsResponse {
            paths: paths.into_iter().map(Into::into).collect(),
        }))
    }

    #[tracing::instrument(
        level = "trace",
        name = "knowledge_service.delete_graph",
        skip(self, request),
        err
    )]
    async fn delete_graph(
        &self,
        request: Request<DeleteGraphRequest>,
    ) -> Result<Response<Empty>, Status> {
        let req = request.into_inner();
        tracing::debug!(graph_id = %req.graph_id, node_keys = ?req.node_keys);

        self.mutate_service
            .delete_graph_data(
                req.graph_id.try_into().map_err(PresentationError::from)?,
                req.node_keys,
            )
            .await?;

        Ok(Response::new(Empty {}))
    }
}
