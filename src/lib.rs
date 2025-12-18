mod utilities;
mod error;
pub mod pipline;
mod types;

pub use error::{Result, ScribeError};
use pipline::optimizer;
pub use types::{Artifact, Intent, Specification};

pub struct RigScribe;
impl RigScribe {
    pub async fn optimize_agentic(request: impl Into<String>, id: i128) -> Result<Artifact> {
        let intent = Intent {
            text: request.into(),
        };
        let res = optimizer(intent, id).await?;
        Ok(res)
    }
}
