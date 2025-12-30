use crate::error::{Result, ScribeError};
use crate::types::{Artifact, Intent, MODEL};
use crate::tools::{deconstructor::Deconstructor, prompt_reviewer::PromptReviewer, web_searcher::WebSearcher};
use crate::utilities::require_env;
use rig::providers::gemini::Client;
use futures::StreamExt;
use rig::client::CompletionClient;
use crate::agents::multi_turn_prompt;
use rig::tool::Tool;

pub async fn optimizer(prompt: Intent) -> Result<Artifact> {
    require_env("GEMINI_API_KEY")?;
    let client = Client::new(require_env("GEMINI_API_KEY")?)?;
    let system_prompt_json = include_str!("../../data/optimizer.json");
    let artifact: Artifact = serde_json::from_str(system_prompt_json)
         .map_err(|e| ScribeError::Validation(format!("Failed to parse embedded optimizer.json: {}", e)))?;
    let system_prompt = artifact.system_prompt;

    // Log tool definitions for verbose output
    let deconstructor_def = Deconstructor.definition("".to_string()).await;
    tracing::info!("Tool Definition - Deconstructor: {:?}", deconstructor_def);

    let prompt_reviewer_def = PromptReviewer.definition("".to_string()).await;
    tracing::info!("Tool Definition - PromptReviewer: {:?}", prompt_reviewer_def);

    let web_searcher_def = WebSearcher.definition("".to_string()).await;
    tracing::info!("Tool Definition - WebSearcher: {:?}", web_searcher_def);

    let prompt_officer = client
        .agent(MODEL)
        .preamble(system_prompt.as_str())
        .tool(Deconstructor)
        .tool(PromptReviewer)
        .tool(WebSearcher)
        .build();

    let input = format!(
        "Follow this workflow to optimize the prompt:
            1. Use the Deconstructor tool to analyze the goal and constraints of: '{}'
            2. Use the PromptReviewer to check, you must research (using WebSearcher), and refine the draft.
            3. Finally, provide the optimized system prompt.

            Constraint: The final output must be the system prompt only, but you MUST use your tools first to arrive at that result.",
        prompt.text
    );
    let mut stream =multi_turn_prompt( prompt_officer,input,Vec::new()).await;

    tracing::info!("Starting optimization streaming...");
    let mut optimized_prompt = String::new();
    while let Some(res) = stream.next().await {
        match res {
            Ok(text) => {
                print!("{}", text.text);
                use std::io::Write;
                let _ = std::io::stdout().flush();
                optimized_prompt.push_str(&text.text);
            }
            Err(e) => {
                tracing::error!("Streaming error: {}", e);
                return Err(ScribeError::ProtocolViolation(e.to_string()));
            }
        }
    }
    println!();
    tracing::info!("Optimization complete. Final artifact length: {}", optimized_prompt.len());
    let artifact = Artifact {
        system_prompt: optimized_prompt,
        signed_by: "".to_string(),
    };

    Ok(artifact)
}
