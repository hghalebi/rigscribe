use rigscribe::{Result, RigScribe};
use termimad::MadSkin;
/// Entry point: Orchestrates the transformation of user intent into a system prompt.
#[tokio::main]
async fn main() -> Result<()> {
    // Input: The raw, often vague user intent.
    let raw_prompt = "generate a function in python";
    eprintln!("\n\nOptimizing ...\n\n");

    // Logic: Execute the multi-agent pipeline.
    // Note: The '2' argument is currently unused (placeholder).
    // It is reserved to maintain signature consistency for a future caching ID.
    let optimized_prompt = RigScribe::optimize_agentic(raw_prompt, 2).await?;

    // Render the resulting Markdown to the terminal.
    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
