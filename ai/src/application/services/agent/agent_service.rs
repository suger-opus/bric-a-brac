use super::{build_system_prompt, chunk_user_message};
use crate::{
    application::{
        services::{read_tools, session_tools, write_tools},
        AgentError, AppError, ToolService,
    },
    infrastructure::{
        Message, MetadataClient, OpenRouterClient, StreamChatResult, ToolCall, ToolDefinition,
    },
};
use bric_a_brac_dtos::{
    AgentEventDto, CreateSessionMessageDto, GraphIdDto, GraphSchemaDto, RoleDto,
    SessionDocumentIdDto, SessionIdDto, SessionMessageDto, SessionMessageRoleDto,
};
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};

const MAX_TOOL_ITERATIONS: usize = 200;

/// Maximum number of concurrent LLM calls across all agent sessions.
/// Prevents overwhelming the LLM provider (`OpenRouter`) with too many
/// simultaneous requests, which would cause 429 rate-limit errors.
const MAX_CONCURRENT_LLM_CALLS: usize = 20;

pub struct AgentService {
    openrouter_client: OpenRouterClient,
    metadata_client: MetadataClient,
    tool_service: ToolService,
    llm_semaphore: Arc<Semaphore>,
}

impl AgentService {
    pub fn new(
        openrouter_client: OpenRouterClient,
        metadata_client: MetadataClient,
        tool_service: ToolService,
    ) -> Self {
        Self {
            openrouter_client,
            metadata_client,
            tool_service,
            llm_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_LLM_CALLS)),
        }
    }

    #[tracing::instrument(
        level = "trace",
        name = "agent_service.send_message",
        skip(self, session_id, user_message, document_id, tx)
    )]
    pub fn send_message(
        &self,
        session_id: SessionIdDto,
        user_message: String,
        document_id: Option<SessionDocumentIdDto>,
        tx: mpsc::Sender<AgentEventDto>,
    ) {
        let openrouter_client = self.openrouter_client.clone();
        let metadata_client = self.metadata_client.clone();
        let tool_service = self.tool_service.clone();
        let llm_semaphore = Arc::clone(&self.llm_semaphore);

        tokio::spawn(async move {
            let result = async {
                let mut agent = AgentLoop::init(
                    openrouter_client,
                    metadata_client,
                    tool_service,
                    llm_semaphore,
                    session_id,
                    tx.clone(),
                )
                .await?;
                agent.run(&user_message, document_id).await
            }
            .await;

            if let Err(err) = result {
                tracing::error!(
                    error = %err,
                    error.source = err.source().map(tracing::field::display),
                    session_id = %session_id,
                    "Agent loop failed"
                );
                #[allow(clippy::let_underscore_must_use)]
                let _ = tx.send(event_error(&format!("{err}"))).await;
            }
        });
    }
}

// ── Agent loop ───────────────────────────────────────────────────────────

struct AgentLoop {
    // Infrastructure
    openrouter_client: OpenRouterClient,
    metadata_client: MetadataClient,
    tool_service: ToolService,
    llm_semaphore: Arc<Semaphore>,
    // Session context
    session_id: SessionIdDto,
    graph_id: GraphIdDto,
    role: RoleDto,
    tools: Vec<ToolDefinition>,
    tx: mpsc::Sender<AgentEventDto>,
    // Mutable state
    messages: Vec<Message>,
    schema: GraphSchemaDto,
    pending_messages: Vec<CreateSessionMessageDto>,
    iteration: usize,
}

