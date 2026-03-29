use cucumber::World;
use std::path::PathBuf;
use tempfile::TempDir;

/// Shared world state for all BDD scenarios.
#[derive(Debug, Default, World)]
pub struct ApplyWorld {
    /// Temporary directory for the scenario — cleaned up automatically.
    pub tmp_dir: Option<TempDir>,
    /// The file being operated on.
    pub target_file: Option<PathBuf>,
    /// The change passed to llm-apply.
    pub change: Option<String>,
    /// The exit code of the last invocation.
    pub exit_code: Option<i32>,
    /// Captured stdout.
    pub stdout: String,
    /// Captured stderr.
    pub stderr: String,
}

mod steps;

#[tokio::main]
async fn main() {
    ApplyWorld::run("tests/features").await;
}
