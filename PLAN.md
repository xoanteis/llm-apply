# Build Plan — llm-apply

## Philosophy

- **Iterative over big-bang** — each iteration ships something that works end-to-end
- **BDD-first** — behaviour is specified in `.feature` files before code is written
- **No silent failures** — every iteration adds regression coverage before adding features
- **Simple before clever** — exact matching before fuzzy, one format before many
- **Benchmarks as citizens** — performance is measured from iteration 1, not bolted on later

Each iteration has:
- A clear goal expressible in one sentence
- Acceptance criteria as BDD scenarios
- A definition of done that includes passing CI, no regressions, updated docs

---

## Iteration 0 — Skeleton (Week 1)

**Goal:** A working repo with CI, test infrastructure, and a CLI that parses arguments but does nothing yet.

### Tasks
- [ ] `cargo new llm-apply --lib` with `main.rs` entry point
- [ ] `clap` for CLI argument parsing (`--format`, `--dry-run`, `--validate`, `--json`)
- [ ] `cucumber` crate wired up — `cargo test --features bdd` runs feature files
- [ ] GitHub Actions CI: `cargo test`, `cargo clippy`, `cargo fmt --check`
- [ ] First `.feature` file: `apply_basic.feature` (scenarios pending, tagged `@wip`)
- [ ] `CONTRIBUTING.md` with BDD workflow documented
- [ ] Dependabot enabled

### Definition of Done
- `cargo test` passes (zero tests, but infrastructure works)
- `cargo clippy -- -D warnings` passes
- CI is green on push to `main`
- `llm-apply --help` outputs structured help

---

## Iteration 1 — Exact Unified Diff (Weeks 2–3)

**Goal:** Apply a syntactically correct unified diff to a file, atomically.

This is the simplest case — no fuzzy matching, no format detection, no validation. Just: given a valid diff and a matching file, produce the correct output.

### BDD Scenarios (`apply_basic.feature`)
```gherkin
Feature: Apply unified diff

  Scenario: Apply a single-hunk diff
    Given a file "src/main.rs" with content:
      """
      fn hello() {
          println!("hello");
      }
      """
    When I apply the unified diff:
      """
      --- a/src/main.rs
      +++ b/src/main.rs
      @@ -1,3 +1,3 @@
       fn hello() {
      -    println!("hello");
      +    println!("hello, world");
       }
      """
    Then the file contains:
      """
      fn hello() {
          println!("hello, world");
      }
      """
    And the exit code is 0

  Scenario: Diff does not match file — fails loudly
    Given a file "src/main.rs" with content:
      """
      fn goodbye() { }
      """
    When I apply a diff that references "fn hello()"
    Then the exit code is non-zero
    And stderr contains "hunk did not match"
    And the file is unchanged
```

### Tasks
- [ ] `ChangeFormat::UnifiedDiff` variant in `formats.rs`
- [ ] Diff parser (use `diffy` crate as base, wrap with our error types)
- [ ] Atomic write: write to `.llm-apply.tmp`, then `rename` (atomic on same filesystem)
- [ ] Exit codes: 0 = success, 1 = apply failure, 2 = invalid input
- [ ] All BDD scenarios green
- [ ] Benchmark baseline: record p50/p99 for 100/500/2000 LOC files

### Definition of Done
- All scenarios in `apply_basic.feature` green
- Atomic write verified: killing process mid-write leaves original file intact
- Benchmark numbers recorded in `benches/README.md`

---

## Iteration 2 — Search/Replace Format (Week 4)

**Goal:** Support the search/replace format that Claude and most LLMs prefer to generate.

### BDD Scenarios (`apply_search_replace.feature`)
```gherkin
Feature: Apply search/replace blocks

  Scenario: Apply a simple search/replace
    Given a file with content:
      """
      def parse(data):
          return data
      """
    When I apply the search/replace change:
      """
      <<<SEARCH
      def parse(data):
          return data
      ===
      def parse(data: dict) -> dict:
          if not data:
              raise ValueError("empty")
          return data
      >>>REPLACE
      """
    Then the file contains the new implementation
    And the exit code is 0

  Scenario: Search block not found — fails loudly
    ...

  Scenario: Search block matches multiple locations — fails with ambiguity error
    ...
```

