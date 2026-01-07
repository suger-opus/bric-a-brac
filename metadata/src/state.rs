use crate::grpc_client::KnowledgeClient;

#[derive(Clone)]
pub struct AppState {
    pub knowledge_client: KnowledgeClient,
}
