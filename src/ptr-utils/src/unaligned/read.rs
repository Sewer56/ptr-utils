//! Unaligned read operations for pointer types.

/// Trait providing convenient unaligned read operations for pointer types.
///
/// This trait eliminates the need for explicit casts when reading from
/// typed pointers (e.g., `*const u32`, `*mut u16`) by providing methods that handle
/// the casting internally.
pub trait UnalignedRead {
    // Unsigned integer types

    /// Reads a [`u8`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the read
    unsafe fn read_u8_at(self, byte_offset: usize) -> u8;

    /// Reads a [`u16`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 2 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_u16_at(self, byte_offset: usize) -> u16;

    /// Reads a [`u32`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_u32_at(self, byte_offset: usize) -> u32;

    /// Reads a [`u64`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_u64_at(self, byte_offset: usize) -> u64;

    /// Reads a [`u128`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 16 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_u128_at(self, byte_offset: usize) -> u128;

    /// Reads a [`usize`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading [`size_of::<usize>()`] bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_usize_at(self, byte_offset: usize) -> usize;

    // Signed integer types

    /// Reads an [`i8`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the read
    unsafe fn read_i8_at(self, byte_offset: usize) -> i8;

    /// Reads an [`i16`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 2 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_i16_at(self, byte_offset: usize) -> i16;

    /// Reads an [`i32`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_i32_at(self, byte_offset: usize) -> i32;

    /// Reads an [`i64`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_i64_at(self, byte_offset: usize) -> i64;

    /// Reads an [`i128`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 16 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_i128_at(self, byte_offset: usize) -> i128;

    /// Reads an [`isize`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading [`size_of::<isize>()`] bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_isize_at(self, byte_offset: usize) -> isize;

    // Floating point types

    /// Reads an [`f32`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_f32_at(self, byte_offset: usize) -> f32;

    /// Reads an [`f64`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - No alignment requirements - this performs unaligned reads
    unsafe fn read_f64_at(self, byte_offset: usize) -> f64;

    // Boolean type

    /// Reads a [`bool`] value from the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for reading 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the read
    /// - The byte value must represent a valid [`bool`] (0 or 1)
    unsafe fn read_bool_at(self, byte_offset: usize) -> bool;
}

// Implementations for const pointers
impl<T> UnalignedRead for *const T {
    #[inline(always)]
    unsafe fn read_u8_at(self, byte_offset: usize) -> u8 {
        (self as *const u8).add(byte_offset).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u16_at(self, byte_offset: usize) -> u16 {
        ((self as *const u8).add(byte_offset) as *const u16).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u32_at(self, byte_offset: usize) -> u32 {
        ((self as *const u8).add(byte_offset) as *const u32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u64_at(self, byte_offset: usize) -> u64 {
        ((self as *const u8).add(byte_offset) as *const u64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u128_at(self, byte_offset: usize) -> u128 {
        ((self as *const u8).add(byte_offset) as *const u128).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_usize_at(self, byte_offset: usize) -> usize {
        ((self as *const u8).add(byte_offset) as *const usize).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i8_at(self, byte_offset: usize) -> i8 {
        ((self as *const u8).add(byte_offset) as *const i8).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i16_at(self, byte_offset: usize) -> i16 {
        ((self as *const u8).add(byte_offset) as *const i16).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i32_at(self, byte_offset: usize) -> i32 {
        ((self as *const u8).add(byte_offset) as *const i32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i64_at(self, byte_offset: usize) -> i64 {
        ((self as *const u8).add(byte_offset) as *const i64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i128_at(self, byte_offset: usize) -> i128 {
        ((self as *const u8).add(byte_offset) as *const i128).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_isize_at(self, byte_offset: usize) -> isize {
        ((self as *const u8).add(byte_offset) as *const isize).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_f32_at(self, byte_offset: usize) -> f32 {
        ((self as *const u8).add(byte_offset) as *const f32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_f64_at(self, byte_offset: usize) -> f64 {
        ((self as *const u8).add(byte_offset) as *const f64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_bool_at(self, byte_offset: usize) -> bool {
        ((self as *const u8).add(byte_offset) as *const bool).read_unaligned()
    }
}

// Implementations for mutable pointers (read operations)
impl<T> UnalignedRead for *mut T {
    #[inline(always)]
    unsafe fn read_u8_at(self, byte_offset: usize) -> u8 {
        (self as *const u8).add(byte_offset).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u16_at(self, byte_offset: usize) -> u16 {
        ((self as *const u8).add(byte_offset) as *const u16).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u32_at(self, byte_offset: usize) -> u32 {
        ((self as *const u8).add(byte_offset) as *const u32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u64_at(self, byte_offset: usize) -> u64 {
        ((self as *const u8).add(byte_offset) as *const u64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_u128_at(self, byte_offset: usize) -> u128 {
        ((self as *const u8).add(byte_offset) as *const u128).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_usize_at(self, byte_offset: usize) -> usize {
        ((self as *const u8).add(byte_offset) as *const usize).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i8_at(self, byte_offset: usize) -> i8 {
        ((self as *const u8).add(byte_offset) as *const i8).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i16_at(self, byte_offset: usize) -> i16 {
        ((self as *const u8).add(byte_offset) as *const i16).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i32_at(self, byte_offset: usize) -> i32 {
        ((self as *const u8).add(byte_offset) as *const i32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i64_at(self, byte_offset: usize) -> i64 {
        ((self as *const u8).add(byte_offset) as *const i64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_i128_at(self, byte_offset: usize) -> i128 {
        ((self as *const u8).add(byte_offset) as *const i128).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_isize_at(self, byte_offset: usize) -> isize {
        ((self as *const u8).add(byte_offset) as *const isize).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_f32_at(self, byte_offset: usize) -> f32 {
        ((self as *const u8).add(byte_offset) as *const f32).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_f64_at(self, byte_offset: usize) -> f64 {
        ((self as *const u8).add(byte_offset) as *const f64).read_unaligned()
    }

    #[inline(always)]
    unsafe fn read_bool_at(self, byte_offset: usize) -> bool {
        ((self as *const u8).add(byte_offset) as *const bool).read_unaligned()
    }
}
