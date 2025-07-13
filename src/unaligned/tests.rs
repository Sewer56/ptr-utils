use core::f32;
use core::f64;

use super::*;

#[test]
fn test_unsigned_integers() {
    unsafe {
        let mut buffer = [0u8; 64];
        let ptr = buffer.as_mut_ptr();

        // Test writing
        ptr.write_u8_at(0, 0x12);
        ptr.write_u16_at(1, 0x1234);
        ptr.write_u32_at(4, 0x12345678);
        ptr.write_u64_at(8, 0x123456789ABCDEF0);
        ptr.write_u128_at(16, 0x123456789ABCDEF0FEDCBA0987654321);
        ptr.write_usize_at(32, 0x12345678);

        // Test reading
        assert_eq!(ptr.read_u8_at(0), 0x12);
        assert_eq!(ptr.read_u16_at(1), 0x1234);
        assert_eq!(ptr.read_u32_at(4), 0x12345678);
        assert_eq!(ptr.read_u64_at(8), 0x123456789ABCDEF0);
        assert_eq!(ptr.read_u128_at(16), 0x123456789ABCDEF0FEDCBA0987654321);
        assert_eq!(ptr.read_usize_at(32), 0x12345678);
    }
}

#[test]
fn test_signed_integers() {
    unsafe {
        let mut buffer = [0u8; 64];
        let ptr = buffer.as_mut_ptr();

        // Test writing negative values
        ptr.write_i8_at(0, -12);
        ptr.write_i16_at(1, -1234);
        ptr.write_i32_at(4, -12345678);
        ptr.write_i64_at(8, -123456789012345678);
        ptr.write_i128_at(16, -12345678901234567890123456789012345678);
        ptr.write_isize_at(32, -12345678);

        // Test reading
        assert_eq!(ptr.read_i8_at(0), -12);
        assert_eq!(ptr.read_i16_at(1), -1234);
        assert_eq!(ptr.read_i32_at(4), -12345678);
        assert_eq!(ptr.read_i64_at(8), -123456789012345678);
        assert_eq!(
            ptr.read_i128_at(16),
            -12345678901234567890123456789012345678
        );
        assert_eq!(ptr.read_isize_at(32), -12345678);
    }
}

#[test]
fn test_floating_point() {
    unsafe {
        let mut buffer = [0u8; 32];
        let ptr = buffer.as_mut_ptr();

        let f32_val = f32::consts::PI;
        let f64_val = f64::consts::E;

        // Test writing
        ptr.write_f32_at(0, f32_val);
        ptr.write_f64_at(8, f64_val);

        // Test reading
        assert_eq!(ptr.read_f32_at(0), f32_val);
        assert_eq!(ptr.read_f64_at(8), f64_val);
    }
}

#[test]
fn test_boolean() {
    unsafe {
        let mut buffer = [0u8; 8];
        let ptr = buffer.as_mut_ptr();

        // Test writing
        ptr.write_bool_at(0, true);
        ptr.write_bool_at(1, false);

        // Test reading
        assert!(ptr.read_bool_at(0));
        assert!(!ptr.read_bool_at(1));
    }
}

#[test]
fn test_with_typed_pointers() {
    unsafe {
        let mut buffer = [0u8; 32];
        let ptr = buffer.as_mut_ptr();

        // Test with different typed pointers
        let u32_ptr = ptr as *mut u32;
        let u64_ptr = ptr as *mut u64;

        u32_ptr.write_u16_at(0, 0x1234);
        u64_ptr.write_u32_at(4, 0x56789ABC);

        assert_eq!(u32_ptr.read_u16_at(0), 0x1234);
        assert_eq!(u64_ptr.read_u32_at(4), 0x56789ABC);
    }
}

