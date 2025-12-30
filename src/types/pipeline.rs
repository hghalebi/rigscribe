use crate::error::{Result, ScribeError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
pub struct Webquery {
    pub(crate) query: String,
}
