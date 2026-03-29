/// Step definitions for BDD scenarios.
///
/// Steps are implemented progressively alongside each iteration.
/// Unimplemented steps will panic with a clear message referencing
/// the relevant iteration in PLAN.md.

use cucumber::{given, then, when};
use std::process::Command;
use tempfile::TempDir;

use super::ApplyWorld;

// ── Given ─────────────────────────────────────────────────────

#[given("I have the llm-apply tool available")]
async fn tool_available(_world: &mut ApplyWorld) {
    // Verify the binary is built — in CI this is always the case.
    // Locally, run `cargo build` first.
}

#[given(expr = "a file {string} with content:")]
async fn file_with_content(world: &mut ApplyWorld, path: String, content: String) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let file_path = tmp.path().join(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create parent dirs");
    }
    std::fs::write(&file_path, content.trim_start_matches('\n'))
        .expect("failed to write fixture file");
    world.target_file = Some(file_path);
    world.tmp_dir = Some(tmp);
}

#[given(expr = "a file {string} with content {string}")]
async fn file_with_inline_content(world: &mut ApplyWorld, path: String, content: String) {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let file_path = tmp.path().join(&path);
    if let Some(parent) = file_path.parent() {
        std::fs::create_dir_all(parent).expect("failed to create parent dirs");
    }
    std::fs::write(&file_path, &content).expect("failed to write fixture file");
    world.target_file = Some(file_path);
    world.tmp_dir = Some(tmp);
}

// ── When ──────────────────────────────────────────────────────

#[when("I apply the unified diff:")]
async fn apply_unified_diff(world: &mut ApplyWorld, diff: String) {
    run_apply(world, &diff, &[]);
}

#[when("I apply an empty change")]
async fn apply_empty(world: &mut ApplyWorld) {
    run_apply(world, "", &[]);
}

#[when(expr = "I apply a valid diff with --dry-run")]
async fn apply_dry_run(world: &mut ApplyWorld) {
    // TODO: store a valid diff in world during Given step
    run_apply(world, "placeholder", &["--dry-run"]);
}

#[when(expr = "I apply a valid diff with --json")]
async fn apply_with_json(world: &mut ApplyWorld) {
    // TODO: store a valid diff in world during Given step
    run_apply(world, "placeholder", &["--json"]);
}

// ── Then ──────────────────────────────────────────────────────

#[then("the exit code is 0")]
async fn exit_code_zero(world: &mut ApplyWorld) {
    assert_eq!(world.exit_code, Some(0), "expected exit code 0, got {:?}\nstderr: {}", world.exit_code, world.stderr);
}

#[then("the exit code is 1")]
async fn exit_code_one(world: &mut ApplyWorld) {
    assert_eq!(world.exit_code, Some(1), "expected exit code 1, got {:?}", world.exit_code);
}

#[then("the exit code is 2")]
async fn exit_code_two(world: &mut ApplyWorld) {
    assert_eq!(world.exit_code, Some(2), "expected exit code 2, got {:?}", world.exit_code);
}

#[then("the exit code is non-zero")]
async fn exit_code_nonzero(world: &mut ApplyWorld) {
    assert_ne!(world.exit_code, Some(0), "expected non-zero exit code");
}

#[then("the file is unchanged")]
async fn file_unchanged(world: &mut ApplyWorld) {
    // Implemented when atomic write is in place (iteration 1)
    todo!("iteration 1 — atomic write")
}

#[then(expr = "stderr contains {string}")]
async fn stderr_contains(world: &mut ApplyWorld, expected: String) {
    assert!(
        world.stderr.contains(&expected),
        "expected stderr to contain {:?}\nActual stderr: {}",
        expected,
        world.stderr
    );
}

#[then("stdout is valid JSON")]
async fn stdout_is_valid_json(world: &mut ApplyWorld) {
    serde_json::from_str::<serde_json::Value>(&world.stdout)
        .expect("stdout is not valid JSON");
}

// ── Helpers ───────────────────────────────────────────────────

fn run_apply(world: &mut ApplyWorld, change: &str, extra_args: &[&str]) {
    let file = world.target_file.as_ref().expect("no target file set");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_llm-apply"));
    cmd.arg(file);
    for arg in extra_args {
        cmd.arg(arg);
    }
    cmd.stdin(std::process::Stdio::piped());
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());

    use std::io::Write;
    let mut child = cmd.spawn().expect("failed to spawn llm-apply");
    child
        .stdin
        .take()
        .expect("stdin not available")
        .write_all(change.as_bytes())
        .expect("failed to write to stdin");

    let output = child.wait_with_output().expect("failed to wait for llm-apply");

    world.exit_code = output.status.code();
    world.stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    world.stderr = String::from_utf8_lossy(&output.stderr).into_owned();
}
