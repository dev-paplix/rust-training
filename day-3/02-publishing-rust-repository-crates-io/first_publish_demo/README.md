# First Publish Demo

A simple demonstration library for learning how to publish Rust crates to crates.io.

## Overview

This crate provides basic utility functions to demonstrate all the requirements and best practices for publishing a Rust crate to the official package registry.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
first_publish_demo = "0.1.0"
```

## Quick Start

```rust
use first_publish_demo::{greet, add, multiply, Calculator};

fn main() {
    // Simple greeting
    println!("{}", greet("World"));  // Hello, World!

    // Math operations
    let sum = add(2, 3);             // 5
    let product = multiply(4, 5);    // 20

    // Using the calculator
    let mut calc = Calculator::new();
    calc.add(10);
    calc.multiply(2);
    println!("Result: {}", calc.result());  // Result: 20
}
```

## Features

This crate demonstrates:

- ✅ **Complete Cargo.toml**: All required and recommended metadata fields
- ✅ **Dual Licensing**: MIT OR Apache-2.0 licenses included
- ✅ **Comprehensive Documentation**: Crate-level and function-level docs with examples
- ✅ **Doc Tests**: All examples are tested automatically
- ✅ **Unit Tests**: Full test coverage
- ✅ **README**: Installation and usage instructions
- ✅ **API Design**: Simple, well-documented public API

## API Reference

### Functions

- `greet(name: &str) -> String` - Returns a greeting message
- `add(a: i32, b: i32) -> i32` - Adds two numbers
- `multiply(a: i32, b: i32) -> i32` - Multiplies two numbers
- `is_even(n: i32) -> bool` - Checks if a number is even
- `reverse_string(s: &str) -> String` - Reverses a string

### Types

- `Calculator` - A simple calculator struct with basic operations
  - `new()` - Creates a new calculator
  - `add(n: i32)` - Adds to the current value
  - `multiply(n: i32)` - Multiplies the current value
  - `result() -> i32` - Returns the current value
  - `reset()` - Resets to zero

## Examples

### Basic Math Operations

```rust
use first_publish_demo::{add, multiply, is_even};

let sum = add(10, 20);
let product = multiply(5, 6);
let even = is_even(42);

assert_eq!(sum, 30);
assert_eq!(product, 30);
assert!(even);
```

### Using the Calculator

```rust
use first_publish_demo::Calculator;

let mut calc = Calculator::new();
calc.add(100);
calc.multiply(2);
calc.add(-50);

assert_eq!(calc.result(), 150);
```

### String Operations

```rust
use first_publish_demo::reverse_string;

let reversed = reverse_string("Hello");
assert_eq!(reversed, "olleH");
```

## Publishing Checklist

This crate demonstrates all requirements for publishing to crates.io:

### Required in Cargo.toml
- [x] `name` - Unique crate name
- [x] `version` - Semantic version (0.1.0)
- [x] `edition` - Rust edition (2021)
- [x] `description` - Brief description
- [x] `license` - License identifier

### Recommended in Cargo.toml
- [x] `authors` - Author information
- [x] `repository` - GitHub repository URL
- [x] `documentation` - docs.rs URL
- [x] `readme` - README.md
- [x] `keywords` - Search keywords (max 5)
- [x] `categories` - Classification categories

### Files
- [x] LICENSE-MIT - MIT license text
- [x] LICENSE-APACHE - Apache 2.0 license text
- [x] README.md - User documentation
- [x] src/lib.rs - Library code with documentation

### Code Quality
- [x] Comprehensive documentation with examples
- [x] All doc tests passing
- [x] Unit tests with good coverage
- [x] No clippy warnings
- [x] Code formatted with rustfmt

## Documentation

For full API documentation, visit [docs.rs/first_publish_demo](https://docs.rs/first_publish_demo) (once published).

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run doc tests
cargo test --doc

# Run with verbose output
cargo test -- --nocapture
```

## Building Documentation

Generate and view documentation locally:

```bash
# Build and open docs
cargo doc --open

# Build without dependencies
cargo doc --no-deps
```

## Publishing Process

This crate was published following these steps:

1. **Prepare Cargo.toml** - Added all required metadata
2. **Add Licenses** - Created LICENSE-MIT and LICENSE-APACHE files
3. **Write README** - Created this README with examples
4. **Document Code** - Added comprehensive doc comments
5. **Test Everything** - Ensured all tests pass
6. **Package** - Ran `cargo package --list` to verify contents
7. **Dry Run** - Ran `cargo publish --dry-run` to test
8. **Publish** - Ran `cargo publish` to upload to crates.io

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

This is a demonstration crate for educational purposes. However, if you find issues or have suggestions, feel free to open an issue on the repository.

## Learning Resources

- [Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [crates.io](https://crates.io)
- [docs.rs](https://docs.rs)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
