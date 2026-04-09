use std::net::{IpAddr, SocketAddr};

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct KnowledgeServerConfig {
    /// Knowledge server host
    #[arg(long, env = "KNOWLEDGE_SERVER_HOST", required = true)]
    knowledge_server_host: IpAddr,

    /// Knowledge gRPC server port
    #[arg(long, env = "KNOWLEDGE_GRPC_SERVER_PORT", required = true)]
    knowledge_grpc_server_port: u16,
}

impl KnowledgeServerConfig {
    pub fn url(&self) -> SocketAddr {
        (self.knowledge_server_host, self.knowledge_grpc_server_port).into()
    }
}
