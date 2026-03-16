use std::collections::HashMap;

use crate::{
    infrastructure::repositories::GraphRepository,
    presentation::errors::{AppError, RequestError},
};
use bric_a_brac_dtos::{CreateGraphDataDto, GraphIdDto, GraphSchemaDto, NodeDataIdDto};
use sqlx::PgPool;

#[derive(Clone)]
pub struct ValidationService {
    pool: PgPool,
    repository: GraphRepository,
}

impl ValidationService {
    pub fn new(pool: PgPool, repository: GraphRepository) -> Self {
        Self { pool, repository }
    }

    #[tracing::instrument(
        level = "trace",
        name = "validation_service.validate_create_graph_data",
        skip(self, create_graph_data)
    )]
    pub async fn validate_create_graph_data(
        &self,
        graph_id: GraphIdDto,
        create_graph_data: &mut CreateGraphDataDto,
    ) -> Result<(), AppError> {
        let mut txn = self.pool.begin().await?;
        let graph_schema = self
            .repository
            .get_schema(&mut txn, graph_id.into())
            .await?;

        // Remap all client-provided node IDs to fresh server-generated UUIDs so
        // clients cannot influence the IDs stored in the database.
        let id_map: HashMap<NodeDataIdDto, NodeDataIdDto> = create_graph_data
            .nodes
            .iter()
            .map(|n| (n.node_data_id, NodeDataIdDto::new()))
            .collect();

        for node in &mut create_graph_data.nodes {
            node.node_data_id = id_map[&node.node_data_id];
        }

        for edge in &mut create_graph_data.edges {
            edge.from_node_data_id =
                *id_map
                    .get(&edge.from_node_data_id)
                    .ok_or_else(|| RequestError::InvalidInput {
                        field: format!("Edge from_node_data_id '{}'", edge.from_node_data_id),
                        issue: "References a node ID not present in the nodes list".to_string(),
                    })?;
            edge.to_node_data_id =
                *id_map
                    .get(&edge.to_node_data_id)
                    .ok_or_else(|| RequestError::InvalidInput {
                        field: format!("Edge to_node_data_id '{}'", edge.to_node_data_id),
                        issue: "References a node ID not present in the nodes list".to_string(),
                    })?;
        }

        // Schema compliance check (shared logic with the AI service)
        let graph_schema_dto: GraphSchemaDto = graph_schema.into();
        create_graph_data
            .validate_against_schema(&graph_schema_dto)
            .map_err(RequestError::SchemaCompliance)?;

        Ok(())
    }
}
