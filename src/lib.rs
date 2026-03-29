//! # llm-apply
//!
//! A reliable, standalone primitive for applying LLM-generated code changes.
//!
//! ## Usage as a library
//!
//! ```rust,no_run
//! use llm_apply::{ApplyEngine, ApplyOptions};
//!
//! let engine = ApplyEngine::default();
//! let result = engine.apply(
//!     "fn hello() {\n    println!(\"hello\");\n}\n",
//!     "--- a/main.rs\n+++ b/main.rs\n...",
//!     ApplyOptions::default(),
//! )?;
//!
//! println!("Applied {} hunks, confidence: {}", result.hunks_applied, result.confidence);
//! # Ok::<(), llm_apply::Error>(())
//! ```

pub mod apply;
pub mod validate;

mod error;
pub use error::Error;

pub use apply::{ApplyEngine, ApplyOptions, ApplyResult};
