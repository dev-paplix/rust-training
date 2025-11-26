# Feature Flags Demo

A demonstration library for using feature flags and conditional compilation when publishing Rust crates to crates.io.

## Overview

This crate shows how to structure a library with optional features, allowing users to enable only the functionality they need. This approach:

- Reduces compile times
- Minimizes dependencies
- Provides flexibility for different use cases
- Supports no_std environments

## Installation

### Basic Installation (minimal dependencies)

```toml
[dependencies]
feature_flags_demo = "0.1.0"
```

### With Optional Features

```toml
[dependencies]
# Enable serde support
feature_flags_demo = { version = "0.1.0", features = ["serde_support"] }

# Enable multiple features
feature_flags_demo = { version = "0.1.0", features = ["serde_support", "async_support"] }

# Enable all features
feature_flags_demo = { version = "0.1.0", features = ["full"] }
```

## Features

### Default Features

- `std` - Standard library support (enabled by default)

### Optional Features

- `serde_support` - Serialization/deserialization with serde and serde_json
- `async_support` - Async runtime support with tokio
- `crypto` - Cryptographic functions (SHA-256 hashing, base64 encoding)
- `full` - Enable all optional features

## Usage

### Basic Usage (No Optional Features)

```rust
use feature_flags_demo::Data;

let data = Data::new("Hello, World!");
assert_eq!(data.content(), "Hello, World!");

// Check which features are enabled
println!("{}", feature_flags_demo::check_features());
```

### With Standard Library Features

```rust
use feature_flags_demo::Data;

let mut data = Data::new("content");
data.set_timestamp("2025-01-01");
assert_eq!(data.timestamp(), Some("2025-01-01"));
```

### With Serde Support

Enable in `Cargo.toml`:
```toml
[dependencies]
feature_flags_demo = { version = "0.1.0", features = ["serde_support"] }
```

```rust
use feature_flags_demo::{Data, serde_utils};

let data = Data::new("test data");

// Serialize to JSON
let json = serde_utils::to_json(&data).unwrap();
println!("{}", json);

// Deserialize from JSON
let parsed: Data = serde_utils::from_json(&json).unwrap();
assert_eq!(parsed.content(), "test data");
```

### With Async Support

Enable in `Cargo.toml`:
```toml
[dependencies]
feature_flags_demo = { version = "0.1.0", features = ["async_support"] }
tokio = { version = "1.0", features = ["rt", "macros"] }
```

```rust
use feature_flags_demo::async_utils;

#[tokio::main]
async fn main() {
    let result = async_utils::process_async("my data").await;
    println!("{}", result);

    let fetched = async_utils::fetch_data().await;
    println!("{}", fetched);
}
```

### With Crypto Support

Enable in `Cargo.toml`:
```toml
[dependencies]
feature_flags_demo = { version = "0.1.0", features = ["crypto"] }
```

```rust
use feature_flags_demo::crypto_utils;

// Compute SHA-256 hash
let hash = crypto_utils::hash("hello world");
println!("Hash: {}", hash);

// Base64 encoding
let encoded = crypto_utils::encode_base64("hello");
assert_eq!(encoded, "aGVsbG8=");

// Base64 decoding
let decoded = crypto_utils::decode_base64("aGVsbG8=").unwrap();
assert_eq!(decoded, "hello");
```

### All Features Combined

Enable in `Cargo.toml`:
```toml
[dependencies]
feature_flags_demo = { version = "0.1.0", features = ["full"] }
tokio = { version = "1.0", features = ["rt", "macros"] }
```

```rust
use feature_flags_demo::{Data, serde_utils, async_utils, crypto_utils};

#[tokio::main]
async fn main() {
    // Create and serialize data
    let data = Data::new("sensitive data");
    let json = serde_utils::to_json(&data).unwrap();
    
    // Hash the JSON
    let hash = crypto_utils::hash(&json);
    
    // Process asynchronously
    let result = async_utils::process_async(&hash).await;
    println!("{}", result);
}
```

