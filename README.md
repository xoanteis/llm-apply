# llm-apply

> A reliable, standalone primitive for applying LLM-generated code changes.

[![CI](https://github.com/xoanteis/llm-apply/actions/workflows/ci.yml/badge.svg)](https://github.com/xoanteis/llm-apply/actions)
[![Crates.io](https://img.shields.io/crates/v/llm-apply.svg)](https://crates.io/crates/llm-apply)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

LLMs generate code changes in inconsistent formats — unified diffs with wrong line numbers, search/replace blocks with whitespace drift, or full rewrites that hallucinate untouched sections. `llm-apply` is the missing primitive that reliably bridges LLM output and your filesystem.

---

## The problem

Every agentic coding pipeline needs to apply LLM-generated changes to files. Today, each framework (Aider, Continue, LangChain) solves this differently, incompletely, and with silent failures. `llm-apply` is the `curl` of this operation: a single, composable, model-agnostic tool that does one thing well.

---

## Install

```bash
cargo install llm-apply
```

---

## Usage

```bash
# From stdin
cat changes.diff | llm-apply src/main.rs

# Dry run — shows what would change, touches nothing
llm-apply --dry-run src/auth.py < patch.txt

# Explicit format
llm-apply --format search-replace src/lib.rs < changes.txt

# With AST validation (requires language flag or auto-detection)
llm-apply --validate src/parser.rs < changes.diff

# Structured output for pipelines
llm-apply --json src/main.rs < changes.diff
```

### JSON output (for agent pipelines)

```json
{
  "success": true,
  "hunks_applied": 3,
  "hunks_failed": 0,
  "confidence": 0.97,
  "valid_ast": true,
  "diff": "..."
}
```

---

## Supported input formats

| Format | Example |
|--------|---------|
| Unified diff | Standard `--- a/` / `+++ b/` |
| Search/replace | `<<<SEARCH` / `===` / `>>>REPLACE` |
| Full rewrite | Complete file content |
| Auto-detect | Default — infers format from content |

---

## Why Rust

- Zero-runtime dependency — ships as a single binary, drops into any pipeline
- Performance — sub-50ms p50 for files under 500 LOC
- Memory safety — no corrupt state if the process is killed mid-write
- Tree-sitter bindings — native AST validation without a language runtime

---

## Design principles

- **Does one thing** — applies changes. No agent loop, no model calls, no IDE.
- **Fails loudly** — ambiguous or unresolvable changes return a non-zero exit code with a structured reason, never silently corrupt files.
- **Atomic writes** — either the full change is applied or the file is untouched.
- **Composable** — works in pipes, scripts, and CI steps.
- **Model-agnostic** — works with Claude, GPT-4o, Gemini, local models.

---

## Build plan

See [PLAN.md](PLAN.md) for the full iterative development roadmap.

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

---

## License

MIT
