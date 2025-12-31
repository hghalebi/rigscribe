use std::path::PathBuf;

use rigscribe::{Result, RigScribe, ScopeId, logging};
use termimad::MadSkin;
use tracing::info;

/// Entry point: Orchestrates the transformation of user intent into a system prompt.
#[tokio::main]
async fn main() -> Result<()> {
    let _guard = logging::init_logging();

    // create caching path
    let cache_path = PathBuf::from("./.prompts_perssitense_cache");
    let scribe = RigScribe::new(cache_path);
    let id = ScopeId(2011);
    // Input: The raw, often vague user intent.
    let raw_prompt = "write a python fonction";
    info!("Starting prompt optimization process for: '{}'", raw_prompt);

    // Execute the multi-agent optimization pipeline.
    let optimized_prompt = scribe.optimize_with_cache(raw_prompt, id).await?;
    // Render the resulting Markdown artifact to the terminal.
    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