impl AgentLoop {
    /// Load session context and construct the agent loop.
    #[tracing::instrument(
        level = "info",
        name = "agent.init",
        skip(openrouter_client, metadata_client, tool_service, llm_semaphore, tx),
        fields(%session_id),
        err
    )]
    async fn init(
        openrouter_client: OpenRouterClient,
        metadata_client: MetadataClient,
        tool_service: ToolService,
        llm_semaphore: Arc<Semaphore>,
        session_id: SessionIdDto,
        tx: mpsc::Sender<AgentEventDto>,
    ) -> Result<Self, AppError> {
        let session = metadata_client
            .get_session(session_id)
            .await
            .map_err(|err| AgentError::GetSession {
                session_id,
                source: err,
            })?;

        let existing_messages = metadata_client
            .get_session_messages(session_id)
            .await
            .map_err(|err| AgentError::GetSessionMessages {
                session_id,
                source: err,
            })?;

        let schema = metadata_client
            .get_schema(session.graph_id)
            .await
            .map_err(|err| AgentError::GetSchema {
                graph_id: session.graph_id,
                source: err,
            })?;

        let tools = build_tool_list(session.role);
        let messages = build_message_history(&schema, &existing_messages);

        Ok(Self {
            openrouter_client,
            metadata_client,
            tool_service,
            llm_semaphore,
            session_id,
            graph_id: session.graph_id,
            role: session.role,
            tools,
            tx,
            messages,
            schema,
            pending_messages: Vec::new(),
            iteration: 0,
        })
    }

    /// Main orchestration: build effective message, chunk it, process each chunk.
    async fn run(
        &mut self,
        user_message: &str,
        document_id: Option<SessionDocumentIdDto>,
    ) -> Result<(), AppError> {
        let effective_message = self
            .build_effective_message(user_message, document_id)
            .await?;
        let chunks = chunk_user_message(&effective_message);
        let total_chunks = chunks.len();
        let is_multi_chunk = total_chunks > 1;

        tracing::debug!(total_chunks, "Document split into chunks");

        for (chunk_idx, chunk_content) in chunks.iter().enumerate() {
            if is_multi_chunk {
                self.send(event_progress(&format!(
                    "Document split into {total_chunks} parts — extracting part {}…",
                    chunk_idx + 1,
                )))
                .await;
            }

            self.messages.push(Message::user(chunk_content));
            self.persist_user_chunk(
                user_message,
                chunk_content,
                document_id,
                chunk_idx,
                is_multi_chunk,
            )
            .await;

            if self.run_tool_loop(is_multi_chunk).await? {
                return Ok(());
            }

            self.flush_pending().await;
        }

        if is_multi_chunk {
            self.finalize_multi_chunk().await?;
        }

        self.flush_pending().await;

        tracing::info!(
            iterations = self.iteration,
            chunks = total_chunks,
            "Agent loop completed"
        );
        Ok(())
    }

    /// Load document content if attached and combine with the user message.
    async fn build_effective_message(
        &self,
        user_message: &str,
        document_id: Option<SessionDocumentIdDto>,
    ) -> Result<String, AppError> {
        let Some(doc_id) = document_id else {
            return Ok(user_message.to_owned());
        };

        let doc = self
            .metadata_client
            .get_session_document(doc_id)
            .await
            .map_err(|err| AgentError::GetSessionDocument {
                document_id: doc_id,
                source: err,
            })?;

        let doc_content = format!("[Document content]\n{}", doc.content);
        if user_message.is_empty() {
            Ok(doc_content)
        } else {
            Ok(format!("{doc_content}\n\n[User message]\n{user_message}"))
        }
    }

    /// Persist the user chunk(s) to the session history.
    async fn persist_user_chunk(
        &self,
        user_message: &str,
        chunk_content: &str,
        document_id: Option<SessionDocumentIdDto>,
        chunk_idx: usize,
        is_multi_chunk: bool,
    ) {
        if !is_multi_chunk {
            let content = if document_id.is_some() {
                user_message.to_owned()
            } else {
                chunk_content.to_owned()
            };
            self.persist(vec![user_msg(content, document_id, None)])
                .await;
            return;
        }

        if chunk_idx == 0 {
            // First chunk: also persist the original user message
            self.persist(vec![user_msg(user_message.to_owned(), document_id, None)])
                .await;
        }

        let chunk_num = i32::try_from(chunk_idx + 1).unwrap_or_default();
        self.persist(vec![user_msg(
            chunk_content.to_owned(),
            document_id,
            Some(chunk_num),
        )])
        .await;
    }

    /// Run the LLM → tool execution loop for the current chunk.
    /// Returns `true` if the "done" tool was called (session complete).
    #[allow(clippy::let_underscore_must_use)]
    async fn run_tool_loop(&mut self, is_multi_chunk: bool) -> Result<bool, AppError> {
        loop {
            if self.iteration >= MAX_TOOL_ITERATIONS {
                tracing::warn!("Maximum tool call limit reached");
                self.flush_pending().await;
                self.send(event_error("Maximum tool call limit reached."))
                    .await;
                return Ok(false);
            }

            let result = self.call_llm().await?;

            // Stream text to user for single-chunk messages only.
            // For multi-chunk, intermediate text stays in history silently.
            if !is_multi_chunk {
                if let Some(ref content) = result.content {
                    if !content.is_empty() {
                        self.send(event_text(content)).await;
                    }
                }
            }

            // No tool calls → done with this chunk
            if result.tool_calls.is_empty() {
                let content = result.content.clone().unwrap_or_default();
                self.messages
                    .push(Message::assistant(Some(content.clone()), None));
                self.pending_messages.push(assistant_msg(content, None));

                if !is_multi_chunk {
                    self.send(event_done(
                        result.content.as_deref().unwrap_or("Task completed."),
                    ))
                    .await;
                }
                return Ok(false);
            }

            // Append assistant message with tool calls to history
            let tool_calls_json = serde_json::to_string(&result.tool_calls).ok();
            self.messages.push(Message::assistant(
                result.content.clone(),
                Some(result.tool_calls.clone()),
            ));
            self.pending_messages.push(assistant_msg(
                result.content.unwrap_or_default(),
                tool_calls_json,
            ));

            // Execute tool calls
            if self.execute_tool_calls(&result.tool_calls).await? {
                return Ok(true); // "done" was called
            }

            self.iteration += 1;
        }
    }

    /// Call the LLM with the current message history, respecting the concurrency semaphore.
    async fn call_llm(&self) -> Result<StreamChatResult, AppError> {
        let permit = self
            .llm_semaphore
            .acquire()
            .await
            .map_err(|err| AgentError::Internal {
                message: "LLM semaphore closed".to_owned(),
                source: Some(Box::new(err)),
            })?;

        let result = self
            .openrouter_client
            .chat_stream(self.messages.clone(), Some(self.tools.clone()))
            .await
            .map_err(|err| AgentError::LlmCall {
                iteration: self.iteration,
                source: err,
            })?;

        drop(permit);
        Ok(result)
    }

    /// Execute a batch of tool calls. Returns `true` if the "done" tool was called.
    async fn execute_tool_calls(&mut self, tool_calls: &[ToolCall]) -> Result<bool, AppError> {
        let mut schema_changed = false;

        for tool_call in tool_calls {
            self.send(event_tool_call(
                &tool_call.id,
                &tool_call.function.name,
                &tool_call.function.arguments,
            ))
            .await;

            let tool_result = self
                .tool_service
                .execute(
                    &tool_call.function.name,
                    &tool_call.function.arguments,
                    self.graph_id,
                    self.session_id,
                    &self.schema,
                    self.role,
                )
                .await;

            self.send(event_tool_result(&tool_call.id, &tool_result.content))
                .await;

            self.messages
                .push(Message::tool(tool_call.id.clone(), &tool_result.content));
            self.pending_messages
                .push(tool_msg(tool_result.content.clone(), tool_call.id.clone()));

            if tool_result.schema_changed {
                schema_changed = true;
            }

            if tool_result.is_done {
                self.flush_pending().await;
                self.send(event_done(&tool_result.content)).await;
                return Ok(true);
            }
        }

        if schema_changed {
            self.refresh_schema().await?;
        }

        Ok(false)
    }

    /// Reload schema from metadata and update the system prompt.
    async fn refresh_schema(&mut self) -> Result<(), AppError> {
        self.schema = self
            .metadata_client
            .get_schema(self.graph_id)
            .await
            .map_err(|err| AgentError::GetSchema {
                graph_id: self.graph_id,
                source: err,
            })?;

        let system_msg = Message::system(build_system_prompt(&self.schema));
        if let Some(first) = self.messages.first_mut() {
            *first = system_msg;
        } else {
            self.messages.insert(0, system_msg);
        }

        Ok(())
    }

    /// Produce a final summary for multi-chunk documents.
    async fn finalize_multi_chunk(&mut self) -> Result<(), AppError> {
        self.send(event_progress("Finalising — preparing summary…"))
            .await;

        self.messages.push(Message::user(
            "All parts of the document have been processed. \
             Provide a concise summary of everything you extracted and stored in the graph.",
        ));

        let permit = self
            .llm_semaphore
            .acquire()
            .await
            .map_err(|err| AgentError::Internal {
                message: "LLM semaphore closed".to_owned(),
                source: Some(Box::new(err)),
            })?;

        let summary_result = self
            .openrouter_client
            .chat_stream(self.messages.clone(), None)
            .await
            .map_err(|err| AgentError::LlmCall {
                iteration: self.iteration,
                source: err,
            })?;

        drop(permit);

        let summary = summary_result
            .content
            .unwrap_or_else(|| "Document processing complete.".to_owned());
        self.send(event_text(&summary)).await;
        self.send(event_done(&summary)).await;

        self.pending_messages.push(assistant_msg(summary, None));

        Ok(())
    }

    /// Send an event to the client, ignoring channel-closed errors.
    #[allow(clippy::let_underscore_must_use)]
    async fn send(&self, event: AgentEventDto) {
        let _ = self.tx.send(event).await;
    }

    /// Persist pending messages and clear the buffer.
    async fn persist(&self, messages: Vec<CreateSessionMessageDto>) {
        if messages.is_empty() {
            return;
        }
        if let Err(err) = self
            .metadata_client
            .append_session_messages(self.session_id, messages)
            .await
        {
            tracing::error!(error = ?err, "Failed to persist session messages");
        }
    }

    /// Flush accumulated pending messages to storage.
    async fn flush_pending(&mut self) {
        let batch = std::mem::take(&mut self.pending_messages);
        self.persist(batch).await;
    }
}

