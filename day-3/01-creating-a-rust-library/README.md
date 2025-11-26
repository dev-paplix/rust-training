# Creating a Rust Library

Creating libraries in Rust allows you to build reusable code that can be shared across projects or published for the wider Rust community. This comprehensive guide covers everything you need to know about creating, organizing, documenting, testing, and publishing Rust libraries.

## Table of Contents

1. [Introduction to Rust Libraries](#introduction-to-rust-libraries)
2. [Creating Your First Library](#creating-your-first-library)
3. [Library Structure and Organization](#library-structure-and-organization)
4. [Modules and Visibility](#modules-and-visibility)
5. [The Public API](#the-public-api)
6. [Documentation](#documentation)
7. [Testing Libraries](#testing-libraries)
8. [Error Handling in Libraries](#error-handling-in-libraries)
9. [Building and Packaging](#building-and-packaging)
10. [Publishing to crates.io](#publishing-to-cratesio)
11. [Best Practices](#best-practices)

---

## Introduction to Rust Libraries

### What is a Library?

A library in Rust is a collection of reusable code that can be:
- Used by other projects
- Published to crates.io
- Shared within an organization
- Distributed as part of a workspace

### Libraries vs Binaries

| Aspect | Library | Binary |
|--------|---------|--------|
| Entry Point | `src/lib.rs` | `src/main.rs` |
| Purpose | Reusable code | Executable program |
| Output | `.rlib` file | Executable file |
| Can have `main()`? | No | Yes |
| Published to crates.io | Common | Less common |

### Why Create Libraries?

1. **Code Reuse**: Share functionality across multiple projects
2. **Modularity**: Separate concerns and organize code
3. **Community**: Contribute to the Rust ecosystem
4. **Maintainability**: Easier to test and update
5. **Distribution**: Easy to share and version

---

## Creating Your First Library

### Using Cargo

```bash
# Create a new library
cargo new my_library --lib

# Navigate to the library
cd my_library

# View the structure
ls -la
```

### Generated Structure

```
my_library/
├── Cargo.toml       # Package manifest
├── src/
│   └── lib.rs      # Library root module
└── .gitignore      # Git ignore file
```

### The Default lib.rs

```rust
// src/lib.rs
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
```

### Understanding lib.rs

- **Entry Point**: `lib.rs` is the root of your library
- **Public Items**: Use `pub` to make items accessible to users
- **Tests**: Can include unit tests with `#[cfg(test)]`
- **Documentation**: Can include module-level documentation

---

## Library Structure and Organization

### Simple Library (Single File)

```rust
// src/lib.rs
//! # Simple Math Library
//! 
//! Provides basic mathematical operations.

/// Adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Subtracts two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

/// Multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers
/// 
/// # Panics
/// 
/// Panics if divisor is zero
pub fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5, 3), 2);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 3), 12);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }
}
```

### Multi-Module Library

**Directory Structure:**
```
my_library/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── math.rs
    ├── string_utils.rs
    └── data/
        ├── mod.rs
        └── parser.rs
```

**src/lib.rs:**
```rust
//! # My Library
//! 
//! A comprehensive library providing various utilities.

// Declare modules
pub mod math;
pub mod string_utils;
pub mod data;

// Re-export commonly used items
pub use math::Calculator;
pub use string_utils::capitalize;

/// Library version constant
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

**src/math.rs:**
```rust
//! Math utilities module

/// A simple calculator
pub struct Calculator {
    value: f64,
}

impl Calculator {
    /// Creates a new calculator with initial value
    pub fn new(value: f64) -> Self {
        Calculator { value }
    }

    /// Adds to the current value
    pub fn add(&mut self, n: f64) -> &mut Self {
        self.value += n;
        self
    }

    /// Subtracts from the current value
    pub fn subtract(&mut self, n: f64) -> &mut Self {
        self.value -= n;
        self
    }

    /// Gets the current value
    pub fn result(&self) -> f64 {
        self.value
    }
}

/// Calculates factorial
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new(10.0);
        calc.add(5.0).subtract(3.0);
        assert_eq!(calc.result(), 12.0);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
    }
}
```

**src/string_utils.rs:**
```rust
//! String manipulation utilities

/// Capitalizes the first letter of a string
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Reverses a string
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Counts vowels in a string
pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_lowercase().next(), Some('a' | 'e' | 'i' | 'o' | 'u')))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
    }

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("aeiou"), 5);
    }
}
```

**src/data/mod.rs:**
```rust
//! Data processing module

pub mod parser;

pub use parser::Parser;

/// Common data types
pub type DataResult<T> = Result<T, DataError>;

#[derive(Debug)]
pub enum DataError {
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DataError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for DataError {}
```

**src/data/parser.rs:**
```rust
//! Parser implementation

use super::{DataResult, DataError};

pub struct Parser {
    data: String,
}

impl Parser {
    pub fn new(data: String) -> Self {
        Parser { data }
    }

    pub fn parse_int(&self) -> DataResult<i32> {
        self.data
            .trim()
            .parse::<i32>()
            .map_err(|e| DataError::ParseError(e.to_string()))
    }

    pub fn parse_float(&self) -> DataResult<f64> {
        self.data
            .trim()
            .parse::<f64>()
            .map_err(|e| DataError::ParseError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        let parser = Parser::new("42".to_string());
        assert_eq!(parser.parse_int().unwrap(), 42);
    }

    #[test]
    fn test_parse_int_error() {
        let parser = Parser::new("not a number".to_string());
        assert!(parser.parse_int().is_err());
    }
}
```

---

## Modules and Visibility

### Module Declaration

```rust
// Inline module
mod inline_module {
    pub fn function() { }
}

// External module (src/external.rs must exist)
mod external;

// Public module
pub mod public_module;
```

### Visibility Modifiers

```rust
// Public - accessible from anywhere
pub fn public_function() { }

// Private - only within current module (default)
fn private_function() { }

// Public within crate only
pub(crate) fn crate_visible() { }

// Public within parent module
pub(super) fn parent_visible() { }

// Public within specific path
pub(in crate::my_module) fn path_visible() { }
```

### Visibility Examples

```rust
// src/lib.rs
mod internal {
    // Private to the library
    fn helper() { }
    
    // Public within crate
    pub(crate) fn crate_helper() { }
    
    // Public API
    pub fn public_api() {
        helper(); // OK - same module
    }
}

pub mod public {
    use super::internal;
    
    pub fn use_crate_helper() {
        internal::crate_helper(); // OK - same crate
        // internal::helper(); // ERROR - private
    }
}
```

### Struct and Enum Visibility

```rust
// Public struct with private fields
pub struct Person {
    pub name: String,      // Public field
    age: u32,              // Private field
}

impl Person {
    // Constructor for private fields
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
    
    // Getter for private field
    pub fn age(&self) -> u32 {
        self.age
    }
    
    // Setter with validation
    pub fn set_age(&mut self, age: u32) -> Result<(), String> {
        if age > 150 {
            Err("Age too high".to_string())
        } else {
            self.age = age;
            Ok(())
        }
    }
}

// Public enum with variants
pub enum Status {
    Active,
    Inactive,
    Pending,
}
```

---

## The Public API

### Designing a Clean API

**Good API Design:**
```rust
// Clear, minimal public surface
pub struct Database {
    // Private internals
    connection: Connection,
}

impl Database {
    // Simple constructor
    pub fn new(url: &str) -> Result<Self, DbError> {
        // ...
    }
    
    // Clear method names
    pub fn connect(&mut self) -> Result<(), DbError> {
        // ...
    }
    
    pub fn execute(&self, query: &str) -> Result<QueryResult, DbError> {
        // ...
    }
}

// Re-export important types
pub use crate::error::DbError;
pub use crate::types::QueryResult;
```

### Re-exporting

```rust
// src/lib.rs
mod internal {
    pub struct Important;
    pub struct AlsoImportant;
    pub struct InternalDetail;
}

// Re-export for convenience
pub use internal::{Important, AlsoImportant};
// InternalDetail stays hidden

// Users can now do:
// use my_library::Important;
// Instead of:
// use my_library::internal::Important;
```

### Prelude Pattern

```rust
// src/prelude.rs
//! Convenient re-exports for common use

pub use crate::math::Calculator;
pub use crate::string_utils::{capitalize, reverse};
pub use crate::data::{Parser, DataError};

// src/lib.rs
pub mod prelude;

// Users can import commonly used items:
// use my_library::prelude::*;
```

---

## Documentation

### Module Documentation

```rust
//! # My Library
//! 
//! This library provides utilities for common programming tasks.
//! 
//! # Examples
//! 
//! ```
//! use my_library::Calculator;
//! 
//! let mut calc = Calculator::new(10.0);
//! calc.add(5.0);
//! assert_eq!(calc.result(), 15.0);
//! ```
//! 
//! # Modules
//! 
//! - `math`: Mathematical operations
//! - `string_utils`: String manipulation
//! - `data`: Data processing

mod internal_module;

pub mod math;
```

### Function Documentation

```rust
/// Calculates the factorial of a number
///
/// # Arguments
///
/// * `n` - The number to calculate factorial for
///
/// # Returns
///
/// The factorial of `n` as a `u64`
///
/// # Examples
///
/// ```
/// use my_library::factorial;
///
/// assert_eq!(factorial(5), 120);
/// assert_eq!(factorial(0), 1);
/// ```
///
/// # Panics
///
/// This function does not panic.
///
/// # Performance
///
/// This implementation uses recursion and has O(n) time complexity.
pub fn factorial(n: u32) -> u64 {
    match n {
        0 | 1 => 1,
        _ => n as u64 * factorial(n - 1),
    }
}
```

### Struct Documentation

```rust
/// A simple calculator for basic arithmetic
///
/// # Examples
///
/// ```
/// use my_library::Calculator;
///
/// let mut calc = Calculator::new(10.0);
/// calc.add(5.0).subtract(3.0);
/// assert_eq!(calc.result(), 12.0);
/// ```
pub struct Calculator {
    /// The current value of the calculator
    value: f64,
}

impl Calculator {
    /// Creates a new calculator with an initial value
    ///
    /// # Arguments
    ///
    /// * `value` - The initial value for the calculator
    ///
    /// # Examples
    ///
    /// ```
    /// use my_library::Calculator;
    ///
    /// let calc = Calculator::new(0.0);
    /// assert_eq!(calc.result(), 0.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Calculator { value }
    }

    /// Adds a value to the calculator
    ///
    /// Returns a mutable reference to self for method chaining.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_library::Calculator;
    ///
    /// let mut calc = Calculator::new(10.0);
    /// calc.add(5.0);
    /// assert_eq!(calc.result(), 15.0);
    /// ```
    pub fn add(&mut self, n: f64) -> &mut Self {
        self.value += n;
        self
    }

    /// Returns the current value
    pub fn result(&self) -> f64 {
        self.value
    }
}
```

### Error Documentation

```rust
/// Errors that can occur in the library
///
/// # Examples
///
/// ```
/// use my_library::{parse_data, DataError};
///
/// match parse_data("invalid") {
///     Ok(data) => println!("Parsed: {:?}", data),
///     Err(DataError::ParseError(msg)) => eprintln!("Parse failed: {}", msg),
///     Err(e) => eprintln!("Other error: {}", e),
/// }
/// ```
#[derive(Debug)]
pub enum DataError {
    /// Error parsing data
    ParseError(String),
    
    /// Error validating data
    ValidationError(String),
    
    /// IO error occurred
    IoError(std::io::Error),
}
```

### Generating Documentation

```bash
# Generate documentation
cargo doc

# Generate and open in browser
cargo doc --open

# Generate with private items
cargo doc --document-private-items

# Test documentation examples
cargo test --doc
```

---

## Testing Libraries

### Unit Tests

```rust
// src/lib.rs or src/module.rs
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

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_add_overflow() {
        // Test edge cases
        assert_eq!(add(i32::MAX, 0), i32::MAX);
    }
}
```

### Integration Tests

**tests/integration_test.rs:**
```rust
// Integration tests are external to your library
use my_library::{Calculator, factorial};

#[test]
fn test_calculator_integration() {
    let mut calc = Calculator::new(0.0);
    calc.add(10.0).add(5.0).subtract(3.0);
    assert_eq!(calc.result(), 12.0);
}

#[test]
fn test_factorial_integration() {
    assert_eq!(factorial(5), 120);
    assert_eq!(factorial(0), 1);
}

#[test]
fn test_library_version() {
    assert!(!my_library::VERSION.is_empty());
}
```

### Test Organization

```
my_library/
├── src/
│   ├── lib.rs              # Unit tests here
│   ├── math.rs             # Unit tests here
│   └── string_utils.rs     # Unit tests here
└── tests/
    ├── integration_test.rs
    ├── advanced_tests.rs
    └── common/             # Shared test utilities
        └── mod.rs
```

**tests/common/mod.rs:**
```rust
// Shared test utilities
pub fn setup() -> TestContext {
    TestContext {
        // initialization
    }
}

pub struct TestContext {
    // shared test data
}
```

**tests/advanced_tests.rs:**
```rust
mod common;

use my_library::*;

#[test]
fn advanced_test() {
    let _context = common::setup();
    // Test using shared utilities
}
```

### Test Commands

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests in specific module
cargo test module_name::

# Run integration tests only
cargo test --test integration_test

# Run with output
cargo test -- --nocapture

# Run tests in release mode
cargo test --release

# Run doc tests
cargo test --doc
```

### Advanced Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // Test that should panic
    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero() {
        divide(10, 0);
    }

    // Ignore expensive tests
    #[test]
    #[ignore]
    fn expensive_test() {
        // Long-running test
    }

    // Test with Result
    #[test]
    fn test_with_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Math is broken"))
        }
    }
}
```

---

## Error Handling in Libraries

### Custom Error Types

```rust
use std::fmt;

