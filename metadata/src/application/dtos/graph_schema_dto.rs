use crate::domain::models::GraphSchemaModel;
use bric_a_brac_dtos::GraphSchemaDto;

impl From<GraphSchemaModel> for GraphSchemaDto {
    fn from(schema: GraphSchemaModel) -> Self {
        Self {
            nodes: schema.nodes.into_iter().map(From::from).collect(),
            edges: schema.edges.into_iter().map(From::from).collect(),
        }
    }
}
