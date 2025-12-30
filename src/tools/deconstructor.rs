use crate::types::{Intent, Specification, MODEL};
use crate::error::{Result, ScribeError};
use crate::utilities::require_env;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use rig::providers::gemini::Client;
use serde::{Deserialize, Serialize};
use rig::completion::Prompt;
use rig::client::ProviderClient;
use rig::prelude::*; // Needed for .prompt() method

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