/// Library-specific error type
#[derive(Debug)]
pub enum LibraryError {
    /// Invalid input provided
    InvalidInput(String),
    
    /// Resource not found
    NotFound,
    
    /// IO error occurred
    Io(std::io::Error),
    
    /// Parse error
    Parse(String),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LibraryError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            LibraryError::NotFound => write!(f, "Resource not found"),
            LibraryError::Io(e) => write!(f, "IO error: {}", e),
            LibraryError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for LibraryError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LibraryError::Io(e) => Some(e),
            _ => None,
        }
    }
}

// Conversion from io::Error
impl From<std::io::Error> for LibraryError {
    fn from(error: std::io::Error) -> Self {
        LibraryError::Io(error)
    }
}
```

### Result Type Alias

```rust
/// Convenient Result type for library operations
pub type Result<T> = std::result::Result<T, LibraryError>;

// Usage in library functions
pub fn read_config(path: &str) -> Result<Config> {
    // Implementation
}

pub fn process_data(data: &str) -> Result<ProcessedData> {
    // Implementation
}
```

### Error Handling Example

```rust
use std::fs::File;
use std::io::Read;

pub fn read_file_content(path: &str) -> Result<String> {
    if path.is_empty() {
        return Err(LibraryError::InvalidInput("Path cannot be empty".to_string()));
    }

    let mut file = File::open(path)?; // Automatic conversion from io::Error
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    
    if content.is_empty() {
        return Err(LibraryError::NotFound);
    }

    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_path() {
        let result = read_file_content("");
        assert!(matches!(result, Err(LibraryError::InvalidInput(_))));
    }
}
```

---

## Building and Packaging

### Cargo.toml Configuration

```toml
[package]
name = "my_awesome_library"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A comprehensive library for awesome things"
documentation = "https://docs.rs/my_awesome_library"
homepage = "https://github.com/yourusername/my_awesome_library"
repository = "https://github.com/yourusername/my_awesome_library"
license = "MIT OR Apache-2.0"
keywords = ["awesome", "library", "utilities"]
categories = ["development-tools"]
readme = "README.md"
exclude = [
    ".github/",
    "examples/large_data/",
]

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", optional = true }

