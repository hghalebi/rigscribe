mod error;
pub mod agents;
pub mod tools;
mod types;
pub mod utilities;

use std::path::PathBuf;

pub use error::{Result, ScribeError};
use agents::optimizer::optimizer;

pub use types::{Artifact, Intent, ScopeId, Specification};

use crate::utilities::{read_artifact, save_artifacts};

pub struct RigScribe {
    cache_dir: PathBuf,
}
impl RigScribe {
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }
    pub async fn optimize_agentic(request: impl Into<String>) -> Result<Artifact> {
        let intent = Intent {
            text: request.into(),
        };
        let artifact = optimizer(intent).await?;
        Ok(artifact)
    }
    pub async fn optimize_with_cache(
        &self,
        request: impl Into<String>,
        id: ScopeId,
    ) -> Result<Artifact> {
        let file_name = format!("{}.json", id.0);
        let path = self.cache_dir.join(file_name);

        if let Ok(cached_artifact) = read_artifact(&path).await {
            eprintln!("Cache HIT: {:?} loaded from disk", path);
            return Ok(cached_artifact);
        }
        eprintln!("Cache MIS: {:?}", path);
        eprintln!("Optimizing ...");
        let fresh_artifact = Self::optimize_agentic(request.into()).await?;
        save_artifacts(&path, &fresh_artifact).await?;
        eprintln!("Optimize prompte cached to: {:?}", path);
        Ok(fresh_artifact)
    }
}
