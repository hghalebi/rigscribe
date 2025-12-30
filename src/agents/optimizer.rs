use crate::error::{Result, ScribeError, map_provider_error};
use crate::types::{Artifact, Intent, MODEL};
use crate::tools::{deconstructor::Deconstructor, prompt_reviewer::PromptReviewer, web_searcher::WebSearcher};
use crate::utilities::require_env;
use rig::providers::gemini::Client;
use rig::prelude::*;
use rig::completion::Prompt;

pub async fn optimizer(prompt: Intent) -> Result<Artifact> {
    require_env("GEMINI_API_KEY")?;
    let client = Client::new(require_env("GEMINI_API_KEY")?)?;
    let system_prompt_json = include_str!("../../data/optimizer.json");
    let artifact: Artifact = serde_json::from_str(system_prompt_json)
         .map_err(|e| ScribeError::Validation(format!("Failed to parse embedded optimizer.json: {}", e)))?;
    let system_prompt = artifact.system_prompt;

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
            2. Use the PromptReviewer to check and refine the draft.
            3. Use the WebSearcher to find the best practice related task/goal.
            4. Finally, provide the optimized system prompt.

            Constraint: The final output must be the system prompt only, but you MUST use your tools first to arrive at that result.",
        prompt.text
    );
    let optimized_prompt = prompt_officer
        .prompt(input)
        .await
        .map_err(map_provider_error)?;
    let artifact = Artifact {
        system_prompt: optimized_prompt,
        signed_by: "".to_string(),
    };

    Ok(artifact)
}
