use std::path::Path;

use tokio::fs;

use crate::{
    Artifact,
    error::{Result, ScribeError},
};

pub fn require_env(name: &str) -> Result<()> {
    if std::env::var(name).is_ok() {
        Ok(())
    } else {
        Err(ScribeError::Config(format!("{name} is missing!")))
    }
}

pub async fn save_artifacts<P: AsRef<Path>>(p: P, artifact: &Artifact) -> Result<()> {
    let mut path = p.as_ref().to_path_buf();
    // check for  json extention

    if path.extension().and_then(|s| s.to_str()) != Some("json") {
        path.set_extension("json");
    }

    // ensure directory existe asynronincly
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await.map_err(|e| {
            ScribeError::Config(format!(
                "Failed to create {:?} directory with this error: {}",
                parent, e
            ))
        })?;
    }
    // serilize content
    let content = serde_json::to_string_pretty(artifact)
        .map_err(|e| ScribeError::Validation(format!("Failed to serlized artifact: {}", e)))?;

    // write to file
    fs::write(&path, content)
        .await
        .map_err(|e| ScribeError::Config(format!("failed to write to file {:?} \n {}", path, e)))?;

    Ok(())
}

pub async fn read_artifact<P: AsRef<Path>>(path: P) -> Result<Artifact> {
    let path = path.as_ref();
    let content = fs::read(path)
        .await
        .map_err(|e| ScribeError::Config(format!("Failed to read file: {:?}\n {:?} ", path, e)))?;

    let artifact: Artifact = serde_json::from_slice(&content).map_err(|e| {
        ScribeError::Validation(format!(
            "Failed to parse Artifact from :{:?}\n{:?}",
            path, e
        ))
    })?;
    Ok(artifact)
}
