#[allow(unused)]
#[allow(dead_code)]
#[allow(unused_imports)]
use crate::error::{Result, ScribeError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Specification {
    pub goal: String,
    pub constraints: String,
}

#[derive(Debug, Clone)]
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
