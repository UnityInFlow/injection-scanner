//! Static scanner for prompt-injection patterns in agent-facing documents.
//!
//! `unwrap()` is denied crate-wide rather than left to review. CLAUDE.md states
//! "Do not use `unwrap()` in Rust in production code", and issue #19 asked for
//! the lint that makes it true; only the `allowlist.rs` cleanup half shipped.
//! `cargo clippy -- -D warnings` does not catch it on its own — `unwrap_used`
//! is a restriction lint and is off by default, so a new `unwrap()` in this
//! crate passed CI silently.
//!
//! `expect()` is deliberately NOT denied: it carries a message, and the one use
//! in `allowlist.rs` is on a compile-time-constant regex covered by a test.
//! Denying it would push that toward a silent fallback, which is worse.
#![deny(clippy::unwrap_used)]

pub mod allowlist;
pub mod baseline;
pub mod context;
pub mod multiline;
pub mod normalize;
pub mod pattern;
pub mod patterns;
pub mod reporter;
pub mod sarif;
pub mod scanner;
pub mod walk;
