/// Tree-sitter based AST validation.
///
/// Implemented in Iteration 5. Skeleton only until then.

/// Languages supported for AST validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    TypeScript,
    Go,
}

impl Language {
    /// Detect language from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "ts" | "tsx" => Some(Language::TypeScript),
            "go" => Some(Language::Go),
            _ => None,
        }
    }
}

/// Validate that `content` is syntactically valid for `language`.
///
/// Returns `Ok(())` if valid, `Err` with location and message if not.
pub fn validate(
    _content: &str,
    _language: &Language,
) -> Result<(), crate::Error> {
    // TODO: implement in iteration 5
    // Use tree-sitter to parse and check for ERROR nodes
    unimplemented!("AST validation — see PLAN.md iteration 5")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_rust_from_extension() {
        assert_eq!(Language::from_extension("rs"), Some(Language::Rust));
    }

    #[test]
    fn detects_python_from_extension() {
        assert_eq!(Language::from_extension("py"), Some(Language::Python));
    }

    #[test]
    fn returns_none_for_unknown_extension() {
        assert_eq!(Language::from_extension("unknown"), None);
    }
}
