#![doc = include_str!("../README.MD")]
#![no_std]
#[cfg(feature = "std")]
extern crate std;

pub mod unaligned;

pub use unaligned::{UnalignedRead, UnalignedWrite};
