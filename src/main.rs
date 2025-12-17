use rigscribe::{Result, RigScribe};
use termimad::MadSkin;

#[tokio::main]
async fn main() -> Result<()> {
    /*//println!("Hello, world!");
    let scribe = RigScribe::from_env()?;
    eprintln!("\n\nOptimizing ...\n\n");
    let artifact = scribe.optimize("generate a function in python").await?;
    // rendering result in markdown format.
    let skin = MadSkin::default();
    skin.print_text(artifact.system_prompt.as_str());
    //dbg!(artifact.system_prompt);*/
    //
    let raw_prompt = "generate a function in python";
    eprintln!("\n\nOptimizing ...\n\n");
    let scribe = RigScribe::from_env()?;
    let optimized_prompt = scribe.optimize_agentic(raw_prompt, 2).await?;
    let skin = MadSkin::default();
    skin.print_text(optimized_prompt.system_prompt.as_str());

    Ok(())
}