## Feature Testing

Test with different feature combinations:

```bash
# Test without default features (no_std compatible)
cargo test --no-default-features

# Test with default features only
cargo test

# Test with specific features
cargo test --features serde_support
cargo test --features async_support
cargo test --features crypto

# Test with all features
cargo test --all-features
```

## Feature Flag Best Practices Demonstrated

### 1. Default Features

```toml
[features]
default = ["std"]  # Most users want std
```

### 2. Optional Dependencies

```toml
[dependencies]
serde = { version = "1.0", optional = true }
```

### 3. Feature-Gated Modules

```rust
#[cfg(feature = "serde_support")]
pub mod serde_utils {
    // Module only available with feature
}
```

### 4. Conditional Compilation

```rust
#[cfg_attr(feature = "serde_support", derive(serde::Serialize))]
pub struct Data {
    // ...
}
```

### 5. Feature-Gated Methods

```rust
#[cfg(feature = "std")]
pub fn set_timestamp(&mut self, timestamp: String) {
    // Method only available with std feature
}
```

### 6. Feature Combinations

```toml
[features]
full = ["std", "serde_support", "async_support", "crypto"]
```

## Publishing Considerations

### Testing Before Publishing

```bash
# Test all feature combinations
cargo test --no-default-features
cargo test
cargo test --all-features

# Test specific combinations
cargo test --features "serde_support async_support"

# Verify package contents
cargo package --list

# Dry run
cargo publish --dry-run
```

### Documentation

The crate includes:
- Feature documentation in crate-level docs
- Conditional doc tests (only run when features enabled)
- Clear examples for each feature
- Feature flags in README

### Version Constraints

When using path dependencies during development:

```toml
[dependencies]
# For development
my_crate = { path = "../my_crate", features = ["full"] }

# For publishing (path removed)
my_crate = { version = "0.1.0", features = ["full"] }
```

## API Reference

### Always Available

- `Data` - Core data structure
- `Data::new(content)` - Create new data
- `Data::content()` - Get content
- `always_available()` - Always returns true
- `check_features()` - Shows enabled features

### With `std` Feature

- `Data::set_timestamp(timestamp)` - Set timestamp
- `Data::timestamp()` - Get timestamp

### With `serde_support` Feature

- `serde_utils::to_json(&value)` - Serialize to JSON
- `serde_utils::from_json(json)` - Deserialize from JSON

### With `async_support` Feature

- `async_utils::process_async(data)` - Async processing
- `async_utils::fetch_data()` - Async data fetch

### With `crypto` Feature

- `crypto_utils::hash(data)` - SHA-256 hash
- `crypto_utils::encode_base64(data)` - Base64 encode
- `crypto_utils::decode_base64(data)` - Base64 decode

## Documentation

For full API documentation with all features, visit [docs.rs/feature_flags_demo](https://docs.rs/feature_flags_demo) (once published).

The documentation on docs.rs is built with all features enabled, so all APIs are documented.

## Examples

See the `examples/` directory for complete examples:

- `basic.rs` - Basic usage without features
- `with_serde.rs` - Using serde support
- `with_async.rs` - Using async support
- `with_crypto.rs` - Using crypto features
- `full_features.rs` - Using all features

## Design Rationale

### Why Feature Flags?

1. **Minimal Dependencies**: Users only pull in what they need
2. **Compile Time**: Faster builds with fewer features
3. **no_std Support**: Core functionality works without std
4. **Flexibility**: Different use cases need different features

### Feature Organization

- Core functionality always available
- Heavy dependencies (serde, tokio) optional
- Related features grouped logically
- "full" feature for convenience

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

This is a demonstration crate for educational purposes.

## Learning Resources

- [Cargo Book - Features](https://doc.rust-lang.org/cargo/reference/features.html)
- [Conditional Compilation](https://doc.rust-lang.org/reference/conditional-compilation.html)
- [Feature Flags Best Practices](https://rust-lang.github.io/api-guidelines/flexibility.html)
