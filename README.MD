# ptr-utils

[![Crates.io](https://img.shields.io/crates/v/ptr-utils.svg)](https://crates.io/crates/ptr-utils)
[![Documentation](https://docs.rs/ptr-utils/badge.svg)](https://docs.rs/ptr-utils)
[![CI](https://github.com/Sewer56/ptr-utils/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/ptr-utils/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

A lightweight, `no_std` compatible library providing utilities for working with raw pointers in Rust.

## Features

Provides miscellaneous utility methods for raw pointers to write cleaner code:

- **Unaligned Operations**: Read/write primitives without alignment requirements
- **Zero-Cost**: All operations inline to the same assembly as manual casts
- **Type Safety**: Avoid explicit pointer casts while keeping safety contracts
- **All Primitives**: Support for integers, floats, and booleans
- **`no_std` Compatible**: Works in embedded environments

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

### Basic Example

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

### Working with Different Pointer Types

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

### Parsing Binary Data

```rust
use ptr_utils::UnalignedRead;

fn parse_header(data: &[u8]) -> (u32, u16, u64) {
    let ptr = data.as_ptr();
    unsafe {
        let magic = ptr.read_u32_at(0);      // byte offset 0
        let version = ptr.read_u16_at(4);    // byte offset 4
        let timestamp = ptr.read_u64_at(6);  // byte offset 6
        (magic, version, timestamp)
    }
}
```

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

[Learn more about Reloaded's general choice of licensing for projects.][reloaded-license].

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

[codecov]: https://about.codecov.io/
[crates-io-key]: https://crates.io/settings/tokens
[nuget-key]: https://www.nuget.org/account/apikeys
[reloaded-license]: https://reloaded-project.github.io/Reloaded.MkDocsMaterial.Themes.R2/Pages/license/