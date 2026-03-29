use anyhow::Context;
use clap::{Parser, ValueEnum};
use std::io::Read;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "llm-apply",
    version,
    about = "Apply LLM-generated code changes reliably",
    long_about = "A reliable, standalone primitive for applying LLM-generated code changes.\n\nReads the change from stdin and applies it to FILE.\n\nExamples:\n  cat changes.diff | llm-apply src/main.rs\n  llm-apply --dry-run --json src/main.rs < patch.txt"
)]
struct Cli {
    /// The file to apply the change to.
    file: PathBuf,

    /// Force a specific change format instead of auto-detecting.
    #[arg(long, value_enum)]
    format: Option<Format>,

    /// Enable fuzzy matching for files that have drifted.
    #[arg(long, default_value_t = true)]
    fuzzy: bool,

    /// Minimum confidence score required to apply (0.0–1.0).
    #[arg(long, default_value_t = 0.75)]
    fuzzy_threshold: f32,

    /// Validate the result with tree-sitter (when language is detectable).
    #[arg(long, default_value_t = true)]
    validate: bool,

    /// Show what would change without writing to disk.
    #[arg(long)]
    dry_run: bool,

    /// Output structured JSON instead of human-readable text.
    #[arg(long)]
    json: bool,

    /// Apply even if confidence is below threshold (dangerous).
    #[arg(long)]
    force: bool,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    #[value(name = "unified-diff")]
    UnifiedDiff,
    #[value(name = "search-replace")]
    SearchReplace,
    #[value(name = "full-rewrite")]
    FullRewrite,
    Auto,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Read change from stdin
    let mut change = String::new();
    std::io::stdin()
        .read_to_string(&mut change)
        .context("failed to read change from stdin")?;

    // Read the target file
    let original = std::fs::read_to_string(&cli.file)
        .with_context(|| format!("failed to read {}", cli.file.display()))?;

    // Build options
    let format = cli.format.and_then(|f| match f {
        Format::UnifiedDiff => Some(llm_apply::apply::formats::ChangeFormat::UnifiedDiff),
        Format::SearchReplace => Some(llm_apply::apply::formats::ChangeFormat::SearchReplace),
        Format::FullRewrite => Some(llm_apply::apply::formats::ChangeFormat::FullRewrite),
        Format::Auto => None,
    });

    let options = llm_apply::ApplyOptions {
        format,
        fuzzy: cli.fuzzy,
        fuzzy_threshold: if cli.force { 0.0 } else { cli.fuzzy_threshold },
        validate_ast: cli.validate,
        dry_run: cli.dry_run,
    };

    let engine = llm_apply::ApplyEngine::default();

    match engine.apply(&original, &change, options) {
        Ok((new_content, result)) => {
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                eprintln!(
                    "✓ Applied {} hunk(s) (confidence: {:.0}%)",
                    result.hunks_applied,
                    result.confidence * 100.0
                );
                if cli.dry_run {
                    eprintln!("  [dry run — file not modified]");
                }
            }

            if !cli.dry_run {
                llm_apply::apply::atomic::write(&cli.file, &new_content)?;
            }

            std::process::exit(0);
        }
        Err(e) => {
            if cli.json {
                let output = serde_json::json!({
                    "success": false,
                    "error": e.to_string(),
                });
                eprintln!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                eprintln!("✗ {}", e);
            }
            std::process::exit(1);
        }
    }
}
