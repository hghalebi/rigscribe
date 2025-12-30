use crate::types::{Intent, Specification, MODEL};
use crate::error::{Result, ScribeError};
use crate::utilities::require_env;
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use rig::providers::gemini::Client;
use serde::{Deserialize, Serialize};
use rig::client::ProviderClient;
use rig::prelude::*; // Needed for .prompt() method

/// A tool that analyzes a raw user prompt to extract key constraints and goals.
///
/// This tool uses a specialized "Senior Solution Architect" agent to process the
/// [`Intent`] and produce a structured [`Specification`].
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
        tracing::info!("[Tool Calling]-> Deconstructor with args: {:?}", args);
        require_env("GEMINI_API_KEY")?;
        let client = Client::from_env();
        let architect = client
            .agent(MODEL)
            .preamble(
                "\n                Role: Senior Solution Architect\n\
                Task: Extract constraints and risks and main goal of given request\n\
                Output: A short bullet list, no prose\n                ",
            )
            .build();
        
        let mut stream = crate::agents::multi_turn_prompt(architect, args.text.clone(), Vec::new()).await;
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
        
        let spec_extractor = client.extractor::<Specification>(MODEL).build();
        let spec = spec_extractor.extract(full_response).await?;

        tracing::debug!("Deconstructor extracted spec: {:?}", spec);
        Ok(spec)
    }
}