[dev-dependencies]
criterion = "0.5"

[features]
default = []
async = ["tokio"]
full = ["async"]

[[example]]
name = "basic_usage"
path = "examples/basic.rs"

[lib]
name = "my_awesome_library"
path = "src/lib.rs"

[package.metadata.docs.rs]
all-features = true
```

### Build Commands

```bash
# Check compilation without building
cargo check

# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release

# Build with specific features
cargo build --features async

# Build all features
cargo build --all-features

# Build without default features
cargo build --no-default-features
```

### Feature Flags

```rust
// src/lib.rs
#[cfg(feature = "async")]
pub mod async_ops {
    use tokio::runtime::Runtime;
    
    pub fn async_function() {
        // Async implementation
    }
}

#[cfg(not(feature = "async"))]
pub mod sync_ops {
    pub fn sync_function() {
        // Sync implementation
    }
}
```

### Package and Publish Prep

```bash
# Check package contents
cargo package --list

# Create package (doesn't upload)
cargo package

# Dry-run publish
cargo publish --dry-run

# Check for common mistakes
cargo clippy

# Format code
cargo fmt

# Run all tests
cargo test

# Check documentation
cargo doc --open
```

---

## Publishing to crates.io

### Prerequisites

1. Create account at [crates.io](https://crates.io)
2. Get API token from account settings
3. Login via cargo:

```bash
cargo login <your-api-token>
```

### Publishing Checklist

- [ ] All tests pass (`cargo test`)
- [ ] Documentation is complete (`cargo doc`)
- [ ] README.md exists and is complete
- [ ] LICENSE file exists
- [ ] Version number is correct in Cargo.toml
- [ ] Package builds successfully (`cargo build --release`)
- [ ] Dry-run succeeds (`cargo publish --dry-run`)

### Publishing Steps

```bash
# 1. Verify everything works
cargo test
cargo build --release
cargo doc

