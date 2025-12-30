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

    /// Defines the tool's schema for the LLM.
    ///
    /// # Arguments
    ///
    /// * `_prompt` - Unused context.
    ///
    /// # Examples
    ///
    /// ```
    /// use rig::tool::Tool;
    /// use rigscribe::tools::deconstructor::Deconstructor;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let tool = Deconstructor;
    ///     let def = tool.definition("".to_string()).await;
    ///     assert_eq!(def.name, "Deconstructor");
    /// }
    /// ```
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        let schema = schemars::schema_for!(Intent);
        let parameters = serde_json::to_value(schema).unwrap();
        ToolDefinition {
            name: "Deconstructor".to_string(),
            description: "this tools take a raw prompte and give back it Specification include goal and constrian".to_string(),
            parameters: parameters,
        }
    }

    /// Executes the tool by calling a secondary LLM agent.
    ///
    /// # Arguments
    ///
    /// * `args` - The user intent to analyze.
    ///
    /// # Errors
    ///
    /// Returns [`ScribeError::Config`] if `GEMINI_API_KEY` is missing.
    /// Returns [`ScribeError::Provider`] if the LLM call fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rig::tool::Tool;
    /// use rigscribe::{tools::deconstructor::Deconstructor, Intent};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let tool = Deconstructor;
    ///     let intent = Intent::new("Make a game").unwrap();
    ///     // Requires GEMINI_API_KEY
    ///     let spec = tool.call(intent).await.unwrap();
    ///     println!("Goal: {}", spec.goal);
    /// }
    /// ```
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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_deconstructor_definition() {
        let tool = Deconstructor;
        let def = tool.definition("".into()).await;
        assert_eq!(def.name, "Deconstructor");
        // Verify parameter schema includes 'text' field
        let params = def.parameters.to_string();
        assert!(params.contains("text"));
    }

    // TODO (UNTESTABLE): test_deconstructor_call
    // This method instantiates `Client::from_env()` internally, making it impossible
    // to mock the LLM provider without refactoring the code to accept an injected Client.
}