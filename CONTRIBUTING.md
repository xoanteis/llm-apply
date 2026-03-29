# Contributing to llm-apply

## Core principle: BDD-first

Every new behaviour starts as a `.feature` file scenario, not as code. This is not optional — it's how we ensure the tool does what it says, not just what we happened to implement.

```
1. Write the .feature scenario (describe behaviour in plain English)
2. Run cargo test → scenario fails (expected)
3. Write the minimum code to make it pass
4. Refactor if needed
5. Run full suite — no regressions
6. Open PR
```

---

## Project structure

```
llm-apply/
├── src/
│   ├── main.rs          # CLI entry point, argument parsing
│   ├── lib.rs           # Public API surface
│   ├── apply/
│   │   ├── mod.rs       # ApplyEngine — orchestrates the apply pipeline
│   │   ├── formats.rs   # ChangeFormat enum + auto-detection
│   │   ├── diff.rs      # Unified diff parsing and application
│   │   ├── search_replace.rs
│   │   ├── rewrite.rs
│   │   ├── fuzzy.rs     # FuzzyMatcher
│   │   └── atomic.rs    # Atomic file write primitives
│   ├── validate/
│   │   ├── mod.rs
│   │   └── ast.rs       # Tree-sitter integration
│   └── cli/
│       ├── mod.rs
│       └── output.rs    # Human + JSON output formatting
├── tests/
│   ├── features/        # .feature files (BDD scenarios)
│   ├── steps/           # Step definitions (Rust)
│   └── fixtures/        # Test files used in scenarios
├── benches/
│   ├── apply_bench.rs   # Criterion benchmarks
│   └── results.md       # Recorded benchmark results
└── docs/
    └── exit-codes.md
```

---

## Running tests

```bash
# All tests including BDD
cargo test

# BDD only
cargo test --test bdd

# Only scenarios not tagged @wip
cargo test --test bdd -- --tags "not @wip"

# Benchmarks
cargo bench
```

---

## Writing feature files

Feature files live in `tests/features/`. Follow this structure:

```gherkin
Feature: <behaviour being described>

  Background:            # optional shared setup
    Given ...

  Scenario: <specific case>
    Given <precondition>
    When  <action>
    Then  <observable outcome>
    And   <additional outcome>

  Scenario: <edge case — especially failure modes>
    ...
```

**Rules for good scenarios:**
- One behaviour per scenario
- Failure modes are as important as happy paths — always add a scenario for how the tool fails
- Use concrete values, not abstract descriptions
- Scenarios should be readable by someone who doesn't know Rust

---

## Code style

- Run `cargo fmt` before committing (CI enforces this)
- `cargo clippy -- -D warnings` must pass — no exceptions
- No `unwrap()` or `expect()` outside of tests — use `?` and proper error types
- Error messages must be actionable: tell the user what to do, not just what went wrong
  - ❌ `"hunk did not match"`
  - ✅ `"hunk did not match at line 42 — the file may have changed since the diff was generated. Try --fuzzy or regenerate the change."`

---

## Benchmark discipline

Before merging any change to the apply pipeline:

```bash
cargo bench -- --save-baseline before
# make your changes
cargo bench -- --baseline before
```

If p99 latency regresses by > 10%, the PR needs a justification comment.

---

## Commit messages

```
<type>: <short description>

<optional body>
```

Types: `feat`, `fix`, `test`, `bench`, `docs`, `chore`, `refactor`

Examples:
```
feat: add search/replace format support
test: add BDD scenarios for ambiguous search blocks
fix: rollback file on AST validation failure
bench: record baseline for fuzzy matching on 500 LOC files
```

---

## Opening a PR

- PR title = commit message format
- Reference the iteration from PLAN.md: `Part of Iteration 4 — Fuzzy Matching`
- All CI checks must be green
- If adding a new `.feature` file, tag new unimplemented scenarios with `@wip`