// ── Message DTO builders ─────────────────────────────────────────────────

const fn user_msg(
    content: String,
    document_id: Option<SessionDocumentIdDto>,
    chunk_index: Option<i32>,
) -> CreateSessionMessageDto {
    CreateSessionMessageDto {
        role: SessionMessageRoleDto::User,
        content,
        tool_calls: None,
        tool_call_id: None,
        document_id,
        document_name: None,
        chunk_index,
    }
}

const fn assistant_msg(content: String, tool_calls: Option<String>) -> CreateSessionMessageDto {
    CreateSessionMessageDto {
        role: SessionMessageRoleDto::Assistant,
        content,
        tool_calls,
        tool_call_id: None,
        document_id: None,
        document_name: None,
        chunk_index: None,
    }
}

const fn tool_msg(content: String, tool_call_id: String) -> CreateSessionMessageDto {
    CreateSessionMessageDto {
        role: SessionMessageRoleDto::Tool,
        content,
        tool_calls: None,
        tool_call_id: Some(tool_call_id),
        document_id: None,
        document_name: None,
        chunk_index: None,
    }
}

// ── Tool list ────────────────────────────────────────────────────────────

fn build_tool_list(role: RoleDto) -> Vec<ToolDefinition> {
    let mut tools = read_tools();
    if matches!(role, RoleDto::Owner | RoleDto::Admin | RoleDto::Editor) {
        tools.extend(write_tools());
    }
    tools.extend(session_tools());
    tools
}

