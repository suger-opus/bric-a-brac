use crate::{
    application::services::{
        agent_error::AgentError,
        prompt::build_system_prompt,
        tool_executor::ToolExecutor,
        tools::{read_tools, session_tools, write_tools},
    },
    infrastructure::clients::{Message, MetadataClient, OpenRouterClient, ToolDefinition},
};
use bric_a_brac_protos::{
    ai::{
        AgentDoneProto, AgentErrorProto, AgentEventProto, AgentTextProto, AgentToolCallProto,
        AgentToolResultProto,
    },
    common::GraphSchemaProto,
    metadata::SessionMessageProto,
};
use std::sync::Arc;
use tokio::sync::{mpsc, Semaphore};

const MAX_TOOL_ITERATIONS: usize = 200;

/// Maximum number of concurrent LLM calls across all agent sessions.
/// Prevents overwhelming the LLM provider (OpenRouter) with too many
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

    pub fn send_message(
        &self,
        session_id: String,
        user_message: String,
        tx: mpsc::Sender<AgentEventProto>,
    ) {
        let openrouter_client = self.openrouter_client.clone();
        let metadata_client = self.metadata_client.clone();
        let tool_executor = self.tool_executor.clone();
        let llm_semaphore = self.llm_semaphore.clone();

        tokio::spawn(async move {
            if let Err(err) = run_agent_loop(
                &openrouter_client,
                &metadata_client,
                &tool_executor,
                &llm_semaphore,
                &session_id,
                &user_message,
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
                let _ = tx.send(event_error(&format!("{err}"))).await;
            }
        });
    }
}

use std::error::Error as StdError;

#[tracing::instrument(
    level = "info",
    name = "agent.run",
    skip(openrouter_client, metadata_client, tool_executor, user_message, tx),
    fields(graph_id, user_role)
)]
async fn run_agent_loop(
    openrouter_client: &OpenRouterClient,
    metadata_client: &MetadataClient,
    tool_executor: &ToolExecutor,
    llm_semaphore: &Semaphore,
    session_id: &str,
    user_message: &str,
    tx: &mpsc::Sender<AgentEventProto>,
) -> Result<(), AgentError> {
    // 1. Load session
    let session =
        metadata_client
            .get_session(session_id)
            .await
            .map_err(|e| AgentError::SessionLoad {
                session_id: session_id.to_owned(),
                source: e,
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
        .map_err(|e| AgentError::MessagesLoad {
            session_id: session_id.to_owned(),
            source: e,
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
            .map_err(|e| AgentError::SchemaLoad {
                graph_id: graph_id.clone(),
                source: e,
            })?;

    let tools = build_tool_list(user_role);

    // 4. Build message history
    let mut messages = build_message_history(&schema, &existing_messages);
    messages.push(Message::user(user_message));

    // Track new messages for persistence
    let mut new_messages: Vec<bric_a_brac_protos::metadata::NewSessionMessageProto> =
        vec![bric_a_brac_protos::metadata::NewSessionMessageProto {
            role: "user".to_owned(),
            content: user_message.to_owned(),
            tool_calls: None,
            tool_call_id: None,
        }];

    // 5. Agent loop
    let mut iteration = 0;
    loop {
        if iteration >= MAX_TOOL_ITERATIONS {
            tracing::warn!("Maximum tool call limit reached");
            let _ = tx
                .send(event_error("Maximum tool call limit reached."))
                .await;
            break;
        }

        // Acquire semaphore permit before calling LLM (caps concurrent calls)
        let _permit = llm_semaphore
            .acquire()
            .await
            .map_err(|_| AgentError::Internal {
                message: "LLM semaphore closed".to_owned(),
            })?;

        // Call LLM
        let result = openrouter_client
            .chat_stream(messages.clone(), Some(tools.clone()))
            .await
            .map_err(|e| AgentError::LlmCall {
                iteration,
                source: e,
            })?;

        // Drop the permit immediately after the LLM call completes
        drop(_permit);

        // Stream text content
        if let Some(ref content) = result.content {
            if !content.is_empty() {
                let _ = tx.send(event_text(content)).await;
            }
        }

        // No tool calls → done
        if result.tool_calls.is_empty() {
            new_messages.push(bric_a_brac_protos::metadata::NewSessionMessageProto {
                role: "assistant".to_owned(),
                content: result.content.clone().unwrap_or_default(),
                tool_calls: None,
                tool_call_id: None,
            });

            let _ = tx
                .send(event_done(
                    result.content.as_deref().unwrap_or("Task completed."),
                ))
                .await;
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
            schema = metadata_client.get_schema(graph_id).await.map_err(|e| {
                AgentError::SchemaRefresh {
                    graph_id: graph_id.clone(),
                    source: e,
                }
            })?;

            messages[0] = Message::system(build_system_prompt(&schema));
        }

        iteration += 1;
    }

    // 6. Persist all new messages
    persist_messages(metadata_client, session_id, new_messages).await;

    tracing::info!(iterations = iteration, "Agent loop completed");
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

fn build_message_history(
    schema: &GraphSchemaProto,
    existing_messages: &[SessionMessageProto],
) -> Vec<Message> {
    let mut messages = vec![Message::system(build_system_prompt(schema))];

    for msg in existing_messages {
        let message = match msg.role.as_str() {
            "user" => Message::user(&msg.content),
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
        messages.push(message);
    }

    messages
}

async fn persist_messages(
    metadata_client: &MetadataClient,
    session_id: &str,
    messages: Vec<bric_a_brac_protos::metadata::NewSessionMessageProto>,
) {
    if messages.is_empty() {
        return;
    }
    if let Err(e) = metadata_client
        .append_session_messages(session_id, messages)
        .await
    {
        tracing::error!(error = %e, "Failed to persist session messages");
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
