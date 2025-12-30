use thiserror::Error;

/// A specialized `Result` type for RigScribe operations.
///
/// This type alias simplifies function signatures by defaults the error type to [`ScribeError`].
pub type Result<T> = std::result::Result<T, ScribeError>;

/// Represents all possible errors that can occur during RigScribe execution.
///
/// This enum consolidates validation failures, configuration issues, LLM provider errors,
/// and internal processing faults into a single type.
///
/// # Examples
///
/// ```
/// use rigscribe::ScribeError;
///
/// let err = ScribeError::Validation("Request is too short".into());
/// assert!(format!("{}", err).contains("Invalid request"));
/// ```
#[derive(Debug, Error)]
pub enum ScribeError {
    /// The input provided by the user or an internal component was invalid.
    ///
    /// # Example
    /// An empty request string was passed to an agent.
    #[error("Invalid request:{0}. Hint: Pass a non empty request string.")]
    Validation(String),

    /// A required configuration (like an environment variable) is missing or malformed.
    #[error(
        "Configuration error: {0}. Hint: check required env vars (for example GEMINI_API_KEY)."
    )]
    Config(String),

    /// The underlying LLM provider (e.g., Gemini) failed to generate a completion.
    ///
    /// This could be due to API key issues, rate limits, or network connectivity.
    #[error(
        "LLM provider call filed: {0}. Hint: verify API key, model name, network, and maybe rate limit."
    )]
    Provider(#[from] rig::completion::PromptError),

    /// The LLM response did not match the expected format or protocol.
    ///
    /// This often happens when the model ignores JSON schema constraints.
    #[error(
        "Protocol violation: {0}. Hint: Provider return an unexpected format or reject the payload"
    )]
    ProtocolViolation(String),

    /// Failed to extract structured data (like JSON) from the model's raw text response.
    #[error("Extraction failed: {0}")]
    Extraction(#[from] rig::extractor::ExtractionError),

    /// A lower-level HTTP client error occurred.
    #[error("Client error: {0}")]
    ClientError(#[from] rig::http_client::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_error_formatting() {
        let err = ScribeError::Validation("Empty input".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Invalid request"));
        assert!(msg.contains("Empty input"));
        assert!(msg.contains("Hint: Pass a non empty request string"));
    }

    #[test]
    fn test_config_error_formatting() {
        let err = ScribeError::Config("MISSING_KEY".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Configuration error"));
        assert!(msg.contains("MISSING_KEY"));
        assert!(msg.contains("Hint: check required env vars"));
    }

    #[test]
    fn test_protocol_violation_error_formatting() {
        let err = ScribeError::ProtocolViolation("Bad JSON".to_string());
        let msg = format!("{}", err);
        assert!(msg.contains("Protocol violation"));
        assert!(msg.contains("Bad JSON"));
    }
}
