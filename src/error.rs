use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("hunk did not match at line {line} — the file may have changed since the diff was generated. Try --fuzzy or regenerate the change.")]
    HunkMismatch { line: usize },

    #[error("ambiguous match: the search block was found at {count} locations (lines: {locations:?}). Provide more context to disambiguate.")]
    AmbiguousMatch { count: usize, locations: Vec<usize> },

    #[error("search block not found — verify the change was generated from the current version of the file.")]
    SearchBlockNotFound,

    #[error("result has invalid syntax at line {line}: {message}. The file has been left unchanged.")]
    InvalidSyntaxAfterApply { line: usize, message: String },

    #[error("confidence too low ({score:.2}) — the file may have changed too much. Use --force to apply anyway.")]
    LowConfidence { score: f32 },

    #[error("could not detect change format — use --format to specify explicitly.")]
    FormatDetectionFailed,

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid UTF-8 in file: {0}")]
    Encoding(#[from] std::string::FromUtf8Error),
}
