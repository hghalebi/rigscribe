use rigscribe::{RigScribe, ScopeId, Result};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Ensure API key is set
    if env::var("GEMINI_API_KEY").is_err() {
        eprintln!("Error: GEMINI_API_KEY environment variable is not set.");
        return Ok(())
    }

    // 1. Initialize RigScribe with a cache directory
    // This folder will store your optimized prompts as JSON files.
    let scribe = RigScribe::new("./.prompts_cache");

    // 2. Define your Intent
    let raw_request = "I want an AI assistant that helps users write Clap CLI tools in Rust.";
    
    // 3. Define a stable ScopeId
    // This ID (e.g., 101) represents this specific "CLI Assistant" feature.
    let feature_id = ScopeId(101);

    println!("Refining prompt for: '{}'...", raw_request);

    // 4. Optimize (or load from cache)
    // The first run will take a few seconds to 'think'.
    // Subsequent runs will be instant.
    match scribe.optimize_with_cache(raw_request, feature_id).await {
        Ok(artifact) => {
            println!("\n--- Optimized System Prompt ---\
");
            println!("{}", artifact.system_prompt);
        }
        Err(e) => {
            eprintln!("Error optimizing prompt: {}", e);
        }
    }
    
    Ok(())
}