# 2. Check package
cargo package --list

# 3. Dry run
cargo publish --dry-run

# 4. Publish!
cargo publish
```

### Versioning (Semantic Versioning)

```
Version Format: MAJOR.MINOR.PATCH

- MAJOR: Breaking changes (e.g., 1.0.0 -> 2.0.0)
- MINOR: New features, backward compatible (e.g., 1.0.0 -> 1.1.0)
- PATCH: Bug fixes, backward compatible (e.g., 1.0.0 -> 1.0.1)
```

**Examples:**
```toml
# Initial development
version = "0.1.0"

# Bug fix
version = "0.1.1"

# New feature (backward compatible)
version = "0.2.0"

# Breaking change - first stable release
version = "1.0.0"

# Breaking change after 1.0
version = "2.0.0"
```

### Updating Published Crates

```bash
# Update version in Cargo.toml
# Then publish new version
cargo publish
```

**Note**: You cannot:
- Delete published versions
- Overwrite published versions
- Unpublish after 72 hours

### Yanking Versions

```bash
# Yank a version (prevents new projects from using it)
cargo yank --vers 1.0.1

# Un-yank a version
cargo yank --vers 1.0.1 --undo
```

---

## Best Practices

### 1. API Design

**Good:**
```rust
// Clear, concise public API
pub struct Config {
    // Private fields
    host: String,
    port: u16,
}