### Tasks
- [ ] `ChangeFormat::SearchReplace` variant
- [ ] Parser for `<<<SEARCH` / `===` / `>>>REPLACE` markers (and common variants: `<<<<`, `>>>>`, `---`, `+++`)
- [ ] Ambiguity detection: if search block matches >1 location, return error with line numbers of all candidates
- [ ] Format auto-detection (`ChangeFormat::detect(input: &str) -> ChangeFormat`)
- [ ] Extend CI matrix: test both formats in the same suite

### Definition of Done
- All search/replace scenarios green
- Auto-detection works for both formats
- No regressions on iteration 1 scenarios

---

## Iteration 3 — Full Rewrite Format (Week 5)

**Goal:** Handle the case where the LLM returns the entire file rewritten.

This is the simplest format to apply but the hardest to validate — did the LLM accidentally change things it shouldn't have?

### BDD Scenarios (`apply_rewrite.feature`)
```gherkin
  Scenario: Apply full rewrite
    ...

  Scenario: Full rewrite preserves file encoding and line endings
    ...
```

### Tasks
- [ ] `ChangeFormat::FullRewrite` variant
- [ ] Line ending preservation (LF vs CRLF)
- [ ] File encoding detection and preservation
- [ ] Auto-detection extended to cover rewrite (heuristic: does the input look like a complete file?)
- [ ] `--format` flag forces format, bypasses auto-detection

### Definition of Done
- All three formats covered with BDD scenarios
- Auto-detection accuracy tracked as a metric in CI

---

## Iteration 4 — Fuzzy Matching (Weeks 6–8)

**Goal:** Apply changes correctly even when the file has drifted from the version the LLM saw.

This is the technically hard iteration. Take the time needed.

### BDD Scenarios (`apply_fuzzy.feature`)
```gherkin
Feature: Fuzzy matching for drifted files

  Scenario: Apply change when whitespace has changed (L1 drift)
    Given the LLM generated a change based on a file with 2-space indent
    And the file now uses 4-space indent (formatted by rustfmt)
    When I apply the change with fuzzy matching enabled
    Then the change is applied correctly
    And confidence score is > 0.90

  Scenario: Apply change when type hints were added nearby (L2 drift)
    ...

  Scenario: Drift is too large — reports low confidence
    Given a file that has been significantly refactored
    When I apply the change
    Then exit code is non-zero
    And the JSON output contains confidence < 0.5
    And stderr suggests running with --force or reviewing manually
```

### Approach
1. **Normalize before matching** — strip whitespace differences, normalize line endings
2. **Sliding window similarity** — find the best matching region using a similarity score (Jaro-Winkler on lines)
3. **Confidence score** — `1.0 - (edit_distance / search_block_length)`, threshold at 0.75
4. **Never apply silently below threshold** — low confidence = non-zero exit + explanation

### Tasks
- [ ] `FuzzyMatcher` struct with configurable threshold
- [ ] Line normalization pipeline (whitespace, comments, type hints)
- [ ] Similarity scoring (use `strsim` crate)
- [ ] Confidence score in JSON output
- [ ] `--fuzzy` flag (default: on), `--fuzzy-threshold 0.75`
- [ ] Benchmark: fuzzy vs exact on the same dataset — measure false positives

### Definition of Done
- All fuzzy scenarios green
- False positive rate (applied to wrong location) < 1% on benchmark dataset
- Performance: fuzzy adds < 15ms overhead on 500 LOC files

---

## Iteration 5 — AST Validation (Weeks 9–10)

**Goal:** Verify that the result of applying a change is syntactically valid using tree-sitter.

### BDD Scenarios (`validate_ast.feature`)
```gherkin
Feature: AST validation after apply

  Scenario: Reject change that produces invalid syntax
    Given a Rust file with valid syntax
    When I apply a change that introduces a syntax error
    Then the exit code is non-zero
    And the file is unchanged (rollback)
    And stderr contains the syntax error location

  Scenario: Accept change that produces valid syntax
    ...

  Scenario: Unknown language — skip validation, emit warning
    ...
```

### Tasks
- [ ] `tree-sitter` crate integration
- [ ] Language grammars: Rust, Python, TypeScript, Go (the four most common in agentic pipelines)
- [ ] Language auto-detection from file extension
- [ ] Validate pre-apply (original is valid) and post-apply (result is valid)
- [ ] If post-apply is invalid: rollback atomically, report error with line number
- [ ] `--validate` flag (default: on when language is detectable), `--no-validate` to skip

