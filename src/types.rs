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
    #[schemars(
        description = "he raw user intent. You must analyze this to extract technical constraints."
    )]
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
    #[schemars(
        description = "The primary goal derived from the user's intent. concise and clear."
    )]
    pub goal: String,
    #[schemars(
        description = "A list of technical constraints, risks, and negative constraints. Format as a bulleted string."
    )]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Webquery {
    pub(crate) query: String,
}