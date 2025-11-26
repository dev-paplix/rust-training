# Creating a Rust Library - Quick Reference

## Library Creation Fundamentals

### Create a New Library
```bash
cargo new my_library --lib
cd my_library
```

### Basic Library Structure
```
my_library/
├── Cargo.toml          # Package configuration
├── src/
│   └── lib.rs         # Library root (entry point)
└── tests/             # Integration tests (optional)
    └── integration_test.rs
```

## Library Configuration (Cargo.toml)

### Basic Configuration
```toml
[package]
name = "my_library"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A brief description of your library"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my_library"
keywords = ["keyword1", "keyword2"]
categories = ["category"]

[dependencies]
# External dependencies go here

[dev-dependencies]
# Test-only dependencies go here
```

## Library Module Organization

| Component | Purpose | File Location |
|-----------|---------|---------------|
| Root Module | Library entry point | `src/lib.rs` |
| Public API | Exported items | `pub` items in `lib.rs` |
| Sub-modules | Code organization | `src/module_name.rs` or `src/module_name/mod.rs` |
| Private helpers | Internal use only | Non-`pub` items |
| Tests | Unit tests | `#[cfg(test)] mod tests` |
| Integration tests | External tests | `tests/*.rs` |

## Visibility and Public API

### Visibility Modifiers
```rust
// Public - accessible from outside
pub fn public_function() { }
pub struct PublicStruct { }

// Private - only within module (default)
fn private_function() { }
struct PrivateStruct { }

// Public in crate only
pub(crate) fn crate_only() { }

// Public in super module
pub(super) fn parent_only() { }
```

### Re-exporting
```rust
// In lib.rs
mod internal_module;

// Re-export for convenience
pub use internal_module::ImportantType;
pub use internal_module::useful_function;
```

## Common Library Patterns

### Simple Library (src/lib.rs)
```rust
//! # My Library
//! 
//! This library provides...

/// Adds two numbers together
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}
```

### Multi-Module Library
```rust
// src/lib.rs
pub mod math;
pub mod string_utils;

// Re-export commonly used items
pub use math::Calculator;
pub use string_utils::format_name;
```

## Documentation

### Module-level Documentation
```rust
//! # Module Name
//! 
//! Brief description of the module.
//! 
//! # Examples
//! 
//! ```
//! use my_library::function;
//! 
//! let result = function();
//! ```
```

### Item Documentation
```rust
/// Brief description
///
/// # Arguments
///
/// * `param` - Description of parameter
///
/// # Returns
///
/// Description of return value
///
/// # Examples
///
/// ```
/// use my_library::function;
/// let result = function(42);
/// assert_eq!(result, 42);
/// ```
///
/// # Panics
///
/// Describes when this function panics
///
/// # Errors
///
/// Describes error conditions (for Result)
pub fn function(param: i32) -> i32 {
    param
}
```

## Testing

### Unit Tests (in lib.rs or module files)
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn it_panics() {
        panic!("Expected panic");
    }
}
```

### Integration Tests (tests/*.rs)
```rust
// tests/integration_test.rs
use my_library;

#[test]
fn test_public_api() {
    let result = my_library::function();
    assert_eq!(result, expected);
}
```

### Run Tests
```bash
cargo test              # Run all tests
cargo test test_name    # Run specific test
cargo test --lib        # Run library unit tests only
cargo test --test integration_test  # Run specific integration test
```

## Building and Publishing

### Build Commands
```bash
cargo build             # Debug build
cargo build --release   # Optimized build
cargo check             # Check without building
cargo doc               # Generate documentation
cargo doc --open        # Generate and open docs
```

### Publishing Preparation
```bash
cargo publish --dry-run  # Test publishing
cargo package           # Create distributable package
cargo login <token>     # Login to crates.io
cargo publish           # Publish to crates.io
```

## Library Types

### Binary Library (both lib and binary)
```toml
# Cargo.toml
[package]
name = "my_project"

[[bin]]
name = "my_binary"
path = "src/main.rs"

[lib]
name = "my_library"
path = "src/lib.rs"
```

### Procedural Macro Library
```toml
[lib]
proc-macro = true

[dependencies]
syn = "2.0"
quote = "1.0"
proc-macro2 = "1.0"
```

## Common Library Features

### Feature Flags
```toml
[features]
default = ["feature1"]
feature1 = []
feature2 = ["dep:optional_dependency"]

[dependencies]
optional_dependency = { version = "1.0", optional = true }
```

```rust
// Use in code
#[cfg(feature = "feature1")]
pub fn feature_specific() { }
```

### Build Features
```bash
cargo build --features "feature1 feature2"
cargo build --no-default-features
cargo build --all-features
```

## Workspace Libraries

### Workspace Configuration
```toml
# Workspace Cargo.toml (root)
[workspace]
members = [
    "lib1",
    "lib2",
    "app",
]

[workspace.dependencies]
serde = "1.0"
```

### Member Library
```toml
# lib1/Cargo.toml
[package]
name = "lib1"

[dependencies]
serde = { workspace = true }
lib2 = { path = "../lib2" }
```

## Error Handling in Libraries

### Custom Error Types
```rust
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    NotFound,
    InvalidInput(String),
    IoError(std::io::Error),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Item not found"),
            MyError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            MyError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}

impl std::error::Error for MyError {}

// Usage
pub fn fallible_function() -> Result<String, MyError> {
    // ...
}
```

## Best Practices

1. **Clear Public API**
   - Export only what users need
   - Use `pub use` for convenience
   - Keep internal details private

2. **Documentation**
   - Document all public items
   - Include examples in docs
   - Write integration tests as examples

3. **Semantic Versioning**
   - MAJOR: Breaking changes
   - MINOR: New features (backward compatible)
   - PATCH: Bug fixes

4. **Testing**
   - Write unit tests for each function
   - Include integration tests
   - Test edge cases and errors

5. **Error Handling**
   - Use `Result<T, E>` for fallible operations
   - Create custom error types
   - Provide helpful error messages

## Quick Command Reference

```bash
# Create library
cargo new my_lib --lib

# Build and test
cargo build
cargo test
cargo doc --open

# Check without building
cargo check

# Format and lint
cargo fmt
cargo clippy

# Publish
cargo publish --dry-run
cargo publish

# Versioning
cargo version

# Add dependency
cargo add serde
```

## Example Library Structure

```rust
// src/lib.rs
//! # My Awesome Library
//! 
//! This library provides utilities for...

pub mod utils;
pub mod core;

pub use core::MainStruct;
pub use utils::helper_function;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
```

## Common Pitfalls

| Issue | Problem | Solution |
|-------|---------|----------|
| Private items | Can't access from outside | Add `pub` keyword |
| Missing docs | No documentation | Add `///` or `//!` comments |
| No tests | Untested code | Add `#[test]` functions |
| Breaking changes | Version not updated | Follow semver properly |
| Large public API | Too many exports | Keep API minimal, use modules |

## Testing Documentation Examples

```bash
# Test code examples in documentation
cargo test --doc

# Test specific doc example
cargo test --doc module_name::function_name
```

## Quick Tips

- Start with `cargo new --lib` for new libraries
- Keep public API minimal and well-documented
- Write tests alongside your code
- Use modules to organize related functionality
- Re-export commonly used items at the root
- Follow Rust naming conventions (snake_case for functions, PascalCase for types)
- Include examples in documentation
- Test your documentation examples with `cargo test --doc`
- Use `cargo clippy` to catch common mistakes
- Format code with `cargo fmt` before committing