impl Config {
    // Builder pattern
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ConfigBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
    
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }
    
    pub fn build(self) -> Result<Config, String> {
        Ok(Config {
            host: self.host.ok_or("Host required")?,
            port: self.port.unwrap_or(8080),
        })
    }
}
```

### 2. Documentation

```rust
//! Module-level documentation goes here
//! 
//! Use `//!` for module/crate level docs
//! Use `///` for item-level docs

/// Function documentation
/// 
/// # Examples
/// 
/// ```
/// use my_lib::function;
/// assert_eq!(function(5), 10);
/// ```
pub fn function(n: i32) -> i32 {
    n * 2
}
```

### 3. Error Handling

```rust
// Define library-specific errors
#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    NotFound,
    External(Box<dyn std::error::Error>),
}

// Implement Display and Error traits
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::NotFound => write!(f, "Not found"),
            Error::External(e) => write!(f, "External error: {}", e),
        }
    }
}

impl std::error::Error for Error {}

// Use Result type alias
pub type Result<T> = std::result::Result<T, Error>;
```

### 4. Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        assert_eq!(add(2, 2), 4);
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    #[should_panic]
    fn test_panic_condition() {
        divide(10, 0);
    }
}
```

### 5. Module Organization

```
src/
├── lib.rs              # Public API, re-exports
├── error.rs            # Error types
├── config.rs           # Configuration
├── core/               # Core functionality
│   ├── mod.rs
│   ├── processor.rs
│   └── validator.rs
└── utils/              # Utilities
    ├── mod.rs
    └── helpers.rs
```

