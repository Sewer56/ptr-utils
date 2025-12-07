# ptr-utils

A lightweight library providing utilities for working with raw pointers.

# Project Structure

- `ptr-utils/` - Main library crate
  - `src/` - Library source code
  - `src/unaligned/` - Unaligned read/write implementations

# Code Guidelines

- Optimize for performance; use zero-cost abstractions, avoid allocations.
- Keep modules under 500 lines (excluding tests); split if larger.
- Place `use` inside functions only for `#[cfg]` conditional compilation.
- Prefer `core` over `std` where possible (`core::mem` over `std::mem`).

# Documentation Standards

- Document public items with `///`
- Add examples in docs where helpful
- Use `//!` for module-level docs
- Focus comments on "why" not "what"
- Use [`TypeName`] rustdoc links, not backticks.

# Post-Change Verification

```bash
cargo test --workspace --all-features
cargo clippy --workspace --all-features -- -D warnings
cargo doc --workspace --all-features
cargo fmt --all
cargo publish --dry-run -p ptr-utils
```

All must pass before submitting.
