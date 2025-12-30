use crate::types::{Intent, Specification, Artifact, MODEL};
use crate::error::{Result, ScribeError};
use crate::utilities::require_env;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use rig::providers::gemini::Client;
use crate::tools::web_searcher::WebSearcher;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use rig::client::ProviderClient;
use rig::prelude::*;

#[derive(Deserialize, Debug, Clone, Serialize, JsonSchema)]
pub struct PromptReviewerArgs {
    intent: Intent,
    spec: Specification,
}

#[derive(Serialize, Deserialize)]
pub struct PromptReviewer;

impl Tool for PromptReviewer {
    const NAME: &'static str = "PromptReviewer";

    type Error = ScribeError;
    type Args = PromptReviewerArgs;
    type Output = Artifact;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(PromptReviewerArgs);
        let parameters = serde_json::to_value(schema).unwrap();
        ToolDefinition {
            name: "PromptReviewer".to_string(),
            description: "this tools take a raw prompte it will evelaute that given promte wiuth its Specification include goal and constrian".to_string(),
            parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output> {
        tracing::info!("[Tool Calling]-> PromptReviewer with args: {:?}", args);
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let system_prompt_json = include_str!("../../data/prompt_officer.json");
        let artifact: Artifact = serde_json::from_str(system_prompt_json)
             .map_err(|e| ScribeError::Validation(format!("Failed to parse embedded prompt_officer.json: {}", e)))?;
        let system_prompt = artifact.system_prompt;
        let prompt_reviewer = client.agent(MODEL)
            .preamble(system_prompt.as_str())
            .tool(WebSearcher)
            .build();
        let input = format!("
        Critisize following prompt base on given property:
        Goal:\n{}\n\nConstraints:\n{}\n\nDraft:\n{}\n\n\
        Instruction: Be highly critical and pessimistic. 
        1. You MUST first use the 'WebSearcher' tool to research state-of-the-art prompt engineering techniques and best practices specifically for this type of task.
        2. Use the search results to find every deficit in the draft.
        3. Rewrite the prompt to be flawless.
        \n",
                            args.spec.goal, args.spec.constraints, args.intent.text);

        let mut stream = crate::agents::multi_turn_prompt(prompt_reviewer, input, Vec::new()).await;
        let mut full_response = String::new();
        while let Some(res) = futures::StreamExt::next(&mut stream).await {
             match res {
                Ok(text) => {
                     print!("{}", text.text);
                     use std::io::Write;
                     let _ = std::io::stdout().flush();
                     full_response.push_str(&text.text);
                }
                Err(e) => return Err(ScribeError::ProtocolViolation(e.to_string())),
            }
        }
        println!();

        let artifact_extractor = client.extractor::<Artifact>(MODEL).build();
        let artifact = artifact_extractor.extract(full_response).await?;

        tracing::debug!("PromptReviewer produced artifact: {:?}", artifact);
        Ok(artifact)
    }
}
