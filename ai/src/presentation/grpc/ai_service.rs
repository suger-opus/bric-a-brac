use bric_a_brac_protos::ai::{
    ai_server::Ai, AgentEventProto, SendMessageRequest,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct AiService;

impl AiService {
    pub fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl Ai for AiService {
    type SendMessageStream = ReceiverStream<Result<AgentEventProto, Status>>;

    async fn send_message(
        &self,
        _request: Request<SendMessageRequest>,
    ) -> Result<Response<Self::SendMessageStream>, Status> {
        todo!("SendMessage will be implemented in Step 6")
    }
}
