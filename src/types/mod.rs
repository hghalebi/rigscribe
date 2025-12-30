pub mod config;
pub mod pipeline;
pub mod artifact;
pub mod common;

pub use config::{RigScribeConfig, MODEL};
pub use pipeline::{Intent, Specification, Webquery};
pub use artifact::Artifact;
pub use common::ScopeId;
