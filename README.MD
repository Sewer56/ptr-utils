# ptr-utils

[![Crates.io](https://img.shields.io/crates/v/ptr-utils.svg)](https://crates.io/crates/ptr-utils)
[![Documentation](https://docs.rs/ptr-utils/badge.svg)](https://docs.rs/ptr-utils)
[![CI](https://github.com/Sewer56/ptr-utils/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/ptr-utils/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

## Why This Library?

This library exists primarily for ergonomics. Compare writing pointer operations manually versus using `ptr-utils`:

**Before** (manual pointer casting):
```rust
/// # Safety
///
/// - input_ptr must be valid for reads of len bytes
/// - output_ptr must be valid for writes of len bytes
pub(crate) unsafe fn u32_untransform_with_separate_pointers(
    mut alpha_byte_in_ptr: *const u16,
    mut alpha_bit_in_ptr: *const u16,
    mut color_byte_in_ptr: *const u32,
    mut index_byte_in_ptr: *const u32,
    mut current_output_ptr: *mut u8,
    len: usize,
) {
    let alpha_byte_end_ptr = alpha_byte_in_ptr.add(len / 16);
    while alpha_byte_in_ptr < alpha_byte_end_ptr {
        // Alpha bytes (2 bytes)
        (current_output_ptr as *mut u16).write_unaligned(alpha_byte_in_ptr.read_unaligned());
        alpha_byte_in_ptr = alpha_byte_in_ptr.add(1);

        // Alpha bits (6 bytes)
        (current_output_ptr.add(2) as *mut u16).write_unaligned(alpha_bit_in_ptr.read_unaligned());
        (current_output_ptr.add(4) as *mut u32)
            .write_unaligned((alpha_bit_in_ptr.add(1) as *const u32).read_unaligned());
        alpha_bit_in_ptr = alpha_bit_in_ptr.add(3);

        // Color bytes (4 bytes)
        (current_output_ptr.add(8) as *mut u32).write_unaligned(color_byte_in_ptr.read_unaligned());
        color_byte_in_ptr = color_byte_in_ptr.add(1);

        // Index bytes (4 bytes)
        (current_output_ptr.add(12) as *mut u32)
            .write_unaligned(index_byte_in_ptr.read_unaligned());
        index_byte_in_ptr = index_byte_in_ptr.add(1);
        current_output_ptr = current_output_ptr.add(16);
    }
}
```

**After** (with `ptr-utils`):

```rust
use ptr_utils::{UnalignedRead, UnalignedWrite};

/// # Safety
///
/// - input_ptr must be valid for reads of len bytes
/// - output_ptr must be valid for writes of len bytes
pub(crate) unsafe fn u32_untransform_with_separate_pointers(
    mut alpha_byte_in_ptr: *const u16,
    mut alpha_bit_in_ptr: *const u16,
    mut color_byte_in_ptr: *const u32,
    mut index_byte_in_ptr: *const u32,
    mut current_output_ptr: *mut u8,
    len: usize,
) {
    let alpha_byte_end_ptr = alpha_byte_in_ptr.add(len / 16);
    while alpha_byte_in_ptr < alpha_byte_end_ptr {
        // Alpha bytes (2 bytes)
        current_output_ptr.write_u16_at(0, alpha_byte_in_ptr.read_u16_at(0));
        alpha_byte_in_ptr = alpha_byte_in_ptr.add(1);

        // Alpha bits (6 bytes)
        current_output_ptr.write_u16_at(2, alpha_bit_in_ptr.read_u16_at(0));
        current_output_ptr.write_u32_at(4, alpha_bit_in_ptr.read_u32_at(2));
        alpha_bit_in_ptr = alpha_bit_in_ptr.add(3);

        // Color bytes (4 bytes)
        current_output_ptr.write_u32_at(8, color_byte_in_ptr.read_u32_at(0));
        color_byte_in_ptr = color_byte_in_ptr.add(1);

        // Index bytes (4 bytes)
        current_output_ptr.write_u32_at(12, index_byte_in_ptr.read_u32_at(0));
        index_byte_in_ptr = index_byte_in_ptr.add(1);
        current_output_ptr = current_output_ptr.add(16);
    }
}
```

The library eliminates repetitive `as *mut T` and `as *const T` casts while maintaining the same safety contracts and zero-cost abstractions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ptr-utils = "0.1.0"
```

For `no_std` environments:

```toml
[dependencies]
ptr-utils = { version = "0.1.0", default-features = false }
```

## Usage

### Unaligned Read/Write Operations

Works with byte pointers as usual:

```rust
use ptr_utils::{UnalignedRead, UnalignedWrite};

let mut buffer = [0u8; 16];
let ptr = buffer.as_mut_ptr();

unsafe {
    // Write a u32 at byte offset 1 (unaligned)
    ptr.write_u32_at(1, 0x12345678);
    
    // Read it back
    let value = ptr.read_u32_at(1);
    assert_eq!(value, 0x12345678);
}
```

But you can also use it with pointers for other basic types:

```rust
use ptr_utils::{UnalignedRead, UnalignedWrite};

// Works with any pointer type
let mut data: [u64; 4] = [0; 4];
let typed_ptr: *mut u64 = data.as_mut_ptr();

unsafe {
    // No need to cast to *mut u8 first
    typed_ptr.write_u16_at(3, 0xABCD); // byte offset 3
    let value = typed_ptr.read_u16_at(3);
    assert_eq!(value, 0xABCD);
}
```

The operations always take byte offsets; for simplicity.

## API Overview

### `UnalignedRead` Trait

Provides unaligned read operations for both `*const T` and `*mut T`:

- `read_u8_at`, `read_u16_at`, `read_u32_at`, `read_u64_at`, `read_u128_at`, `read_usize_at`
- `read_i8_at`, `read_i16_at`, `read_i32_at`, `read_i64_at`, `read_i128_at`, `read_isize_at`
- `read_f32_at`, `read_f64_at`
- `read_bool_at`

### `UnalignedWrite` Trait

Provides unaligned write operations for `*mut T`:

- `write_u8_at`, `write_u16_at`, `write_u32_at`, `write_u64_at`, `write_u128_at`, `write_usize_at`
- `write_i8_at`, `write_i16_at`, `write_i32_at`, `write_i64_at`, `write_i128_at`, `write_isize_at`
- `write_f32_at`, `write_f64_at`
- `write_bool_at`

## Safety

This library provides `unsafe` functions that require careful use:

- Ensure pointers are valid for the requested read/write operation
- Verify that the memory range `[ptr + offset, ptr + offset + size_of::<T>())` is accessible
- For writes, ensure the memory is mutable
- The caller is responsible for preventing data races in multi-threaded contexts

## Cargo Features

- `std` (default): Enables standard library support
- Default features can be disabled for `no_std` environments

## Development

For information on how to work with this codebase, see [README-DEV.MD](README-DEV.MD).

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/ptr-utils).

## License

Licensed under [MIT](./LICENSE).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

[codecov]: https://about.codecov.io/
[crates-io-key]: https://crates.io/settings/tokens
[nuget-key]: https://www.nuget.org/account/apikeys
[reloaded-license]: https://reloaded-project.github.io/Reloaded.MkDocsMaterial.Themes.R2/Pages/license/