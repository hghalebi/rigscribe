pub mod optimizer;

use futures::{Stream, StreamExt};
use rig::{
    OneOrMany,
    agent::Agent,
    completion::{self, CompletionError, CompletionModel, PromptError},
    message::{AssistantContent, Message, Text, ToolResultContent, UserContent},
    streaming::{StreamedAssistantContent, StreamingCompletion},
    tool::{ToolError, ToolSetError},
};
use std::pin::Pin;
use thiserror::Error;

/// Represents errors that can occur during streaming communication with an agent.
///
/// # Examples
///
/// ```
/// use rigscribe::agents::StreamingError;
/// use rig::completion::CompletionError;
///
/// let err = StreamingError::Completion(CompletionError::ResponseError("bad".into()));
/// assert!(format!("{}", err).contains("CompletionError"));
/// ```
#[derive(Debug, Error)]
pub enum StreamingError {
    /// Error during the completion process (e.g., network issue).
    #[error("CompletionError: {0}")]
    Completion(#[from] CompletionError),
    /// Error related to prompt construction or validation.
    #[error("PromptError: {0}")]
    Prompt(#[from] Box<PromptError>),
    /// Error occurred while executing a tool.
    #[error("ToolSetError: {0}")]
    Tool(#[from] ToolSetError),
}

/// A type alias for a pinned, boxed stream of text chunks or errors.
pub type StreamingResult = Pin<Box<dyn Stream<Item = std::result::Result<Text, StreamingError>> + Send>>;

/// Manages a multi-turn conversation with an agent, handling tool calls and streaming responses.
///
/// This function executes a loop where it:
/// 1. Sends the current prompt and chat history to the agent.
/// 2. Streams the response back to the caller.
/// 3. Detects and executes tool calls.
/// 4. Appends tool results to the chat history and continues the conversation if necessary.
///
/// # Arguments
///
/// * `agent` - The configured Rig agent.
/// * `prompt` - The initial message to start or continue the conversation.
/// * `chat_history` - A vector of previous messages to maintain context.
///
/// # Returns
///
/// A `StreamingResult` that yields text chunks as they are generated.
///
/// # Examples
///
/// ```no_run
/// // This example is hypothetical as it requires a configured Agent which needs API keys.
/// ```
pub async fn multi_turn_prompt<M>(
    agent: Agent<M>,
    prompt: impl Into<Message> + Send,
    mut chat_history: Vec<completion::Message>,
) -> StreamingResult
where
    M: CompletionModel + 'static,
    <M as CompletionModel>::StreamingResponse: std::marker::Send,
{
    let prompt: Message = prompt.into();

    (Box::pin(async_stream::stream! {
        let mut current_prompt = prompt;
        let mut did_call_tool = false;

        'outer: loop {
            let mut stream = agent
                .stream_completion(current_prompt.clone(), chat_history.clone())
                .await?
                .stream()
                .await?;

            chat_history.push(current_prompt.clone());

            let mut tool_calls = vec![];
            let mut tool_results = vec![];

            while let Some(content) = stream.next().await {
                match content {
                    Ok(StreamedAssistantContent::Text(text)) => {
                        yield Ok(Text { text: text.text });
                        did_call_tool = false;
                    },
                    Ok(StreamedAssistantContent::ToolCall(tool_call)) => {
                        let tool_result =
                            agent.tool_server_handle.call_tool(&tool_call.function.name, &tool_call.function.arguments.to_string()).await
                            .map_err(|x| StreamingError::Tool(ToolSetError::ToolCallError(ToolError::ToolCallError(x.into()))))?;

                        tracing::info!(
                            tool = tool_call.function.name,
                            args = tool_call.function.arguments.to_string(),
                            result = tool_result,
                            "Tool executed"
                        );

                        let tool_call_msg = AssistantContent::ToolCall(tool_call.clone());

                        tool_calls.push(tool_call_msg);
                        tool_results.push((tool_call.id, tool_call.call_id, tool_result));

                        did_call_tool = true;
                        // break;
                    },
                    Ok(StreamedAssistantContent::Reasoning(rig::message::Reasoning { reasoning, .. })) => {
                        if !reasoning.is_empty() {
                            yield Ok(Text { text: reasoning.first().unwrap().to_owned() });
                        }
                        did_call_tool = false;
                    },
                    Ok(_) => {
                        // do nothing here as we don't need to accumulate token usage
                    }
                    Err(e) => {
                        yield Err(e.into());
                        break 'outer;
                    }
                }
            }

            // Add (parallel) tool calls to chat history
            if !tool_calls.is_empty() {
                chat_history.push(Message::Assistant {
                    id: None,
                    content: OneOrMany::many(tool_calls).expect("Impossible EmptyListError"),
                });
            }

            // Add tool results to chat history
            for (id, call_id, tool_result) in tool_results {
                if let Some(call_id) = call_id {
                    chat_history.push(Message::User {
                        content: OneOrMany::one(UserContent::tool_result_with_call_id(
                            id,
                            call_id,
                            OneOrMany::one(ToolResultContent::text(tool_result)),
                        )),
                    });
                } else {
                    chat_history.push(Message::User {
                        content: OneOrMany::one(UserContent::tool_result(
                            id,
                            OneOrMany::one(ToolResultContent::text(tool_result)),
                        )),
                    });

                }

            }

            // Set the current prompt to the last message in the chat history
            current_prompt = match chat_history.pop() {
                Some(prompt) => prompt,
                None => unreachable!("Chat history should never be empty at this point"),
            };

            if !did_call_tool {
                break;
            }
        }

    })) as _
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streaming_error_fmt() {
        let err = StreamingError::Completion(completion::CompletionError::ResponseError("Test error".into()));
        assert!(format!("{}", err).contains("CompletionError"));
    }

    // TODO (UNTESTABLE): test_multi_turn_prompt
    // Requires a mock Agent<M> which is hard to construct without rig::providers::Mock (which isn't standard here).
}
