use crate::{
    application::services::{
        agent_error::AgentError,
        chunking::chunk_user_message,
        prompt::build_system_prompt,
        tool_executor::ToolExecutor,
        tools::{read_tools, session_tools, write_tools},
    },
    infrastructure::clients::{Message, MetadataClient, OpenRouterClient, ToolDefinition},
};
use bric_a_brac_protos::{
    ai::{
        AgentDoneProto, AgentErrorProto, AgentEventProto, AgentProgressProto, AgentTextProto,
        AgentToolCallProto, AgentToolResultProto,
    },
    common::GraphSchemaProto,
    metadata::SessionMessageProto,
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
    tool_executor: ToolExecutor,
    llm_semaphore: Arc<Semaphore>,
}

impl AgentService {
    pub fn new(
        openrouter_client: OpenRouterClient,
        metadata_client: MetadataClient,
        tool_executor: ToolExecutor,
    ) -> Self {
        Self {
            openrouter_client,
            metadata_client,
            tool_executor,
            llm_semaphore: Arc::new(Semaphore::new(MAX_CONCURRENT_LLM_CALLS)),
        }
    }

    // TODO: tracing
    pub fn send_message(
        &self,
        session_id: String,
        user_message: String,
        document_id: Option<String>,
        tx: mpsc::Sender<AgentEventProto>,
    ) {
        let openrouter_client = self.openrouter_client.clone();
        let metadata_client = self.metadata_client.clone();
        let tool_executor = self.tool_executor.clone();
        let llm_semaphore = Arc::clone(&self.llm_semaphore);

        tokio::spawn(async move {
            if let Err(err) = run_agent_loop(
                &openrouter_client,
                &metadata_client,
                &tool_executor,
                &llm_semaphore,
                &session_id,
                &user_message,
                document_id.as_deref(),
                &tx,
            )
            .await
            {
                tracing::error!(
                    error = %err,
                    error.source = err.source().map(tracing::field::display),
                    session_id,
                    "Agent loop failed"
                );
                #[allow(clippy::let_underscore_must_use)]
                let _ = tx.send(event_error(&format!("{err}"))).await;
            }
        });
    }
}

