use rigscribe::{Result, RigScribe};
use termimad::MadSkin;

#[tokio::main]
async fn main() -> Result<()> {
    let raw_prompt = "generate a function in python";
    eprintln!("\n\nOptimizing ...\n\n");
    let scribe = RigScribe::from_env()?;
    let optimized_prompt = scribe.optimize_agentic(raw_prompt, 2).await?;
    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
