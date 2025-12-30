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
