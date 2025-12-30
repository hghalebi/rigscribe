//pub const MODEL: &str = "gemini-3-pro-preview"; // does not work
//pub const MODEL: &str = "gemini-3-flash-preview"; // does not work
//pub const MODEL: &str = "gemini-2.5-flash-lite"; //does not work
/// The default Gemini model used by RigScribe.
///
/// Currently set to `gemini-2.5-pro` as it provides the best balance of quality and reliability.
pub const MODEL: &str = "gemini-2.5-pro"; // it works
//pub const MODEL: &str = "gemini-2.5-flash"; //it works
//pub const MODEL: &str = "gemini-2.0-flash-lite"; // it works but out come is so low quality
//pub const MODEL: &str = "gemini-1.5-pro"; // it does not works

/// Configuration options for the RigScribe application.
#[derive(Debug, Clone)]
pub struct RigScribeConfig {
    /// The name of the LLM model to use (e.g., "gemini-1.5-pro").
    pub model: &'static str,
}

impl RigScribeConfig {
    /// Sets the model to be used.
    ///
    /// # Arguments
    ///
    /// * `model` - A static string slice representing the model name.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // This is internal configuration
    /// ```
    #[allow(dead_code)]
    fn set_model(&mut self, model: &'static str) {
        self.model = model;
    }
}

impl Default for RigScribeConfig {
    fn default() -> Self {
        Self { model: MODEL }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = RigScribeConfig::default();
        assert_eq!(config.model, MODEL);
        assert_eq!(config.model, "gemini-2.5-pro");
    }

    #[test]
    fn test_set_model() {
        let mut config = RigScribeConfig::default();
        config.set_model("test-model-v1");
        assert_eq!(config.model, "test-model-v1");
    }
    
    #[test]
    fn test_config_clone() {
        let config = RigScribeConfig::default();
        let cloned = config.clone();
        assert_eq!(cloned.model, config.model);
    }
    
    #[test]
    fn test_config_debug() {
        let config = RigScribeConfig::default();
        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("RigScribeConfig"));
        assert!(debug_str.contains(MODEL));
    }
}