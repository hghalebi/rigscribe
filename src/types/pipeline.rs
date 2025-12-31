use crate::error::{Result, ScribeError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Represents the initial user intent.
///
/// This struct wraps the raw input text provided by the user before any processing.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Intent {
    /// The raw user intent. You must analyze this to extract technical constraints.
    #[schemars(
        description = "he raw user intent. You must analyze this to extract technical constraints."
    )]
    pub text: String,
}

impl Intent {
    /// Creates a new `Intent` from a string.
    ///
    /// # Errors
    ///
    /// Returns a [`ScribeError::Validation`] if the input text is empty or whitespace-only.
    ///
    /// # Examples
    ///
    /// ```
    /// use rigscribe::Intent;
    ///
    /// let intent = Intent::new("Create a Python script").unwrap();
    /// assert_eq!(intent.text, "Create a Python script");
    ///
    /// let err = Intent::new("   ").unwrap_err();
    /// assert!(format!("{}", err).contains("Invalid request"));
    /// ```
    pub fn new(text: impl Into<String>) -> Result<Self> {
        let text = text.into();
        if text.trim().is_empty() {
            return Err(ScribeError::Validation("Request is empty".into()));
        }
        Ok(Self { text })
    }
}

/// A structured technical specification derived from the user's `Intent`.
///
/// This is produced by the `Deconstructor` tool.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Specification {
    /// The primary goal derived from the user's intent. concise and clear.
    #[schemars(
        description = "The primary goal derived from the user's intent. concise and clear."
    )]
    pub goal: String,

    /// A list of technical constraints, risks, and negative constraints.
    ///
    /// Format as a bulleted string.
    #[schemars(
        description = "A list of technical constraints, risks, and negative constraints. Format as a bulleted string."
    )]
    pub constraints: String,
}

/// A search query intended for a web search tool.
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
pub struct Webquery {
    pub(crate) query: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_new_valid() {
        let intent = Intent::new("valid input").expect("Should succeed");
        assert_eq!(intent.text, "valid input");
    }

    #[test]
    fn test_intent_new_trimming_empty() {
        let res = Intent::new("   ");
        match res {
            Err(ScribeError::Validation(msg)) => assert_eq!(msg, "Request is empty"),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_intent_new_completely_empty() {
        let res = Intent::new("");
        match res {
            Err(ScribeError::Validation(msg)) => assert_eq!(msg, "Request is empty"),
            _ => panic!("Expected Validation error"),
        }
    }

    #[test]
    fn test_specification_serialization() {
        let spec = Specification {
            goal: "Goal".into(),
            constraints: "None".into(),
        };
        let json = serde_json::to_string(&spec).unwrap();
        assert!(json.contains("Goal"));
    }

    #[test]
    fn test_webquery_serialization() {
        let wq = Webquery { query: "rust".into() };
        let json = serde_json::to_string(&wq).unwrap();
        assert!(json.contains("rust"));
    }
    
    #[test]
    fn test_intent_json_schema() {
        let schema = schemars::schema_for!(Intent);
        let json = serde_json::to_string(&schema).unwrap();
        assert!(json.contains("user intent"));
    }
}
