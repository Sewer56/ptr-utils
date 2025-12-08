#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

pub mod unaligned;

pub use unaligned::{UnalignedRead, UnalignedWrite};