#[test]
fn test_const_pointers() {
    unsafe {
        let buffer = [0x12u8, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0];
        let ptr = buffer.as_ptr();

        // Test reading from const pointer
        assert_eq!(ptr.read_u8_at(0), 0x12);
        assert_eq!(ptr.read_u16_at(0), u16::from_le_bytes([0x12, 0x34]));
        assert_eq!(
            ptr.read_u32_at(0),
            u32::from_le_bytes([0x12, 0x34, 0x56, 0x78])
        );
        assert_eq!(
            ptr.read_u64_at(0),
            u64::from_le_bytes([0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0])
        );
    }
}

#[test]
fn test_unaligned_access() {
    unsafe {
        let mut buffer = [0u8; 16];
        let ptr = buffer.as_mut_ptr();

        // Write at unaligned offsets
        ptr.write_u32_at(1, 0x12345678); // Unaligned 4-byte write
        ptr.write_u64_at(5, 0x9ABCDEF012345678); // Unaligned 8-byte write

        // Read back
        assert_eq!(ptr.read_u32_at(1), 0x12345678);
        assert_eq!(ptr.read_u64_at(5), 0x9ABCDEF012345678);
    }
}

#[test]
fn test_round_trip_all_types() {
    unsafe {
        let mut buffer = [0u8; 128];
        let ptr = buffer.as_mut_ptr();
        let mut offset = 0;

        // Test all unsigned types
        let test_u8 = 0xFFu8;
        let test_u16 = 0xFFEEu16;
        let test_u32 = 0xFFEEDDCCu32;
        let test_u64 = 0xFFEEDDCCBBAA9988u64;
        let test_u128 = 0xFFEEDDCCBBAA99887766554433221100u128;
        let test_usize = 0xDEADBEEFusize;

        ptr.write_u8_at(offset, test_u8);
        offset += 1;
        ptr.write_u16_at(offset, test_u16);
        offset += 2;
        ptr.write_u32_at(offset, test_u32);
        offset += 4;
        ptr.write_u64_at(offset, test_u64);
        offset += 8;
        ptr.write_u128_at(offset, test_u128);
        offset += 16;
        ptr.write_usize_at(offset, test_usize);

        offset = 0;
        assert_eq!(ptr.read_u8_at(offset), test_u8);
        offset += 1;
        assert_eq!(ptr.read_u16_at(offset), test_u16);
        offset += 2;
        assert_eq!(ptr.read_u32_at(offset), test_u32);
        offset += 4;
        assert_eq!(ptr.read_u64_at(offset), test_u64);
        offset += 8;
        assert_eq!(ptr.read_u128_at(offset), test_u128);
        offset += 16;
        assert_eq!(ptr.read_usize_at(offset), test_usize);
        offset += 8;

        // Test all signed types
        let test_i8 = -128i8;
        let test_i16 = -32768i16;
        let test_i32 = -2147483648i32;
        let test_i64 = -9223372036854775808i64;
        let test_i128 = -170141183460469231731687303715884105728i128;
        let test_isize = -1234567890isize;

        ptr.write_i8_at(offset, test_i8);
        offset += 1;
        ptr.write_i16_at(offset, test_i16);
        offset += 2;
        ptr.write_i32_at(offset, test_i32);
        offset += 4;
        ptr.write_i64_at(offset, test_i64);
        offset += 8;
        ptr.write_i128_at(offset, test_i128);
        offset += 16;
        ptr.write_isize_at(offset, test_isize);
        offset += 8;

        offset -= 39;
        assert_eq!(ptr.read_i8_at(offset), test_i8);
        offset += 1;
        assert_eq!(ptr.read_i16_at(offset), test_i16);
        offset += 2;
        assert_eq!(ptr.read_i32_at(offset), test_i32);
        offset += 4;
        assert_eq!(ptr.read_i64_at(offset), test_i64);
        offset += 8;
        assert_eq!(ptr.read_i128_at(offset), test_i128);
        offset += 16;
        assert_eq!(ptr.read_isize_at(offset), test_isize);
    }
}
