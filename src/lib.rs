#[allow(unused)]
#[allow(dead_code)]
#[allow(unused_imports)]
mod agents;
mod error;
mod types;

use agents::Chief;
pub use error::{Result, ScribeError};
pub use types::{Artifact, Intent, Specification};

pub struct RigScribe {
    chief: Chief,
}
impl RigScribe {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            chief: Chief::from_env()?,
        })
    }
    pub async fn optimize(&self, request: impl Into<String>) -> Result<Artifact> {
        let intent = Intent::new(request)?;
        let spec = self.chief.plan(&intent).await?;
        let draft = self.chief.build(&spec).await?;
        self.chief.review(&spec, &draft).await
    }
}
