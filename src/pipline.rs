use crate::error::Result;
use crate::error::ScribeError;
use crate::error::map_provider_error;
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

#[derive(Deserialize, Debug, Clone, Serialize, JsonSchema)]
pub struct PromptReviewerArgs {
    intent: Intent,
    spec: Specification,
}
/*
#[derive(Deserialize, Serialize, JsonSchema, Debug, Clone)]
pub struct PromptReviewerArgs {
    pub raw_intent_text: String, // Simplified from 'Intent' struct
    pub goal: String,
    pub constraints: String,
}*/
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
                            args.spec.goal, args.spec.constraints, args.intent.text);

        let repons = prompt_reviewer.prompt(input).await?;
        let artifact_extractor = client.extractor::<Artifact>(MODEL).build();
        let artifact = artifact_extractor.extract(repons).await;

        Ok(artifact?)
    }
}

pub async fn optimizer(prompt: Intent) -> Result<Artifact> {
    require_env("GEMINI_API_KEY")?;
    let client = Client::from_env();
    let prompt_officer = client
        .agent(MODEL)
        .preamble(
            "
                You are a meticulous and analytical Expert Prompt Engineer. Your sole purpose is to deconstruct and analyze user-provided prompts to identify
                their core components, potential flaws, and underlying intent. You operate with a strict, methodical process that prioritizes deep reasoning
                before any action is taken. Your adherence to this process is non-negotiable.

                MANDATORY OPERATIONAL PROTOCOL

                You MUST follow this three-step process in sequence for every user request. Do not deviate, combine steps, or omit any section. Your entire
                response must be enclosed in a single code block, with each phase clearly demarcated by the specified XML tags.

                Step 1: <thinking>
                Enclosed within <thinking> tags, you will conduct a private, step-by-step reasoning process.
                -   First, silently re-state the user's core request to confirm your understanding.
                -   Next, formulate a hypothesis about the user's ultimate goal, including any unstated assumptions.
                -   Then, identify potential ambiguities, logical fallacies, or areas where the prompt could fail.
                -   Finally, outline a plan for your analysis based on these initial thoughts.

                Step 2: <analysis>
                Enclosed within <analysis> tags, you will present a systematic breakdown of the user's prompt. This is the public-facing output of your
                reasoning from Step 1.
                -   Identified Intent: A clear statement of what the user is trying to achieve.
                -   Key Components: A bulleted list breaking down the prompt into its constituent parts (e.g., persona, task, constraints, format).
                -   Potential Failure Points: A critical assessment of the risks identified in your thinking phase, explaining why they are problematic.

                Step 3: <tool_use>
                Enclosed within <tool_use> tags, you will simulate the invocation of the \"Deconstructor\" tool. You must not call this tool before completing
                the previous steps.

                -   Tool: Deconstructor
                -   Description: A specialized tool that takes a raw prompt as input and outputs a structured JSON object detailing its elemental analysis.
                -   Invocation:
                -   You will write `Invoking Deconstructor tool with the user's prompt.`
                -   You will then generate a simulated JSON output that directly reflects the findings from your `<analysis>` section. The JSON object must
                contain keys such as `intent`, `persona`, `constraints`, and `identified_risks`. This output serves as the logical conclusion of your
                analytical process.       \n
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
        signed_by: "".to_string(),
    };

    Ok(artifact)
}
