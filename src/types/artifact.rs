use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Represents the final output of the optimization pipeline.
///
/// An `Artifact` contains the polished system prompt ready for use, along with
/// a signature indicating which agent or process finalized it.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Artifact {
    /// The optimized system prompt.
    pub system_prompt: String,
    /// The name or identifier of the agent that produced this artifact.
    pub signed_by: String,
}

impl Artifact {
    /// Creates a new `Artifact`.
    ///
    /// # Arguments
    ///
    /// * `system_prompt` - The generated system instructions.
    /// * `signed_by` - The authoring agent's ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use rigscribe::Artifact;
    ///
    /// let artifact = Artifact::new("You are a helpful assistant.", "Agent-007");
    /// assert_eq!(artifact.system_prompt, "You are a helpful assistant.");
    /// assert_eq!(artifact.signed_by, "Agent-007");
    /// ```
    pub fn new(system_prompt: impl Into<String>, signed_by: impl Into<String>) -> Self {
        Self {
            system_prompt: system_prompt.into(),
            signed_by: signed_by.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_artifact_new() {
        let artifact = Artifact::new("Prompt content", "Signer");
        assert_eq!(artifact.system_prompt, "Prompt content");
        assert_eq!(artifact.signed_by, "Signer");
    }

    #[test]
    fn test_artifact_serialization() {
        let artifact = Artifact::new("System Prompt", "Agent A");
        let json = serde_json::to_string(&artifact).expect("Serialization failed");
        
        assert!(json.contains("System Prompt"));
        assert!(json.contains("Agent A"));
    }

    #[test]
    fn test_artifact_deserialization() {
        let json = r#"{"system_prompt": "Deserialize me", "signed_by": "Agent B"}"#;
        let artifact: Artifact = serde_json::from_str(json).expect("Deserialization failed");
        
        assert_eq!(artifact.system_prompt, "Deserialize me");
        assert_eq!(artifact.signed_by, "Agent B");
    }
    
    #[test]
    fn test_artifact_clone() {
         let artifact = Artifact::new("A", "B");
         let cloned = artifact.clone();
         assert_eq!(cloned.system_prompt, "A");
    }
    
    #[test]
    fn test_artifact_debug() {
        let artifact = Artifact::new("Debug", "Log");
        let debug_str = format!("{:?}", artifact);
        assert!(debug_str.contains("Artifact"));
        assert!(debug_str.contains("Debug"));
    }
}
