use std::path::Path;

use tokio::fs;

use crate::{
    Artifact,
    error::{Result, ScribeError},
};

/// Retrieves an environment variable or returns a configuration error.
///
/// # Arguments
///
/// * `name` - The name of the environment variable.
///
/// # Errors
///
/// Returns [`ScribeError::Config`] if the variable is not set.
pub fn require_env(name: &str) -> Result<String> {
    match std::env::var(name) {
        Ok(val) => Ok(val),
        Err(_) => Err(ScribeError::Config(format!("{name} is missing!"))),
    }
}

/// Saves an [`Artifact`] to disk as a JSON file.
///
/// If the provided path does not have a `.json` extension, it will be appended.
/// The parent directory is created if it does not exist.
///
/// # Arguments
///
/// * `p` - The path to save the artifact to.
/// * `artifact` - The artifact to serialize and save.
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

/// Reads and deserializes an [`Artifact`] from a JSON file.
///
/// # Arguments
///
/// * `path` - The path to the JSON file.
///
/// # Errors
///
/// Returns [`ScribeError::Config`] if the file cannot be read.
/// Returns [`ScribeError::Validation`] if deserialization fails.
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