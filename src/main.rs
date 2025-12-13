use rigscribe::{Result, RigScribe};
use termimad::MadSkin;
#[tokio::main]
async fn main() -> Result<()> {
    //println!("Hello, world!");
    let scribe = RigScribe::from_env()?;
    let artifact = scribe.optimize("generate a function in python").await?;
    // rendering result in markdown format.
    let skin = MadSkin::default();
    skin.print_text(artifact.system_prompt.as_str());
    //dbg!(artifact.system_prompt);
    Ok(())
}