### 6. Examples

Create examples in `examples/` directory:

**examples/basic_usage.rs:**
```rust
use my_library::Calculator;

fn main() {
    let mut calc = Calculator::new(0.0);
    calc.add(10.0).subtract(3.0);
    println!("Result: {}", calc.result());
}
```

Run with:
```bash
cargo run --example basic_usage
```

### 7. README Template

````markdown
# My Awesome Library

Brief description of what your library does.

## Installation

```toml
[dependencies]
my_awesome_library = "0.1.0"
```

## Quick Start

```rust
use my_awesome_library::Calculator;

fn main() {
    let mut calc = Calculator::new(10.0);
    calc.add(5.0);
    println!("Result: {}", calc.result());
}
```

## Features

- Feature 1
- Feature 2
- Feature 3

## Documentation

See [docs.rs](https://docs.rs/my_awesome_library) for full documentation.

## License

Licensed under MIT OR Apache-2.0
````

### 8. Continuous Integration

**.github/workflows/ci.yml:**
```yaml
name: CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo clippy
      - run: cargo fmt --check
```

### 9. Dependencies

```toml
[dependencies]
# Specify versions carefully
serde = "1.0"           # Good: allows minor updates
tokio = "=1.28.0"       # Strict: exact version
regex = "1"             # Flexible: any 1.x.x

[dev-dependencies]
# Dev dependencies for testing
criterion = "0.5"
proptest = "1.0"
```

### 10. Feature Organization

```toml
[features]
default = ["std"]
std = []
async = ["tokio", "futures"]
full = ["async", "serde"]
```

```rust
#[cfg(feature = "async")]
pub mod async_api {
    // Async implementation
}

#[cfg(not(feature = "async"))]
pub mod sync_api {
    // Sync implementation
}
```

---

## Common Library Patterns

### Builder Pattern

```rust
pub struct Database {
    host: String,
    port: u16,
    username: String,
    password: String,
}

pub struct DatabaseBuilder {
    host: Option<String>,
    port: Option<u16>,
    username: Option<String>,
    password: Option<String>,
}

impl Database {
    pub fn builder() -> DatabaseBuilder {
        DatabaseBuilder {
            host: None,
            port: None,
            username: None,
            password: None,
        }
    }
}

impl DatabaseBuilder {
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn username(mut self, username: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn build(self) -> Result<Database, String> {
        Ok(Database {
            host: self.host.ok_or("Host is required")?,
            port: self.port.unwrap_or(5432),
            username: self.username.ok_or("Username is required")?,
            password: self.password.ok_or("Password is required")?,
        })
    }
}

// Usage:
// let db = Database::builder()
//     .host("localhost")
//     .port(5432)
//     .username("admin")
//     .password("secret")
//     .build()?;
```

### Newtype Pattern

```rust
// Wrapper type for type safety
pub struct UserId(u64);

impl UserId {
    pub fn new(id: u64) -> Self {
        UserId(id)
    }

    pub fn value(&self) -> u64 {
        self.0
    }
}

pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        if email.contains('@') {
            Ok(Email(email))
        } else {
            Err("Invalid email".to_string())
        }
    }
}
```

### Facade Pattern

```rust
// Simplified interface to complex subsystem
mod internal {
    pub struct ComplexSystem1;
    pub struct ComplexSystem2;
    pub struct ComplexSystem3;
}

pub struct SimpleFacade {
    system1: internal::ComplexSystem1,
    system2: internal::ComplexSystem2,
    system3: internal::ComplexSystem3,
}

impl SimpleFacade {
    pub fn new() -> Self {
        SimpleFacade {
            system1: internal::ComplexSystem1,
            system2: internal::ComplexSystem2,
            system3: internal::ComplexSystem3,
        }
    }

    pub fn do_complex_operation(&self) -> Result<String, Error> {
        // Coordinates all subsystems
        Ok("Success".to_string())
    }
}
```

---

## Complete Example Library

Here's a complete, real-world example library:

**Cargo.toml:**
```toml
[package]
name = "text_analyzer"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A library for analyzing text"
license = "MIT OR Apache-2.0"
repository = "https://github.com/yourusername/text_analyzer"
keywords = ["text", "analysis", "nlp"]
categories = ["text-processing"]

[dependencies]
regex = "1.10"

[dev-dependencies]
criterion = "0.5"
```

**src/lib.rs:**
```rust
//! # Text Analyzer Library
//! 
//! A simple library for analyzing text content.
//! 
//! # Examples
//! 
//! ```
//! use text_analyzer::TextAnalyzer;
//! 
//! let text = "Hello world! This is a test.";
//! let analyzer = TextAnalyzer::new(text);
//! 
//! assert_eq!(analyzer.word_count(), 6);
//! assert_eq!(analyzer.sentence_count(), 2);
//! ```

pub mod error;
pub mod stats;

pub use error::{Error, Result};
pub use stats::TextStats;

use regex::Regex;

/// Main text analyzer struct
pub struct TextAnalyzer {
    text: String,
}

impl TextAnalyzer {
    /// Creates a new text analyzer
    pub fn new(text: impl Into<String>) -> Self {
        TextAnalyzer {
            text: text.into(),
        }
    }

    /// Counts words in the text
    pub fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    /// Counts sentences in the text
    pub fn sentence_count(&self) -> usize {
        let re = Regex::new(r"[.!?]+").unwrap();
        re.split(&self.text)
            .filter(|s| !s.trim().is_empty())
            .count()
    }

    /// Gets comprehensive statistics
    pub fn stats(&self) -> TextStats {
        TextStats {
            words: self.word_count(),
            sentences: self.sentence_count(),
            characters: self.text.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_count() {
        let analyzer = TextAnalyzer::new("Hello world");
        assert_eq!(analyzer.word_count(), 2);
    }

    #[test]
    fn test_sentence_count() {
        let analyzer = TextAnalyzer::new("Hello. World!");
        assert_eq!(analyzer.sentence_count(), 2);
    }
}
```

**src/error.rs:**
```rust
//! Error types for the library

use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    ProcessingError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
```

**src/stats.rs:**
```rust
//! Text statistics types

/// Statistics about analyzed text
#[derive(Debug, Clone, PartialEq)]
pub struct TextStats {
    pub words: usize,
    pub sentences: usize,
    pub characters: usize,
}

impl TextStats {
    /// Average word length
    pub fn avg_word_length(&self) -> f64 {
        if self.words == 0 {
            0.0
        } else {
            self.characters as f64 / self.words as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avg_word_length() {
        let stats = TextStats {
            words: 2,
            sentences: 1,
            characters: 10,
        };
        assert_eq!(stats.avg_word_length(), 5.0);
    }
}
```

**tests/integration_test.rs:**
```rust
use text_analyzer::TextAnalyzer;

#[test]
fn test_full_analysis() {
    let text = "Hello world! This is a test. Amazing!";
    let analyzer = TextAnalyzer::new(text);
    
    let stats = analyzer.stats();
    assert_eq!(stats.words, 7);
    assert_eq!(stats.sentences, 3);
}
```

**examples/basic.rs:**
```rust
use text_analyzer::TextAnalyzer;

fn main() {
    let text = "The quick brown fox jumps over the lazy dog. This is a test sentence!";
    
    let analyzer = TextAnalyzer::new(text);
    let stats = analyzer.stats();
    
    println!("Text Analysis:");
    println!("  Words: {}", stats.words);
    println!("  Sentences: {}", stats.sentences);
    println!("  Characters: {}", stats.characters);
    println!("  Avg word length: {:.2}", stats.avg_word_length());
}
```

---

## Summary

Creating Rust libraries involves:

1. **Structure**: Organize code with clear modules and visibility
2. **API Design**: Create intuitive, minimal public interfaces
3. **Documentation**: Document all public items with examples
4. **Testing**: Write comprehensive unit and integration tests
5. **Error Handling**: Use custom error types and Result
6. **Publishing**: Follow semantic versioning and publish to crates.io

**Key Takeaways:**

- Use `cargo new --lib` to create libraries
- Keep public API minimal and well-documented
- Write tests alongside your code
- Use feature flags for optional functionality
- Follow semantic versioning
- Provide examples and good documentation
- Test documentation examples
- Use CI/CD for quality assurance

Libraries are the foundation of the Rust ecosystem - create reusable, well-tested, and well-documented code that others can depend on!