### Definition of Done
- Validation works for all four languages
- Rollback is tested: corrupt intermediate state never persists
- `--no-validate` documented with clear warning about tradeoffs

---

## Iteration 6 — JSON API & Structured Output (Week 11)

**Goal:** Make `llm-apply` first-class composable in agent pipelines via structured output.

### Tasks
- [ ] `--json` flag outputs full structured result:
  ```json
  {
    "success": bool,
    "hunks_applied": int,
    "hunks_failed": int,
    "confidence": float,
    "valid_ast": bool | null,
    "format_detected": string,
    "error": string | null,
    "diff": string
  }
  ```
- [ ] Machine-readable exit codes documented in `man` page / README
- [ ] `--dry-run` works with `--json` (shows what would change without touching the file)
- [ ] Stdin/stdout pipeline mode: `cat change.txt | llm-apply file.rs | jq .confidence`

### Definition of Done
- All output modes covered by BDD scenarios
- Exit codes are stable (semver-stable after this point)

---

## Iteration 7 — Benchmark Suite (Weeks 12–13)

**Goal:** A reproducible, public benchmark that demonstrates where `llm-apply` outperforms alternatives.

### Tasks
- [ ] Dataset builder: extract (file, diff, expected) triplets from CPython and Django git history
- [ ] Synthetic cases: generate diffs with GPT-4o and Claude, capture verbatim LLM output
- [ ] Cases from Aider/Continue issues: manual transcription of reported failures
- [ ] Benchmark runner: `cargo bench` + custom runner for correctness metrics
- [ ] Comparison runner: runs same dataset against Aider's internal apply logic
- [ ] Results published to `benches/results.md` as part of CI

### Metrics tracked
| Metric | Target |
|--------|--------|
| Apply correctness (exact) | > 99% |
| Apply correctness (L4 fuzzy) | > 88% |
| Ambiguity detection rate | > 92% |
| False positive rate | < 1% |
| p50 latency (500 LOC) | < 50ms |
| p99 latency (500 LOC) | < 200ms |

---

## Iteration 8 — HTTP API (Weeks 14–16)

**Goal:** Expose `llm-apply` as a local or hosted HTTP service for pipelines that can't shell out.

```
POST /apply
Content-Type: application/json

{
  "content": "<file content>",
  "change": "<diff or search-replace>",
  "format": "auto",
  "language": "rust",
  "fuzzy": true,
  "validate": true
}
```

### Tasks
- [ ] `axum` HTTP server behind `--serve` flag
- [ ] Request/response types shared with CLI JSON output (same structs)
- [ ] Health endpoint: `GET /health`
- [ ] Rate limiting middleware (token bucket, configurable)
- [ ] OpenAPI spec auto-generated with `utoipa`
- [ ] Docker image: `FROM scratch`, single binary, < 20MB

### Definition of Done
- HTTP API passes same BDD scenarios as CLI (shared step definitions)
- Docker image builds in CI and is pushed to GHCR

---

## Ongoing practices across all iterations

### Before writing any code in an iteration
1. Write the `.feature` scenarios first
2. Run `cargo test` — new scenarios should fail (`@wip`)
3. Only then write implementation

### Before merging any PR
- `cargo test` green (including BDD)
- `cargo clippy -- -D warnings` clean
- `cargo fmt --check` passes
- No regression in benchmark numbers (automated check in CI)
- `CHANGELOG.md` updated

### Dependency policy
- Prefer crates with > 1M downloads/month or explicit maintenance commitment
- No `unsafe` outside of explicitly justified, reviewed blocks
- Dependency additions require a comment in `Cargo.toml` explaining why

---

## Milestone summary

| Milestone | Iterations | Deliverable |
|-----------|-----------|-------------|
| **v0.1 — Core** | 0–3 | CLI applying all three formats, exact matching |
| **v0.2 — Robust** | 4–5 | Fuzzy matching + AST validation |
| **v0.3 — Pipeline-ready** | 6 | Full JSON API, composable |
| **v0.4 — Credible** | 7 | Public benchmark suite |
| **v1.0 — API** | 8 | HTTP service, Docker image |
