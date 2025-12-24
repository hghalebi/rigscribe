mod error;
pub mod pipline;
mod types;
mod utilities;

pub use error::{Result, ScribeError};
use pipline::optimizer;
pub use types::{Artifact, Intent, Specification};

pub struct RigScribe;
impl RigScribe {
    pub async fn optimize_agentic(request: impl Into<String>) -> Result<Artifact> {
        let intent = Intent {
            text: request.into(),
        };
        let res = optimizer(intent).await?;
        Ok(res)
    }
}
