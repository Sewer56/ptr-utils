//! Unaligned write operations for pointer types.

/// Trait providing convenient unaligned write operations for mutable pointer types.
pub trait UnalignedWrite {
    // Unsigned integer types

    /// Writes a [`u8`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    unsafe fn write_u8_at(self, byte_offset: usize, value: u8);

    /// Writes a [`u16`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 2 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_u16_at(self, byte_offset: usize, value: u16);

    /// Writes a [`u32`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_u32_at(self, byte_offset: usize, value: u32);

    /// Writes a [`u64`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_u64_at(self, byte_offset: usize, value: u64);

    /// Writes a [`u128`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 16 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_u128_at(self, byte_offset: usize, value: u128);

    /// Writes a [`usize`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing [`size_of::<usize>()`] bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_usize_at(self, byte_offset: usize, value: usize);

    // Signed integer types

    /// Writes an [`i8`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    unsafe fn write_i8_at(self, byte_offset: usize, value: i8);

    /// Writes an [`i16`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 2 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_i16_at(self, byte_offset: usize, value: i16);

    /// Writes an [`i32`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_i32_at(self, byte_offset: usize, value: i32);

    /// Writes an [`i64`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_i64_at(self, byte_offset: usize, value: i64);

    /// Writes an [`i128`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 16 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_i128_at(self, byte_offset: usize, value: i128);

    /// Writes an [`isize`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing [`size_of::<isize>()`] bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_isize_at(self, byte_offset: usize, value: isize);

    // Floating point types

    /// Writes an [`f32`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 4 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_f32_at(self, byte_offset: usize, value: f32);

    /// Writes an [`f64`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 8 bytes
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    /// - No alignment requirements - this performs unaligned writes
    unsafe fn write_f64_at(self, byte_offset: usize, value: f64);

    // Boolean type

    /// Writes a [`bool`] value to the pointer at the given byte offset.
    ///
    /// # Safety
    /// - The pointer plus byte offset must be valid for writing 1 byte
    /// - The caller must ensure the pointer remains valid for the duration of the write
    /// - The memory location must be mutable
    unsafe fn write_bool_at(self, byte_offset: usize, value: bool);
}

impl<T> UnalignedWrite for *mut T {
    #[inline(always)]
    unsafe fn write_u8_at(self, byte_offset: usize, value: u8) {
        (self as *mut u8).add(byte_offset).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_u16_at(self, byte_offset: usize, value: u16) {
        ((self as *mut u8).add(byte_offset) as *mut u16).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_u32_at(self, byte_offset: usize, value: u32) {
        ((self as *mut u8).add(byte_offset) as *mut u32).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_u64_at(self, byte_offset: usize, value: u64) {
        ((self as *mut u8).add(byte_offset) as *mut u64).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_u128_at(self, byte_offset: usize, value: u128) {
        ((self as *mut u8).add(byte_offset) as *mut u128).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_usize_at(self, byte_offset: usize, value: usize) {
        ((self as *mut u8).add(byte_offset) as *mut usize).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_i8_at(self, byte_offset: usize, value: i8) {
        ((self as *mut u8).add(byte_offset) as *mut i8).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_i16_at(self, byte_offset: usize, value: i16) {
        ((self as *mut u8).add(byte_offset) as *mut i16).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_i32_at(self, byte_offset: usize, value: i32) {
        ((self as *mut u8).add(byte_offset) as *mut i32).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_i64_at(self, byte_offset: usize, value: i64) {
        ((self as *mut u8).add(byte_offset) as *mut i64).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_i128_at(self, byte_offset: usize, value: i128) {
        ((self as *mut u8).add(byte_offset) as *mut i128).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_isize_at(self, byte_offset: usize, value: isize) {
        ((self as *mut u8).add(byte_offset) as *mut isize).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_f32_at(self, byte_offset: usize, value: f32) {
        ((self as *mut u8).add(byte_offset) as *mut f32).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_f64_at(self, byte_offset: usize, value: f64) {
        ((self as *mut u8).add(byte_offset) as *mut f64).write_unaligned(value);
    }

    #[inline(always)]
    unsafe fn write_bool_at(self, byte_offset: usize, value: bool) {
        ((self as *mut u8).add(byte_offset) as *mut bool).write_unaligned(value);
    }
}
