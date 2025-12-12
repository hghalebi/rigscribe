use rigscribe::{Result, RigScribe};
#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");
    let scribe = RigScribe::from_env()?;
    let artifact = scribe.optimize("generate fonctions in python").await?;
    dbg!(artifact.system_prompt);
    Ok(())
}
