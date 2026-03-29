pub mod formats;
pub mod fuzzy;
pub mod atomic;

use serde::{Deserialize, Serialize};
use crate::Error;

/// Options controlling how a change is applied.
#[derive(Debug, Clone)]
pub struct ApplyOptions {
    /// Force a specific format instead of auto-detecting.
    pub format: Option<formats::ChangeFormat>,
    /// Enable fuzzy matching for drifted files.
    pub fuzzy: bool,
    /// Minimum confidence score required to apply (0.0–1.0).
    pub fuzzy_threshold: f32,
    /// Validate the result with tree-sitter if a language is detected.
    pub validate_ast: bool,
    /// If true, compute the result but do not write to disk.
    pub dry_run: bool,
}

impl Default for ApplyOptions {
    fn default() -> Self {
        Self {
            format: None,
            fuzzy: true,
            fuzzy_threshold: 0.75,
            validate_ast: true,
            dry_run: false,
        }
    }
}

/// The result of a successful apply operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplyResult {
    pub success: bool,
    pub hunks_applied: usize,
    pub hunks_failed: usize,
    pub confidence: f32,
    pub valid_ast: Option<bool>,
    pub format_detected: String,
    pub diff: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// The main engine — stateless, cheap to construct.
#[derive(Default)]
pub struct ApplyEngine;

impl ApplyEngine {
    /// Apply a change to `original` content and return the modified content.
    ///
    /// Does not touch the filesystem — use [`atomic::write`] for that.
    pub fn apply(
        &self,
        _original: &str,
        change: &str,
        options: ApplyOptions,
    ) -> Result<(String, ApplyResult), Error> {
        let format = match options.format {
            Some(f) => f,
            None => formats::detect(change).ok_or(Error::FormatDetectionFailed)?,
        };

        // Dispatch to format-specific apply logic (implemented in later iterations)
        match format {
            formats::ChangeFormat::UnifiedDiff => {
                Err(Error::FormatDetectionFailed) // placeholder until iteration 1
            }
            formats::ChangeFormat::SearchReplace => {
                Err(Error::FormatDetectionFailed) // placeholder until iteration 2
            }
            formats::ChangeFormat::FullRewrite => {
                Err(Error::FormatDetectionFailed) // placeholder until iteration 3
            }
        }
    }
}
