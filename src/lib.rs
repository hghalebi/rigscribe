//! # RigScribe
//!
//! `RigScribe` is an intelligent agentic system designed to transform vague user intents
//! into highly optimized, production-ready system prompts.
//!
//! It orchestrates a team of AI agents (Deconstructor, Prompt Officer, Web Researcher)
//! to analyze, research, and refine prompts.
//!
//! ## Example
//!
//! ```rust,no_run
//! use rigscribe::{RigScribe, ScopeId, Result};
//! use std::path::PathBuf;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let scribe = RigScribe::new("./cache");
//!     let result = scribe.optimize_with_cache("Write a snake game in python", ScopeId(1)).await?;
//!     println!("{}", result.system_prompt);
//!     Ok(())
//! }
//! ```

mod error;
pub mod agents;
pub mod tools;
mod types;
pub mod logging;
pub mod utilities;

use std::path::PathBuf;

pub use error::{Result, ScribeError};
use agents::optimizer::optimizer;

pub use types::{Artifact, Intent, ScopeId, Specification};

use crate::utilities::{read_artifact, save_artifacts};

/// The main client for the RigScribe engine.
///
/// Handles configuration, caching, and dispatching requests to the agent swarm.
pub struct RigScribe {
    /// Directory where optimized prompts are cached to avoid re-running expensive agent chains.
    cache_dir: PathBuf,
}
use tracing::info;

impl RigScribe {
    /// Creates a new `RigScribe` instance.
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Path to the directory where artifacts should be stored.
    pub fn new(cache_dir: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: cache_dir.into(),
        }
    }

    /// Triggers the full agentic optimization pipeline without caching.
    ///
    /// This method converts the string request into an [`Intent`] and passes it
    /// to the [`optimizer`](agents::optimizer::optimizer) agent.
    pub async fn optimize_agentic(request: impl Into<String>) -> Result<Artifact> {
        let intent = Intent {
            text: request.into(),
        };
        let artifact = optimizer(intent).await?;
        Ok(artifact)
    }

    /// Optimizes a prompt with filesystem-based caching.
    ///
    /// If an artifact with the given [`ScopeId`] exists in the `cache_dir`, it is returned immediately.
    /// Otherwise, the agentic pipeline is triggered, and the result is saved to disk.
    ///
    /// # Arguments
    ///
    /// * `request` - The user's prompt intent.
    /// * `id` - A unique identifier for this request scope.
    pub async fn optimize_with_cache(
        &self,
        request: impl Into<String>,
        id: ScopeId,
    ) -> Result<Artifact> {
        let file_name = format!("{}.json", id.0);
        let path = self.cache_dir.join(file_name);

        if let Ok(cached_artifact) = read_artifact(&path).await {
            info!("Cache HIT: {:?} loaded from disk", path);
            return Ok(cached_artifact);
        }
        info!("Cache MIS: {:?}", path);
        info!("Optimizing ...");
        let fresh_artifact = Self::optimize_agentic(request.into()).await?;
        save_artifacts(&path, &fresh_artifact).await?;
        info!("Optimize prompt cached to: {:?}", path);
        Ok(fresh_artifact)
    }
}