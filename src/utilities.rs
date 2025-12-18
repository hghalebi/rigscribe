use crate::error::{Result, ScribeError};

pub fn require_env(name: &str) -> Result<()> {
    if std::env::var(name).is_ok() {
        Ok(())
    } else {
        Err(ScribeError::Config(format!("{name} is missing!")))
    }
}
