use crate::{
    application::{AppError, CreateGraphDto, GraphMetadataDto},
    domain::{
        CreateAccessModel, CreateEdgeSchemaModel, CreateNodeSchemaModel, EdgeSchemaIdModel,
        NodeSchemaIdModel, RoleModel,
    },
    infrastructure::{AccessRepository, GraphRepository, KnowledgeClient},
};
use bric_a_brac_dtos::{
    ColorDto, EdgeSchemaDto, GraphDataDto, GraphIdDto, GraphSchemaDto, KeyDto, LabelDto,
    NodeSchemaDto, UserIdDto,
};
use sqlx::PgPool;

#[derive(Clone)]
pub struct GraphService {
    pool: PgPool,
    repository: GraphRepository,
    access_repository: AccessRepository,
    knowledge_client: KnowledgeClient,
}

impl GraphService {
    pub const fn new(
        pool: PgPool,
        repository: GraphRepository,
        access_repository: AccessRepository,
        knowledge_client: KnowledgeClient,
    ) -> Self {
        Self {
            pool,
            repository,
            access_repository,
            knowledge_client,
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_all_metadata",
        skip(self, user_id),
        err
    )]
    pub async fn get_all_metadata(
        &self,
        user_id: UserIdDto,
    ) -> Result<Vec<GraphMetadataDto>, AppError> {
        let mut txn = self.pool.begin().await?;
        let graphs = self
            .repository
            .get_all_metadata(&mut txn, user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graphs.into_iter().map(From::from).collect())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.create_graph",
        skip(self, user_id, create_graph),
        err
    )]
    pub async fn create_graph(
        &self,
        user_id: UserIdDto,
        create_graph: CreateGraphDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph_id = self
            .repository
            .create_graph(&mut txn, create_graph.into())
            .await?;
        let create_access = CreateAccessModel {
            graph_id,
            user_id: user_id.into(),
            role: RoleModel::Owner,
        };
        self.access_repository
            .create(&mut txn, create_access)
            .await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph_id, user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_metadata",
        skip(self, graph_id, user_id),
        err
    )]
    pub async fn get_metadata(
        &self,
        graph_id: GraphIdDto,
        user_id: UserIdDto,
    ) -> Result<GraphMetadataDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let graph = self
            .repository
            .get_metadata(&mut txn, graph_id.into(), user_id.into())
            .await?;
        txn.commit().await?;

        Ok(graph.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_schema",
        skip(self, graph_id),
        err
    )]
    pub async fn get_schema(&self, graph_id: GraphIdDto) -> Result<GraphSchemaDto, AppError> {
        let mut txn = self.pool.begin().await?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.get_data",
        skip(self, graph_id),
        err
    )]
    pub async fn get_data(&self, graph_id: GraphIdDto) -> Result<GraphDataDto, AppError> {
        self.knowledge_client
            .load_graph(graph_id)
            .await
            .map_err(From::from)
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.create_node_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_node_schema(
        &self,
        graph_id: GraphIdDto,
        label: LabelDto,
        description: String,
    ) -> Result<NodeSchemaDto, AppError> {
        let key = KeyDto::new();
        let create = CreateNodeSchemaModel {
            node_schema_id: NodeSchemaIdModel::new(),
            graph_id: graph_id.into(),
            label: label.into(),
            key: key.clone().into(),
            color: ColorDto::new().into(),
            description,
        };

        let mut txn = self.pool.begin().await?;
        let schema = self.repository.create_node_schema(&mut txn, create).await?;
        txn.commit().await?;

        // Initialize schema in knowledge service
        self.knowledge_client
            .initialize_schema(graph_id, vec![key])
            .await?;

        Ok(schema.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.create_edge_schema",
        skip(self, graph_id, label, description),
        err
    )]
    pub async fn create_edge_schema(
        &self,
        graph_id: GraphIdDto,
        label: LabelDto,
        description: String,
    ) -> Result<EdgeSchemaDto, AppError> {
        let create = CreateEdgeSchemaModel {
            edge_schema_id: EdgeSchemaIdModel::new(),
            graph_id: graph_id.into(),
            label: label.into(),
            key: KeyDto::new().into(),
            color: ColorDto::new().into(),
            description,
        };

        let mut txn = self.pool.begin().await?;
        let schema = self.repository.create_edge_schema(&mut txn, create).await?;
        txn.commit().await?;

        Ok(schema.into())
    }

    #[tracing::instrument(
        level = "trace",
        name = "graph_service.delete_graph",
        skip(self, graph_id),
        err
    )]
    pub async fn delete_graph(&self, graph_id: GraphIdDto) -> Result<(), AppError> {
        // Fetch node schema keys so we can drop vector indexes in Memgraph
        let mut txn = self.pool.begin().await?;
        let schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;

        // Delete all graph data from Memgraph (nodes, edges, vector indexes)
        self.knowledge_client
            .delete_graph(
                graph_id,
                schema.nodes.into_iter().map(|n| n.key.into()).collect(),
            )
            .await?;

        // Delete graph from Postgres (CASCADE handles schemas, sessions, accesses)
        self.repository
            .delete_graph(&mut txn, graph_id.into())
            .await?;
        txn.commit().await?;

        Ok(())
    }
}
