use crate::error::{Result, ScribeError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
//pub const MODEL: &str = "gemini-3-pro-preview"; // does not work
//pub const MODEL: &str = "gemini-3-flash-preview"; // does not work
//pub const MODEL: &str = "gemini-2.5-flash-lite"; //does not work
pub const MODEL: &str = "gemini-2.5-pro"; // it works
//pub const MODEL: &str = "gemini-2.5-flash"; //it works
//pub const MODEL: &str = "gemini-2.0-flash-lite"; // it works but out come is so low quality
//pub const MODEL: &str = "gemini-1.5-pro"; // it does not works
#[derive(Debug, Clone)]
pub struct RigScribeConfig {
    pub model: &'static str,
}
impl RigScribeConfig {
    fn set_model(&mut self, model: &'static str) {
        self.model = model;
    }
}
impl Default for RigScribeConfig {
    fn default() -> Self {
        Self { model: MODEL }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Intent {
    pub text: String,
}

impl Intent {
    pub fn new(text: impl Into<String>) -> Result<Self> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ScribeError::Validation("Request is empty".into()));
        }
        Ok(Self { text })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Specification {
    pub goal: String,
    pub constraints: String,
}

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
