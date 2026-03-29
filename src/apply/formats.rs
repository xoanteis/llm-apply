/// The format of a LLM-generated change.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeFormat {
    /// Standard unified diff (--- a/ / +++ b/ / @@ hunks)
    UnifiedDiff,
    /// Search/replace blocks (<<<SEARCH / === / >>>REPLACE)
    SearchReplace,
    /// Full file rewrite (the entire new content)
    FullRewrite,
}

impl std::fmt::Display for ChangeFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChangeFormat::UnifiedDiff => write!(f, "unified-diff"),
            ChangeFormat::SearchReplace => write!(f, "search-replace"),
            ChangeFormat::FullRewrite => write!(f, "full-rewrite"),
        }
    }
}

/// Attempt to detect the format of a change from its content.
///
/// Returns `None` if the format cannot be determined — caller should
/// ask the user to specify `--format` explicitly.
pub fn detect(change: &str) -> Option<ChangeFormat> {
    if looks_like_unified_diff(change) {
        return Some(ChangeFormat::UnifiedDiff);
    }
    if looks_like_search_replace(change) {
        return Some(ChangeFormat::SearchReplace);
    }
    // FullRewrite is the fallback — we can't reliably distinguish it
    // from arbitrary text, so only return it if the input is non-empty.
    if !change.trim().is_empty() {
        return Some(ChangeFormat::FullRewrite);
    }
    None
}

fn looks_like_unified_diff(s: &str) -> bool {
    s.contains("\n--- ") || s.starts_with("--- ")
}

fn looks_like_search_replace(s: &str) -> bool {
    // Accept common LLM variants: <<<, <<<<, SEARCH, etc.
    s.contains("<<<") && (s.contains("===") || s.contains(">>>"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_unified_diff() {
        let input = "--- a/main.rs\n+++ b/main.rs\n@@ -1,3 +1,3 @@\n";
        assert_eq!(detect(input), Some(ChangeFormat::UnifiedDiff));
    }

    #[test]
    fn detects_search_replace() {
        let input = "<<<SEARCH\nfn foo() {}\n===\nfn foo() { bar() }\n>>>REPLACE";
        assert_eq!(detect(input), Some(ChangeFormat::SearchReplace));
    }

    #[test]
    fn falls_back_to_full_rewrite() {
        let input = "fn hello() {\n    println!(\"hello\");\n}\n";
        assert_eq!(detect(input), Some(ChangeFormat::FullRewrite));
    }

    #[test]
    fn returns_none_for_empty() {
        assert_eq!(detect(""), None);
        assert_eq!(detect("   \n  "), None);
    }
}
