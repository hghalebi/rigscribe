use crate::error::map_provider_error;
use crate::error::Result;
use crate::error::ScribeError;
use crate::types::MODEL;
pub use crate::types::{Artifact, Intent, Specification};
use crate::utilities::require_env;
use rig::completion::Prompt;

use rig::prelude::*;
use rig::providers::gemini::Client;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Deconstructor;
impl Tool for Deconstructor {
    const NAME: &'static str = "Deconstructor";

    type Error = ScribeError;
    type Args = Intent;
    type Output = Specification;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(Intent);
        let parameters = serde_json::to_value(schema).unwrap();
        ToolDefinition {
            name: "Deconstructor".to_string(),
            description: "this tools take a raw prompte and give back it Specification include goal and constrian".to_string(),
            parameters: parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output> {
        println!("[Tool Calling]-> Deconstructor!");
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let architect = client
            .agent(MODEL)
            .preamble(
                "
                Role: Senior Solution Architect\n\
                Task: Extract constraints and risks and main goal of given request\n\
                Output: A short bullet list, no prose
                ",
            )
            .build();
        let repons = architect.prompt(args.text.clone()).await?;
        let spec_extractor = client.extractor::<Specification>(MODEL).build();
        let spec = spec_extractor.extract(repons).await;

        Ok(spec?)
    }
}

/*#[derive(Deserialize, Debug, Clone, Serialize, JsonSchema)]
pub struct PromptOperationArgs {
    intent: Intent,
    spec: Specification,
}*/

#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct PromptReviewerArgs {
    pub raw_intent_text: String, // Simplified from 'Intent' struct
    pub goal: String,
    pub constraints: String,
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
            parameters: parameters,
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output> {
        println!("[Tool Calling]-> PromptReviewer!");
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let prompt_reviewer = client
            .agent(MODEL)
            .preamble(
                "
                Role: Chief Prompt Officer\n\
                Task: Review the draft for safety, clarity, and constraint compliance\n\
                Output: Only the final system prompt text
                ",
            )
            .build();
        let input = format!("
        Critisize following prompt base on given property:
        Goal:\n{}\n\nConstraints:\n{}\n\nDraft:\n{}\n\n\
        Instruction: Be highly cretical and persimiste and find every defit or any point which could be better. and use all best practice and if needed use websearch.  \n",
                            args.goal, args.constraints, args.raw_intent_text);

        let repons = prompt_reviewer.prompt(input).await?;
        let artifact_extractor = client.extractor::<Artifact>(MODEL).build();
        let artifact = artifact_extractor.extract(repons).await;

        Ok(artifact?)
    }
}

pub async fn optimizer(prompt: Intent, id: i128) -> Result<Artifact> {
    require_env("GEMINI_API_KEY")?;
    let client = Client::from_env();
    let prompt_officer = client
        .agent(MODEL)
        .preamble(
            "
            You are an expert prompt engineer.
            IMPORTANT: You must think step-by-step before using any tool.
            First, analyze the user's intent, then use the Deconstructor tool.
            Do not skip the reasoning phase.\n
            ",
        )
        .tool(Deconstructor)
        .tool(PromptReviewer)
        .build();

    let input = format!(
        "Follow this workflow to optimize the prompt:
        1. Use the Deconstructor tool to analyze the goal and constraints of: '{}'
        2. Use the PromptReviewer to check and refine the draft.
        3. Finally, provide the optimized system prompt.

        Constraint: The final output must be the system prompt only, but you MUST use your tools first to arrive at that result.",
        prompt.text
    );
    let optimized_prompt = prompt_officer
        .prompt(input)
        .multi_turn(10)
        .await
        .map_err(map_provider_error)?;
    let artifact = Artifact {
        system_prompt: optimized_prompt,
        signed_by: id.to_string(),
    };

    Ok(artifact)
}
