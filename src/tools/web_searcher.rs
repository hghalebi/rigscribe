use crate::types::Webquery;
use crate::error::{Result, ScribeError};
use rig::completion::ToolDefinition;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serpscraper::get_markdown_for_query;

/// A tool for performing web searches to gather external information.
///
/// This tool uses the `serpscraper` library (wrapping an API like Serper.dev)
/// to fetch search results in Markdown format.
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
            description: "A research tool. Use this to find best practices, domain-specific knowledge, or to verify assumptions about the user's goal.".to_string(),
            parameters,
        }
    }
    async fn call(&self, args: Self::Args) -> Result<Self::Output> {
        tracing::info!("[Tool Calling]-> WebSearcher with args: {:?}", args);
        let api_key = std::env::var("SERPER_API_KEY").map_err(
            |e| 
                ScribeError::Config(format!("SERPER_API_KEY not set: {}", e))
        );
        let markdown = get_markdown_for_query(&args.query, &api_key?.to_string()).await;
        Ok(markdown.unwrap().to_string())
    }
}