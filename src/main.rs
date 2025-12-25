use std::path::{Path, PathBuf};

use rigscribe::utilities::save_artifacts;
use rigscribe::{Result, RigScribe, ScopeId};
use termimad::MadSkin;
/// Entry point: Orchestrates the transformation of user intent into a system prompt.
#[tokio::main]
async fn main() -> Result<()> {
    let cache_path = PathBuf::from("./prompts_perssitense_cache");
    let scribe = RigScribe::new(cache_path);
    let id = ScopeId(2026);
    // Input: The raw, often vague user intent.
    let raw_prompt = " Role: Chief Prompt Officer\n\
    Task: Review the draft for safety, clarity, and constraint compliance\n\
    Output: Only the final system prompt text";
    eprintln!("\n\nOptimizing ...\n\n");

    // Logic: Execute the multi-agent pipeline.
    // Note: The '2' argument is currently unused (placeholder).
    // It is reserved to maintain signature consistency for a future caching ID.
    let optimized_prompt = scribe.optimize_with_cache(raw_prompt, id).await?;

    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
