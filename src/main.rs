use std::path::PathBuf;

use rigscribe::{Result, RigScribe, ScopeId};
use termimad::MadSkin;
/// Entry point: Orchestrates the transformation of user intent into a system prompt.
#[tokio::main]
async fn main() -> Result<()> {
    // create caching path
    let cache_path = PathBuf::from("./.prompts_perssitense_cache");
    let scribe = RigScribe::new(cache_path);
    let id = ScopeId(2028);
    // Input: The raw, often vague user intent.
    let raw_prompt = "write a python fonction";
    eprintln!("\n\nOptimizing ...\n\n");

    // Execute the multi-agent optimization pipeline.
    let optimized_prompt = scribe.optimize_with_cache(raw_prompt, id).await?;
    // Render the resulting Markdown artifact to the terminal.
    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