// TODO: clean this and split
#[tracing::instrument(
    level = "info",
    name = "agent.run",
    skip(
        openrouter_client,
        metadata_client,
        tool_executor,
        user_message,
        document_id,
        tx
    ),
    fields(graph_id, user_role),
    err
)]
#[allow(clippy::let_underscore_must_use)]
async fn run_agent_loop(
    openrouter_client: &OpenRouterClient,
    metadata_client: &MetadataClient,
    tool_executor: &ToolExecutor,
    llm_semaphore: &Semaphore,
    session_id: &str,
    user_message: &str,
    document_id: Option<&str>,
    tx: &mpsc::Sender<AgentEventProto>,
) -> Result<(), AgentError> {
    // 1. Load session
    let session =
        metadata_client
            .get_session(session_id)
            .await
            .map_err(|err| AgentError::SessionLoad {
                session_id: session_id.to_owned(),
                source: err,
            })?;

    let graph_id = &session.graph_id;
    let user_role = &session.role;

    // Record dynamic span fields
    tracing::Span::current().record("graph_id", graph_id.as_str());
    tracing::Span::current().record("user_role", user_role.as_str());

    // 2. Load existing messages
    let existing_messages = metadata_client
        .get_session_messages(session_id)
        .await
        .map_err(|err| AgentError::MessagesLoad {
            session_id: session_id.to_owned(),
            source: err,
        })?;

    tracing::debug!(
        existing_message_count = existing_messages.len(),
        "Loaded session context"
    );

    // 3. Load schema + build tool list
    let mut schema =
        metadata_client
            .get_schema(graph_id)
            .await
            .map_err(|err| AgentError::SchemaLoad {
                graph_id: graph_id.clone(),
                source: err,
            })?;

    let tools = build_tool_list(user_role);

    // 4. Build message history (without new user message yet)
    let mut messages = build_message_history(&schema, &existing_messages);

    // Track new messages for persistence
    let mut new_messages: Vec<bric_a_brac_protos::metadata::NewSessionMessageProto> = Vec::new();

    // 5. If a document was attached, load its content and build the combined message
    let effective_message = if let Some(doc_id) = document_id {
        let doc = metadata_client
            .get_session_document(doc_id)
            .await
            .map_err(|err| AgentError::Internal {
                message: format!("Failed to load document {doc_id}: {err}"),
                source: Some(Box::new(err)),
            })?;

        // Build the combined format that the chunker expects
        let doc_content = format!("[Document content]\n{}", doc.content);
        if user_message.is_empty() {
            doc_content
        } else {
            format!("{doc_content}\n\n[User message]\n{user_message}")
        }
    } else {
        user_message.to_owned()
    };

    // 6. Chunk the document if it's large
    let chunks = chunk_user_message(&effective_message);
    let total_chunks = chunks.len();

    if total_chunks > 1 {
        tracing::info!(total_chunks, "Document split into chunks");
    }

    // 7. Process each chunk
    let mut iteration = 0;
    let is_multi_chunk = total_chunks > 1;

    for (chunk_idx, chunk_content) in chunks.iter().enumerate() {
        // Progress indicator for multi-chunk documents
        if is_multi_chunk {
            let progress = format!(
                "Document split into {} parts — extracting part {}…",
                total_chunks,
                chunk_idx + 1,
            );
            let _ = tx.send(event_progress(&progress)).await;
        }

        // Add chunk as user message to LLM context
        messages.push(Message::user(chunk_content));

        // Persist the user message. For the original message (chunk 0 or single),
        // store the user's text + document_id. For subsequent chunks, store the
        // chunk content with chunk_index.
        if !is_multi_chunk {
            // Single message (no chunking) — persist as-is
            let persisted_content = if document_id.is_some() {
                user_message.to_owned()
            } else {
                chunk_content.clone()
            };
            persist_messages(
                metadata_client,
                session_id,
                vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "user".to_owned(),
                    content: persisted_content,
                    tool_calls: None,
                    tool_call_id: None,
                    document_id: document_id.map(String::from),
                    document_name: None,
                    chunk_index: None,
                }],
            )
            .await;
        } else if chunk_idx == 0 {
            // First chunk of multi-chunk — persist the user's text as the original message
            persist_messages(
                metadata_client,
                session_id,
                vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "user".to_owned(),
                    content: user_message.to_owned(),
                    tool_calls: None,
                    tool_call_id: None,
                    document_id: document_id.map(String::from),
                    document_name: None,
                    chunk_index: None,
                }],
            )
            .await;
            // Then persist the actual chunk content
            persist_messages(
                metadata_client,
                session_id,
                vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "user".to_owned(),
                    content: chunk_content.clone(),
                    tool_calls: None,
                    tool_call_id: None,
                    document_id: document_id.map(String::from),
                    document_name: None,
                    chunk_index: Some(i32::try_from(chunk_idx + 1).unwrap_or_default()),
                }],
            )
            .await;
        } else {
            // Subsequent chunks — persist with chunk_index
            persist_messages(
                metadata_client,
                session_id,
                vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "user".to_owned(),
                    content: chunk_content.clone(),
                    tool_calls: None,
                    tool_call_id: None,
                    document_id: document_id.map(String::from),
                    document_name: None,
                    chunk_index: Some(i32::try_from(chunk_idx + 1).unwrap_or_default()),
                }],
            )
            .await;
        }

        // Inner agent loop: LLM calls + tool execution for this chunk
        loop {
            if iteration >= MAX_TOOL_ITERATIONS {
                tracing::warn!("Maximum tool call limit reached");
                let _ = tx
                    .send(event_error("Maximum tool call limit reached."))
                    .await;
                break;
            }

            let permit = llm_semaphore
                .acquire()
                .await
                .map_err(|err| AgentError::Internal {
                    message: "LLM semaphore closed".to_owned(),
                    source: Some(Box::new(err)),
                })?;

            let result = openrouter_client
                .chat_stream(messages.clone(), Some(tools.clone()))
                .await
                .map_err(|err| AgentError::LlmCall {
                    iteration,
                    source: err,
                })?;

            drop(permit);

            // Stream text to user only for single-chunk messages.
            // For multi-chunk, intermediate text is kept in history silently.
            if !is_multi_chunk {
                if let Some(ref content) = result.content {
                    if !content.is_empty() {
                        let _ = tx.send(event_text(content)).await;
                    }
                }
            }

            // No tool calls → done with this chunk
            if result.tool_calls.is_empty() {
                let content = result.content.clone().unwrap_or_default();
                // Keep in LLM context for cross-chunk continuity
                messages.push(Message::assistant(Some(content.clone()), None));

                // Always persist assistant responses
                new_messages.push(bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "assistant".to_owned(),
                    content,
                    tool_calls: None,
                    tool_call_id: None,
                    document_id: None,
                    document_name: None,
                    chunk_index: None,
                });

                if !is_multi_chunk {
                    let _ = tx
                        .send(event_done(
                            result.content.as_deref().unwrap_or("Task completed."),
                        ))
                        .await;
                }

                break;
            }

            // Append assistant message with tool calls to history
            let tool_calls_json = serde_json::to_string(&result.tool_calls).ok();
            messages.push(Message::assistant(
                result.content.clone(),
                Some(result.tool_calls.clone()),
            ));
            new_messages.push(bric_a_brac_protos::metadata::NewSessionMessageProto {
                role: "assistant".to_owned(),
                content: result.content.unwrap_or_default(),
                tool_calls: tool_calls_json,
                tool_call_id: None,
                document_id: None,
                document_name: None,
                chunk_index: None,
            });

            // Execute tool calls
            let mut schema_changed = false;
            for tool_call in &result.tool_calls {
                let _ = tx
                    .send(event_tool_call(
                        &tool_call.id,
                        &tool_call.function.name,
                        &tool_call.function.arguments,
                    ))
                    .await;

                let tool_result = tool_executor
                    .execute(
                        &tool_call.function.name,
                        &tool_call.function.arguments,
                        graph_id,
                        session_id,
                        &schema,
                        user_role,
                    )
                    .await;

                let _ = tx
                    .send(event_tool_result(&tool_call.id, &tool_result.content))
                    .await;

                // Add tool response to message history
                messages.push(Message::tool(tool_call.id.clone(), &tool_result.content));
                new_messages.push(bric_a_brac_protos::metadata::NewSessionMessageProto {
                    role: "tool".to_owned(),
                    content: tool_result.content.clone(),
                    tool_calls: None,
                    tool_call_id: Some(tool_call.id.clone()),
                    document_id: None,
                    document_name: None,
                    chunk_index: None,
                });

                if tool_result.schema_changed {
                    schema_changed = true;
                }

                if tool_result.is_done {
                    persist_messages(metadata_client, session_id, new_messages).await;
                    let _ = tx.send(event_done(&tool_result.content)).await;
                    return Ok(());
                }
            }

            // Refresh schema if any tool modified it
            if schema_changed {
                schema = metadata_client.get_schema(graph_id).await.map_err(|err| {
                    AgentError::SchemaRefresh {
                        graph_id: graph_id.clone(),
                        source: err,
                    }
                })?;

                if let Some(first) = messages.get_mut(0) {
                    *first = Message::system(build_system_prompt(&schema));
                } else {
                    messages.insert(0, Message::system(build_system_prompt(&schema)));
                }
            }

            iteration += 1;
        }

        // Persist messages accumulated during this chunk
        let batch = std::mem::take(&mut new_messages);
        persist_messages(metadata_client, session_id, batch).await;
    }

    // 7. Final summary for multi-chunk documents
    if is_multi_chunk {
        let _ = tx
            .send(event_progress("Finalising — preparing summary…"))
            .await;

        messages.push(Message::user(
            "All parts of the document have been processed. \
             Provide a concise summary of everything you extracted and stored in the graph.",
        ));

        let permit = llm_semaphore
            .acquire()
            .await
            .map_err(|err| AgentError::Internal {
                message: "LLM semaphore closed".to_owned(),
                source: Some(Box::new(err)),
            })?;

        let summary_result = openrouter_client
            .chat_stream(messages.clone(), None)
            .await
            .map_err(|err| AgentError::LlmCall {
                iteration,
                source: err,
            })?;

        drop(permit);

        let summary = summary_result
            .content
            .unwrap_or_else(|| "Document processing complete.".to_owned());
        let _ = tx.send(event_text(&summary)).await;
        let _ = tx.send(event_done(&summary)).await;

        persist_messages(
            metadata_client,
            session_id,
            vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
                role: "assistant".to_owned(),
                content: summary,
                tool_calls: None,
                tool_call_id: None,
                document_id: None,
                document_name: None,
                chunk_index: None,
            }],
        )
        .await;
    }

    // 8. Persist any remaining messages (single-chunk case)
    persist_messages(metadata_client, session_id, new_messages).await;

    tracing::info!(
        iterations = iteration,
        chunks = total_chunks,
        "Agent loop completed"
    );
    Ok(())
}

