use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScribeError>;

#[derive(Debug, Error)]
pub enum ScribeError {
    #[error("Invalid request:{0}. Hint: Pass a non empty request string.")]
    Validation(String),

    #[error(
        "Configuration error: {0}. Hint: check required env vars (for example GEMINI_API_KEY)."
    )]
    Config(String),

    #[error(
        "LLM provider call filed: {0}. Hint: verify API key, model name, network, and maybe rate limit."
    )]
    Provider(#[from] rig::completion::PromptError),

    #[error(
        "Protocol violation: {0}. Hint: Provider return an unexpected format or reject the payload"
    )]
    ProtocolViolation(String),

    #[error("Extraction failed: {0}")]
    Extraction(#[from] rig::extractor::ExtractionError),

    #[error("Client error: {0}")]
    ClientError(#[from] rig::http_client::Error),
}

pub fn map_provider_error(e: rig::completion::PromptError) -> ScribeError {
    let msg = e.to_string();
    if msg.contains("though_signature") || msg.contains("INVALID_ARGUMENT") {
        ScribeError::ProtocolViolation(msg)
    } else {
        ScribeError::Provider(e)
    }
}
