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
    pub fn new(system_prompt: impl Into<String>, signed_by: impl Into<String>) -> Self {
        Self {
            system_prompt: system_prompt.into(),
            signed_by: signed_by.into(),
        }
    }
}