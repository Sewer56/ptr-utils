//! Convenient unaligned read/write operations for various pointer types.
//!
//! This module provides a trait-based solution to avoid explicit pointer casts
//! when performing unaligned reads and writes on all common Rust primitive types.

/// Trait providing convenient unaligned read operations for pointer types.
///
/// This trait eliminates the need for explicit casts when reading from
/// typed pointers (e.g., `*const u32`, `*mut u16`) by providing methods that handle
/// the casting internally.
pub trait UnalignedReadWrite {
    // Unsigned integer types
    unsafe fn read_u8_at(self, offset: usize) -> u8;
    unsafe fn read_u16_at(self, offset: usize) -> u16;
    unsafe fn read_u32_at(self, offset: usize) -> u32;
    unsafe fn read_u64_at(self, offset: usize) -> u64;
    unsafe fn read_u128_at(self, offset: usize) -> u128;
    unsafe fn read_usize_at(self, offset: usize) -> usize;
    
    // Signed integer types
    unsafe fn read_i8_at(self, offset: usize) -> i8;
    unsafe fn read_i16_at(self, offset: usize) -> i16;
    unsafe fn read_i32_at(self, offset: usize) -> i32;
    unsafe fn read_i64_at(self, offset: usize) -> i64;
    unsafe fn read_i128_at(self, offset: usize) -> i128;
    unsafe fn read_isize_at(self, offset: usize) -> isize;
    
    // Floating point types
    unsafe fn read_f32_at(self, offset: usize) -> f32;
    unsafe fn read_f64_at(self, offset: usize) -> f64;
    
    // Boolean type
    unsafe fn read_bool_at(self, offset: usize) -> bool;
}

/// Trait providing convenient unaligned write operations for mutable pointer types.
pub trait UnalignedWrite {
    // Unsigned integer types
    unsafe fn write_u8_at(self, offset: usize, value: u8);
    unsafe fn write_u16_at(self, offset: usize, value: u16);
    unsafe fn write_u32_at(self, offset: usize, value: u32);
    unsafe fn write_u64_at(self, offset: usize, value: u64);
    unsafe fn write_u128_at(self, offset: usize, value: u128);
    unsafe fn write_usize_at(self, offset: usize, value: usize);
    
    // Signed integer types
    unsafe fn write_i8_at(self, offset: usize, value: i8);
    unsafe fn write_i16_at(self, offset: usize, value: i16);
    unsafe fn write_i32_at(self, offset: usize, value: i32);
    unsafe fn write_i64_at(self, offset: usize, value: i64);
    unsafe fn write_i128_at(self, offset: usize, value: i128);
    unsafe fn write_isize_at(self, offset: usize, value: isize);
    
    // Floating point types
    unsafe fn write_f32_at(self, offset: usize, value: f32);
    unsafe fn write_f64_at(self, offset: usize, value: f64);
    
    // Boolean type
    unsafe fn write_bool_at(self, offset: usize, value: bool);
}