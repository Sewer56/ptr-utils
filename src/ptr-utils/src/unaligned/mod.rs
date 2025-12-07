//! Convenient unaligned read/write operations for various pointer types.
//!
//! This module provides a trait-based solution to avoid explicit pointer casts
//! when performing unaligned reads and writes on all common Rust primitive types.

pub mod read;
pub mod write;

#[cfg(test)]
mod tests;

pub use read::UnalignedRead;
pub use write::UnalignedWrite;
