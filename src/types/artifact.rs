use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Artifact {
    pub system_prompt: String,
    pub signed_by: String,
}

impl Artifact {
    pub fn new(system_prompt: impl Into<String>, signed_by: impl Into<String>) -> Self {
        Self {
            system_prompt: system_prompt.into(),
            signed_by: signed_by.into(),
        }
    }
}
