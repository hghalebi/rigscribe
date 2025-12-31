/// A unique identifier for a caching scope.
///
/// This ID is used to generate filenames for persisting optimized prompts to disk.
///
/// # Examples
///
/// ```
/// use rigscribe::ScopeId;
///
/// let id = ScopeId(42);
/// assert_eq!(id.0, 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(pub u64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_id_creation() {
        let id = ScopeId(12345);
        assert_eq!(id.0, 12345);
    }

    #[test]
    fn test_scope_id_equality() {
        let id1 = ScopeId(10);
        let id2 = ScopeId(10);
        let id3 = ScopeId(20);

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_scope_id_debug_fmt() {
        let id = ScopeId(99);
        assert_eq!(format!("{:?}", id), "ScopeId(99)");
    }
}
