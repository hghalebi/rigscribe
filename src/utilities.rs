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
///
/// # Examples
///
/// ```
/// use rigscribe::utilities::require_env;
/// use std::env;
///
/// // set_var is unsafe in multi-threaded tests
/// unsafe { env::set_var("TEST_VAR", "value"); }
/// assert_eq!(require_env("TEST_VAR").unwrap(), "value");
///
/// unsafe { env::remove_var("TEST_MISSING"); }
/// assert!(require_env("TEST_MISSING").is_err());
/// ```
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
///
/// # Examples
///
/// ```no_run
/// use rigscribe::{Artifact, utilities::save_artifacts};
/// use std::path::PathBuf;
///
/// #[tokio::main]
/// async fn main() {
///     let artifact = Artifact::new("prompt", "agent");
///     let path = PathBuf::from("/tmp/test_artifact.json");
///     save_artifacts(&path, &artifact).await.unwrap();
/// }
/// ```
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
///
/// # Examples
///
/// ```no_run
/// use rigscribe::utilities::read_artifact;
///
/// #[tokio::main]
/// async fn main() {
///     let artifact = read_artifact("/tmp/test_artifact.json").await.unwrap();
/// }
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_require_env_exists() {
        unsafe { env::set_var("EXISTS_KEY", "value"); }
        assert_eq!(require_env("EXISTS_KEY").unwrap(), "value");
    }

    #[test]
    fn test_require_env_missing() {
        unsafe { env::remove_var("MISSING_KEY"); }
        match require_env("MISSING_KEY") {
            Err(ScribeError::Config(msg)) => assert!(msg.contains("MISSING_KEY")),
            _ => panic!("Expected Config error"),
        }
    }

    #[tokio::test]
    async fn test_save_and_read_artifact() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_artifact_rw.json");
        let artifact = Artifact::new("sys_prompt", "agent_x");

        // Test Save
        save_artifacts(&file_path, &artifact).await.expect("Save failed");
        assert!(file_path.exists());

        // Test Read
        let loaded = read_artifact(&file_path).await.expect("Read failed");
        assert_eq!(loaded.system_prompt, "sys_prompt");
        assert_eq!(loaded.signed_by, "agent_x");

        // Cleanup
        let _ = fs::remove_file(file_path).await;
    }

    #[tokio::test]
    async fn test_save_artifact_appends_extension() {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("test_artifact_no_ext");
        let artifact = Artifact::new("A", "B");

        save_artifacts(&file_path, &artifact).await.unwrap();
        
        let expected_path = temp_dir.join("test_artifact_no_ext.json");
        assert!(expected_path.exists());

        let _ = fs::remove_file(expected_path).await;
    }

    #[tokio::test]
    async fn test_read_missing_file() {
        let path = Path::new("/non/existent/file.json");
        let res = read_artifact(path).await;
        assert!(matches!(res, Err(ScribeError::Config(_))));
    }
}