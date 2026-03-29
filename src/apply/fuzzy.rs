/// Fuzzy matcher for finding the best location to apply a change
/// when the file has drifted from the version the LLM saw.
///
/// Implemented in Iteration 4. This module is a skeleton until then.
pub struct FuzzyMatcher {
    /// Minimum similarity score (0.0–1.0) required to accept a match.
    #[allow(dead_code)] // used in iteration 4
    threshold: f32,
}

impl FuzzyMatcher {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    /// Find the best matching region in `haystack` for `needle`.
    ///
    /// Returns `(start_line, end_line, confidence)` or `None` if no
    /// match exceeds the threshold.
    pub fn find(&self, _haystack: &str, _needle: &str) -> Option<(usize, usize, f32)> {
        // TODO: implement in iteration 4
        // Algorithm:
        //   1. Normalize both strings (strip whitespace differences)
        //   2. Slide a window of needle.len() lines over haystack
        //   3. Score each window with strsim::jaro_winkler line-by-line
        //   4. Return the best scoring window if score >= threshold
        unimplemented!("fuzzy matching — see PLAN.md iteration 4")
    }
}

/// Normalize a string for fuzzy comparison:
/// - Trim trailing whitespace on each line
/// - Normalize indentation to a canonical form
/// - Strip inline comments (optional, controlled by caller)
pub fn normalize(s: &str) -> String {
    s.lines()
        .map(|line| line.trim_end())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_strips_trailing_whitespace() {
        assert_eq!(normalize("fn foo() {   \n    bar()   \n}"), "fn foo() {\n    bar()\n}");
    }
}
