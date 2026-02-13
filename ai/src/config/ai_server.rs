use std::net::{IpAddr, SocketAddr};

#[derive(Clone, clap::Args, derive_more::Debug)]
pub struct AiServerConfig {
    /// AI server host
    #[arg(long, env = "AI_SERVER_HOST", required = true)]
    ai_server_host: IpAddr,

    /// AI server port
    #[arg(long, env = "AI_SERVER_PORT", required = true)]
    ai_server_port: u16,
}

impl AiServerConfig {
    pub fn url(&self) -> SocketAddr {
        (self.ai_server_host, self.ai_server_port).into()
    }
}
