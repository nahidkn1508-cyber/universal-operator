// src/lib.rs
//! # FileManager
//!
//! A deterministic, cross-platform file‑system management crate for the Universal AI Operator.
//! Every public method performs exactly one filesystem operation, returns a rich `Result` type,
//! and never logs, talks to AI, or executes arbitrary commands.
//!
//! ## Features
//!
//! - File creation, reading, writing, appending, copying, moving, deleting.
//! - Directory creation (recursive), copying, moving, deleting (recursive).
//! - Metadata queries: existence, type, size, listing, canonicalisation.
//! - Working directory management.
//! - Temporary files and directories with automatic cleanup.
//! - Comprehensive error handling via `FileError`.
//!
//! ## Example
//!
//! ```rust
//! use filemanager::{FileManager, Result};
//!
//! fn main() -> Result<()> {
//!     FileManager::write_file("hello.txt", b"Hello, world!")?;
//!     let content = FileManager::read_file("hello.txt")?;
//!     assert_eq!(content, "Hello, world!");
//!     Ok(())
//! }
//! ```

mod error;
mod file_manager;
mod temp;
pub(crate) mod utils;

pub use error::{FileError, Result};
pub use file_manager::FileManager;
pub use temp::{TempDir, TempFile};

#[cfg(test)]
mod tests;