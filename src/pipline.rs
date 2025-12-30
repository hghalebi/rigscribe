use crate::error::map_provider_error;
use crate::error::Result;
use crate::error::ScribeError;
pub use crate::types::{Artifact, Intent, Specification};
use crate::types::{Webquery, MODEL};
use crate::utilities::read_artifact;
use crate::utilities::require_env;
use rig::completion::Prompt;

use rig::prelude::*;
use rig::providers::gemini::Client;
use rig::{completion::ToolDefinition, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serpscraper::get_markdown_for_query;

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
        println!("[Tool Calling]-> PromptReviewer!");
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let system_prompt = read_artifact("data/prompt_officer.json")
            .await
            .unwrap()
            .system_prompt;
        let prompt_reviewer = client.agent(MODEL).preamble(system_prompt.as_str()).build();
        let input = format!("
        Critisize following prompt base on given property:
        Goal:\n{}\n\nConstraints:\n{}\n\nDraft:\n{}\n\n\
        Instruction: Be highly cretical and persimiste and find every defit or any point which could be better. and use all best practice and if needed use websearch.  \n",
                            args.spec.goal, args.spec.constraints, args.intent.text);

        let repons = prompt_reviewer.prompt(input).await?;
        let artifact_extractor = client.extractor::<Artifact>(MODEL).build();
        let artifact = artifact_extractor.extract(repons).await;

        Ok(artifact?)
    }
}

#[derive(Serialize, Deserialize)]
pub struct WebSearcher;
impl Tool for WebSearcher {
    const NAME: &'static str = "WebSearcher";
    type Error = ScribeError;
    type Args = Webquery;
    type Output = String;
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(Webquery);
        let parameters = serde_json::to_value(schema).unwrap();
        ToolDefinition {
            name: "WebSearcher".to_string(),
            description: "this tools query  will search on web and retern result in one string".to_string(),
            parameters,
        }
    }
    async fn call(&self, args: Self::Args) -> Result<Self::Output> {
        println!("[Tool Calling]-> WebSearcher");
        let api_key = std::env::var("SERPER_API_KEY").map_err(
            |e| 
                ScribeError::Config(format!("SERPER_API_KEY not set: {}", e))
        );
        let markdown = get_markdown_for_query(&args.query, &api_key?.to_string()).await;
        Ok(markdown.unwrap().to_string())
    }
}
pub async fn optimizer(prompt: Intent) -> Result<Artifact> {
    require_env("GEMINI_API_KEY")?;
    let client = Client::from_env();
    let system_prompt = read_artifact("data/optimizer.json")
        .await
        .unwrap()
        .system_prompt;

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
        .multi_turn(10)
        .await
        .map_err(map_provider_error)?;
    let artifact = Artifact {
        system_prompt: optimized_prompt,
        signed_by: "".to_string(),
    };

    Ok(artifact)
}
