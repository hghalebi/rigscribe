/// A unique identifier for a caching scope.
///
/// This ID is used to generate filenames for persisting optimized prompts to disk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);