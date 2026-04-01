use crate::{application::AgentService, presentation::error::PresentationError};
use bric_a_brac_dtos::{AgentEventDto, SessionDocumentIdDto};
use bric_a_brac_protos::ai::{ai_server::Ai, AgentEventProto, SendMessageRequest};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub struct AiService {
    agent_service: AgentService,
}

impl AiService {
    pub const fn new(agent_service: AgentService) -> Self {
        Self { agent_service }
    }
}

#[tonic::async_trait]
impl Ai for AiService {
    type SendMessageStream = ReceiverStream<Result<AgentEventProto, Status>>;

    #[tracing::instrument(
        level = "trace",
        name = "ai_service.send_message",
        skip(self, request),
        err
    )]
    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<Self::SendMessageStream>, Status> {
        let req = request.into_inner();

        // Channel for raw agent events
        let (agent_tx, mut agent_rx) = mpsc::channel::<AgentEventDto>(64);

        // Channel for gRPC stream results
        let (result_tx, result_rx) = mpsc::channel::<Result<AgentEventProto, Status>>(64);

        // Spawn the agent loop (sends events to agent_tx)
        self.agent_service.send_message(
            req.session_id.try_into().map_err(PresentationError::from)?,
            req.content,
            req.document_id
                .map(SessionDocumentIdDto::try_from)
                .transpose()
                .map_err(PresentationError::from)?,
            agent_tx,
        );

        // Forward agent events as Ok(event) to the gRPC stream
        tokio::spawn(async move {
            while let Some(event) = agent_rx.recv().await {
                if result_tx.send(Ok(event.into())).await.is_err() {
                    break;
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(result_rx)))
    }
}