fn build_tool_list(role: &str) -> Vec<ToolDefinition> {
    let mut tools = read_tools();
    match role {
        "Owner" | "Admin" | "Editor" => tools.extend(write_tools()),
        _ => {}
    }
    tools.extend(session_tools());
    tools
}

/// Maximum context window budget in characters (~125K tokens at ~4 chars/token).
/// GPT-4.1 supports 1M tokens but quality degrades on very long contexts.
const MAX_CONTEXT_CHARS: usize = 500_000;

fn build_message_history(
    schema: &GraphSchemaProto,
    existing_messages: &[SessionMessageProto],
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
        let message = match msg.role.as_str() {
            "user" => {
                // Chunk user messages → compressed reference
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
            "assistant" => {
                let tool_calls = msg
                    .tool_calls
                    .as_ref()
                    .and_then(|tc| serde_json::from_str(tc).ok());
                Message::assistant(Some(msg.content.clone()), tool_calls)
            }
            "tool" => Message::tool(msg.tool_call_id.clone().unwrap_or_default(), &msg.content),
            _ => continue,
        };
        total_chars += message_char_size(&message);
        history.push(message);
    }

    // Second pass: if still over budget, collapse old tool sequences
    if total_chars > budget {
        total_chars = 0;
        let mut trimmed: Vec<Message> = Vec::with_capacity(history.len());

        // We walk from the end to find how many recent messages fit
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

        // Always keep the first user message (session intent)
        if keep_from > 0 && !history.is_empty() {
            if let Some(first) = history.first() {
                trimmed.push(first.clone());
                total_chars += message_char_size(first);
            }

            let dropped = keep_from - 1; // minus the first message we kept
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
        let _ = total_chars; // suppress unused warning
    }

    messages.extend(history);
    messages
}

fn message_char_size(msg: &Message) -> usize {
    msg.content.as_ref().map_or(0, std::string::String::len)
}

async fn persist_messages(
    metadata_client: &MetadataClient,
    session_id: &str,
    messages: Vec<bric_a_brac_protos::metadata::NewSessionMessageProto>,
) {
    if messages.is_empty() {
        return;
    }
    if let Err(err) = metadata_client
        .append_session_messages(session_id, messages)
        .await
    {
        tracing::error!(
            error = ?err,
            "Failed to persist session messages"
        );
    }
}

// --- Event constructors ---

fn event_text(content: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(bric_a_brac_protos::ai::agent_event_proto::Event::Text(
            AgentTextProto {
                content: content.to_owned(),
            },
        )),
    }
}

fn event_tool_call(id: &str, name: &str, arguments: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(bric_a_brac_protos::ai::agent_event_proto::Event::ToolCall(
            AgentToolCallProto {
                tool_call_id: id.to_owned(),
                name: name.to_owned(),
                arguments: arguments.to_owned(),
            },
        )),
    }
}

fn event_tool_result(id: &str, content: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(
            bric_a_brac_protos::ai::agent_event_proto::Event::ToolResult(AgentToolResultProto {
                tool_call_id: id.to_owned(),
                content: content.to_owned(),
            }),
        ),
    }
}

fn event_done(summary: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(bric_a_brac_protos::ai::agent_event_proto::Event::Done(
            AgentDoneProto {
                summary: summary.to_owned(),
            },
        )),
    }
}

fn event_error(message: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(bric_a_brac_protos::ai::agent_event_proto::Event::Error(
            AgentErrorProto {
                message: message.to_owned(),
            },
        )),
    }
}

fn event_progress(content: &str) -> AgentEventProto {
    AgentEventProto {
        event: Some(bric_a_brac_protos::ai::agent_event_proto::Event::Progress(
            AgentProgressProto {
                content: content.to_owned(),
            },
        )),
    }
}