// ── Message history ──────────────────────────────────────────────────────

/// Maximum context window budget in characters (~125K tokens at ~4 chars/token).
/// GPT-4.1 supports 1M tokens but quality degrades on very long contexts.
const MAX_CONTEXT_CHARS: usize = 500_000;

fn build_message_history(
    schema: &GraphSchemaDto,
    existing_messages: &[SessionMessageDto],
) -> Vec<Message> {
    let system_prompt = build_system_prompt(schema);
    let system_size = system_prompt.len();
    let mut messages = vec![Message::system(system_prompt)];

    // Budget available for history (reserve some for the new user message + response)
    let budget = MAX_CONTEXT_CHARS
        .saturating_sub(system_size)
        .saturating_sub(20_000);

    // First pass: build messages with chunk compression
    let mut history: Vec<Message> = Vec::with_capacity(existing_messages.len());
    let mut total_chars: usize = 0;

    for msg in existing_messages {
        let message = match msg.role {
            SessionMessageRoleDto::User => {
                if msg.chunk_index.is_some() {
                    let chunk_label = format!(
                        "[Chunk {} — already processed. Use read_document to access content.]",
                        msg.chunk_index.unwrap_or(0),
                    );
                    Message::user(&chunk_label)
                } else if let Some(ref doc_id) = msg.document_id {
                    let note = format!(
                        "[Document attached: id={doc_id}. Use read_document tool to access its content.]\n{}",
                        msg.content
                    );
                    Message::user(&note)
                } else {
                    Message::user(&msg.content)
                }
            }
            SessionMessageRoleDto::Assistant => {
                let tool_calls = msg
                    .tool_calls
                    .as_ref()
                    .and_then(|tc| serde_json::from_str(tc).ok());
                Message::assistant(Some(msg.content.clone()), tool_calls)
            }
            SessionMessageRoleDto::Tool => {
                Message::tool(msg.tool_call_id.clone().unwrap_or_default(), &msg.content)
            }
            SessionMessageRoleDto::System => continue,
        };
        total_chars += message_char_size(&message);
        history.push(message);
    }

    // Second pass: if still over budget, collapse old tool sequences
    if total_chars > budget {
        total_chars = 0;
        let mut trimmed: Vec<Message> = Vec::with_capacity(history.len());

        let mut keep_from = 0;
        let mut running = 0usize;
        for (i, msg) in history.iter().enumerate().rev() {
            let size = message_char_size(msg);
            if running + size > budget && i > 0 {
                keep_from = i + 1;
                break;
            }
            running += size;
        }

        if keep_from > 0 && !history.is_empty() {
            if let Some(first) = history.first() {
                trimmed.push(first.clone());
                total_chars += message_char_size(first);
            }

            let dropped = keep_from - 1;
            if dropped > 0 {
                let note = format!("[{dropped} older messages trimmed to fit context window]");
                trimmed.push(Message::user(&note));
                total_chars += note.len();
            }
        }

        for msg in history
            .iter()
            .skip(keep_from.max(usize::from(keep_from > 0)))
        {
            total_chars += message_char_size(msg);
            trimmed.push(msg.clone());
        }

        history = trimmed;
        let _ = total_chars;
    }

    messages.extend(history);
    messages
}

fn message_char_size(msg: &Message) -> usize {
    msg.content.as_ref().map_or(0, std::string::String::len)
}

// ── Event builders ───────────────────────────────────────────────────────

fn event_text(content: &str) -> AgentEventDto {
    AgentEventDto::Text {
        content: content.to_owned(),
    }
}

fn event_tool_call(id: &str, name: &str, arguments: &str) -> AgentEventDto {
    AgentEventDto::ToolCall {
        tool_call_id: id.to_owned(),
        name: name.to_owned(),
        arguments: arguments.to_owned(),
    }
}

fn event_tool_result(id: &str, content: &str) -> AgentEventDto {
    AgentEventDto::ToolResult {
        tool_call_id: id.to_owned(),
        content: content.to_owned(),
    }
}

fn event_done(summary: &str) -> AgentEventDto {
    AgentEventDto::Done {
        summary: summary.to_owned(),
    }
}

fn event_error(message: &str) -> AgentEventDto {
    AgentEventDto::Error {
        message: message.to_owned(),
    }
}

fn event_progress(content: &str) -> AgentEventDto {
    AgentEventDto::Progress {
        content: content.to_owned(),
    }
}
