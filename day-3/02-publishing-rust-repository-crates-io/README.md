# Publishing to the Rust Repository (crates.io)

## Table of Contents

1. [Introduction](#introduction)
2. [Understanding crates.io](#understanding-cratesio)
3. [Prerequisites and Setup](#prerequisites-and-setup)
4. [Preparing Your Crate for Publishing](#preparing-your-crate-for-publishing)
5. [Cargo.toml Metadata](#cargotoml-metadata)
6. [Documentation Requirements](#documentation-requirements)
7. [Licensing Your Crate](#licensing-your-crate)
8. [Testing Before Publishing](#testing-before-publishing)
9. [The Publishing Process](#the-publishing-process)
10. [Version Management](#version-management)
11. [Feature Flags and Optional Dependencies](#feature-flags-and-optional-dependencies)
12. [Publishing Workspaces](#publishing-workspaces)
13. [Publishing Binary Crates](#publishing-binary-crates)
14. [Post-Publishing Maintenance](#post-publishing-maintenance)
15. [Best Practices](#best-practices)
16. [Common Issues and Solutions](#common-issues-and-solutions)

---

## Introduction

Publishing your Rust crate to [crates.io](https://crates.io) makes it available to the entire Rust ecosystem. This guide covers everything you need to know about preparing, publishing, and maintaining crates on the official Rust package registry.

### What You'll Learn

- Setting up your crates.io account and API token
- Preparing your crate with proper metadata
- Writing effective documentation
- Choosing and applying licenses
- The complete publishing workflow
- Managing versions with semantic versioning
- Handling feature flags and optional dependencies
- Publishing workspace crates
- Publishing binary crates (CLI tools)
- Maintaining published crates

### Why Publish to crates.io?

1. **Discoverability**: Your crate becomes searchable and accessible to all Rust developers
2. **Documentation**: Automatic documentation generation on docs.rs
3. **Version Management**: Cargo handles dependency resolution automatically
4. **Community**: Join the Rust ecosystem and collaborate with others
5. **Reliability**: Immutable versions ensure reproducible builds

---

## Understanding crates.io

### What is crates.io?

crates.io is the official package registry for Rust. It hosts:
- **Library crates**: Reusable code that other projects can depend on
- **Binary crates**: Installable CLI tools and applications
- **Proc-macro crates**: Procedural macros for compile-time code generation

### Key Features

1. **Immutability**: Once published, a version cannot be changed (only yanked)
2. **Automatic Documentation**: docs.rs builds and hosts your documentation
3. **Dependency Resolution**: Cargo automatically manages versions
4. **Search and Discovery**: Users can find crates by keywords and categories
5. **API Tokens**: Secure authentication for publishing

### crates.io vs docs.rs

- **crates.io**: Hosts the actual crate files (.crate archives)
- **docs.rs**: Builds and hosts documentation for all published crates

---

## Prerequisites and Setup

### Creating a crates.io Account

1. **Visit crates.io**
   ```
   https://crates.io
   ```

2. **Sign in with GitHub**
   - Click "Log in with GitHub"
   - Authorize crates.io to access your GitHub account
   - Your GitHub username becomes your crates.io identity

3. **Generate an API Token**
   - Go to Account Settings
   - Navigate to "API Tokens"
   - Click "New Token"
   - Give it a descriptive name (e.g., "laptop-dev-token")
   - Save the token securely (you won't see it again)

### Logging in with Cargo

```bash
# Login to crates.io with your API token
cargo login <your-api-token>
```

This stores your token in `~/.cargo/credentials.toml`:

```toml
[registry]
token = "your-api-token-here"
```

**Security Note**: Keep this token private. Anyone with this token can publish crates under your name.

### Revoking Tokens

If a token is compromised:
1. Go to crates.io Account Settings
2. Find the token in "API Tokens"
3. Click "Revoke"
4. Generate a new token
5. Re-run `cargo login` with the new token

---

## Preparing Your Crate for Publishing

### Pre-Publishing Checklist

Before publishing, ensure you have:

- [ ] Unique crate name (check on crates.io)
- [ ] Complete `Cargo.toml` with all required metadata
- [ ] LICENSE file with appropriate license
- [ ] README.md with installation and usage instructions
- [ ] Comprehensive documentation (//! and /// comments)
- [ ] All tests passing
- [ ] Code formatted with `cargo fmt`
- [ ] No warnings from `cargo clippy`
- [ ] Documentation builds successfully
- [ ] `.gitignore` excludes unnecessary files

### Choosing a Crate Name

```bash
# Check if name is available
# Visit: https://crates.io/crates/your_crate_name
# If it 404s, the name is available
```

**Naming Guidelines**:
- Use lowercase letters, numbers, and underscores
- Be descriptive but concise
- Avoid names too similar to existing crates
- Consider using a prefix for related crates (e.g., `serde_json`, `tokio_util`)

### Directory Structure

A typical publishable crate:

```
my_crate/
├── Cargo.toml           # Package metadata
├── LICENSE              # License file
├── LICENSE-APACHE       # Optional: Apache license
├── LICENSE-MIT          # Optional: MIT license
├── README.md            # User-facing documentation
├── CHANGELOG.md         # Version history (recommended)
├── src/
│   ├── lib.rs           # Library entry point
│   ├── module1.rs
│   └── module2.rs
├── tests/               # Integration tests
│   └── integration_test.rs
├── examples/            # Example code
│   └── basic_usage.rs
└── benches/             # Benchmarks
    └── benchmark.rs
```

---

## Cargo.toml Metadata

### Required Fields

```toml
[package]
name = "my_crate"              # Unique crate name
version = "0.1.0"              # Semantic version
edition = "2021"               # Rust edition
description = "A brief description of what this crate does"  # Max ~100 chars
license = "MIT OR Apache-2.0"  # SPDX license identifier
```

### Recommended Fields

```toml
[package]
authors = ["Your Name <you@example.com>"]
repository = "https://github.com/username/my_crate"
documentation = "https://docs.rs/my_crate"  # Auto-generated if omitted
homepage = "https://my-crate.io"            # Optional: project website
readme = "README.md"                         # Path to README
keywords = ["keyword1", "keyword2"]          # Max 5, lowercase
categories = ["development-tools"]           # From crates.io category list
```

### Full Example

```toml
[package]
name = "awesome_parser"
version = "0.1.0"
edition = "2021"
authors = ["Jane Developer <jane@example.com>"]
description = "A fast and ergonomic parser for custom file formats"
license = "MIT OR Apache-2.0"
repository = "https://github.com/jane/awesome_parser"
documentation = "https://docs.rs/awesome_parser"
homepage = "https://awesome-parser.dev"
readme = "README.md"
keywords = ["parser", "parsing", "format", "text"]
categories = ["parsing", "text-processing"]
exclude = [
    ".github/",
    "tests/fixtures/large_files/",
    "*.swp",
]

[dependencies]
# Your dependencies

[dev-dependencies]
# Development dependencies

[features]
default = ["std"]
std = []
```

### Keywords

Keywords help users discover your crate:

```toml
keywords = ["json", "parser", "serialization"]
```

**Rules**:
- Maximum 5 keywords
- Lowercase only
- Use hyphens for multi-word keywords (e.g., "command-line")
- Be specific and relevant

**Good Keywords**: `json`, `cli`, `async`, `web`, `database`
**Bad Keywords**: `rust`, `library`, `tool` (too generic)

### Categories

Categories provide structured classification:

```toml
categories = ["command-line-utilities", "parsing"]
```

**Official Categories** (select from):
- `algorithms`
- `api-bindings`
- `asynchronous`
- `authentication`
- `caching`
- `command-line-utilities`
- `config`
- `cryptography`
- `data-structures`
- `database`
- `date-and-time`
- `development-tools`
- `encoding`
- `filesystem`
- `game-development`
- `network-programming`
- `parser-implementations`
- `rust-patterns`
- `web-programming`

View all categories at: https://crates.io/categories

### Excluding Files

```toml
# Blacklist approach
exclude = [
    ".github/",
    "*.swp",
    ".DS_Store",
    "tests/fixtures/large_data/",
]

# Whitelist approach (alternative)
include = [
    "src/**/*",
    "Cargo.toml",
    "Cargo.lock",  # For binary crates
    "README.md",
    "LICENSE",
    "LICENSE-MIT",
    "LICENSE-APACHE",
]
```

**What to Exclude**:
- CI/CD configuration (`.github/`, `.gitlab-ci.yml`)
- Editor files (`.vscode/`, `*.swp`)
- OS files (`.DS_Store`, `Thumbs.db`)
- Large test fixtures
- Documentation source files

---

## Documentation Requirements

### Crate-Level Documentation

In `src/lib.rs`, add top-level documentation:

```rust
//! # My Crate
//!
//! `my_crate` provides utilities for doing awesome things.
//!
//! # Quick Start
//!
//! ```
//! use my_crate::AwesomeThing;
//!
//! let thing = AwesomeThing::new();
//! assert_eq!(thing.do_something(), 42);
//! ```
//!
//! # Features
//!
//! - Fast and efficient
//! - Easy to use API
//! - Well tested
//!
//! # Optional Features
//!
//! - `serde`: Enable serialization support
//! - `async`: Enable async runtime support

/// Public function that does something useful
pub fn do_something() -> i32 {
    42
}
```

### Function Documentation

```rust
/// Calculates the sum of two numbers.
///
/// # Examples
///
/// ```
/// use my_crate::add;
///
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// ```
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The sum of `a` and `b`
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### Error Documentation

```rust
/// Parses a string into an integer.
///
/// # Examples
///
/// ```
/// use my_crate::parse_number;
///
/// assert_eq!(parse_number("42").unwrap(), 42);
/// assert!(parse_number("invalid").is_err());
/// ```
///
/// # Errors
///
/// Returns `ParseError` if the string cannot be parsed as an integer.
pub fn parse_number(s: &str) -> Result<i32, ParseError> {
    s.parse().map_err(|_| ParseError::InvalidFormat)
}
```

### Struct Documentation

```rust
/// A configuration builder for creating `Config` instances.
///
/// # Examples
///
/// ```
/// use my_crate::ConfigBuilder;
///
/// let config = ConfigBuilder::new()
///     .host("localhost")
///     .port(8080)
///     .build()
///     .unwrap();
/// ```
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
}

impl ConfigBuilder {
    /// Creates a new `ConfigBuilder` with default values.
    pub fn new() -> Self {
        Self {
            host: None,
            port: None,
        }
    }

    /// Sets the host address.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::ConfigBuilder;
    ///
    /// let builder = ConfigBuilder::new().host("127.0.0.1");
    /// ```
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }
}
```

### Testing Documentation

```bash
# Test all documentation examples
cargo test --doc

# Build documentation and open in browser
cargo doc --open

# Build without dependencies
cargo doc --no-deps
```

### README.md

Create a comprehensive README:

```markdown
# My Crate

Brief description of what your crate does.

## Installation

Add this to your `Cargo.toml`:

\`\`\`toml
[dependencies]
my_crate = "0.1.0"
\`\`\`

## Quick Start

\`\`\`rust
use my_crate::Something;

fn main() {
    let thing = Something::new();
    thing.do_work();
}
\`\`\`

## Features

- Feature 1: Description
- Feature 2: Description

## Optional Features

Enable features in `Cargo.toml`:

\`\`\`toml
[dependencies]
my_crate = { version = "0.1.0", features = ["serde"] }
\`\`\`

Available features:
- `serde`: Serialization support
- `async`: Async runtime support

## Documentation

For full documentation, see [docs.rs/my_crate](https://docs.rs/my_crate).

## Examples

See the [examples](examples/) directory for more examples.

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
```

---

## Licensing Your Crate

### Choosing a License

Most Rust crates use dual licensing:

```toml
license = "MIT OR Apache-2.0"
```

**Why Dual License?**
- MIT: Simple, permissive, widely compatible
- Apache-2.0: Includes patent protection, more comprehensive
- Dual licensing gives users choice

### Other License Options

```toml
# Single license
license = "MIT"
license = "Apache-2.0"
license = "GPL-3.0"
license = "BSD-3-Clause"

# Custom license file
license-file = "LICENSE.txt"
```

### Creating LICENSE Files

**MIT License** (`LICENSE-MIT`):

```
MIT License

Copyright (c) 2025 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

**Apache License 2.0** (`LICENSE-APACHE`):

Get the full text from: https://www.apache.org/licenses/LICENSE-2.0.txt

### Single LICENSE File

For dual licensing, you can use a single `LICENSE` file:

```
Licensed under either of:
- Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
```

---

## Testing Before Publishing

### Comprehensive Testing

```bash
# Run all tests
cargo test

# Run with all features
cargo test --all-features

# Run without default features
cargo test --no-default-features

# Test specific feature
cargo test --features serde

# Test documentation
cargo test --doc

# Run integration tests
cargo test --test '*'

# Run with release optimizations
cargo test --release
```

### Code Quality Checks

```bash
# Check for errors
cargo check

# Check with all features
cargo check --all-features

# Run clippy (linter)
cargo clippy

# Clippy with warnings as errors
cargo clippy -- -D warnings

# Format code
cargo fmt

# Check formatting without modifying
cargo fmt -- --check
```

### Building Documentation

```bash
# Build documentation
cargo doc

# Build and open in browser
cargo doc --open

# Build without dependencies
cargo doc --no-deps

# Check for documentation warnings
cargo rustdoc -- -D warnings
```

### Package Verification

```bash
# Create a package archive
cargo package

# List files that will be included
cargo package --list

# Verify package can be built
cargo package --verify

# Allow dirty working directory
cargo package --allow-dirty
```

The `cargo package` command creates a `.crate` file in `target/package/`.

---

## The Publishing Process

### Step-by-Step Publishing

#### 1. Prepare Your Crate

```bash
# Ensure everything is committed
git status

# Update version if needed
# Edit Cargo.toml: version = "0.1.0"

# Update CHANGELOG.md
# Document changes in this version
```

#### 2. Run Pre-Publish Checks

```bash
# Full test suite
cargo test --all-features

# Linting
cargo clippy -- -D warnings

# Formatting
cargo fmt --check

# Documentation
cargo doc --no-deps
```

#### 3. Package the Crate

```bash
# Create package
cargo package

# Verify contents
cargo package --list

# Ensure it builds
cargo package --verify
```

#### 4. Dry Run

```bash
# Test publishing without actually publishing
cargo publish --dry-run
```

This performs all checks without uploading to crates.io.

#### 5. Publish

```bash
# Publish to crates.io
cargo publish
```

**What Happens**:
1. Package is uploaded to crates.io
2. crates.io validates the package
3. Version becomes available for download
4. docs.rs begins building documentation

#### 6. Verify Publication

```bash
# Check on crates.io
# Visit: https://crates.io/crates/your_crate

# Check documentation
# Visit: https://docs.rs/your_crate

# Test installation
cargo install your_crate  # For binary crates
```

### Post-Publish Git Workflow

```bash
# Tag the release
git tag -a v0.1.0 -m "Release version 0.1.0"

# Push tag to remote
git push origin v0.1.0

# Create GitHub release (optional)
# Go to GitHub → Releases → Create new release from tag
```

### Publish to Test Registry (Alternative)

For testing, you can use an alternative registry:

```bash
# In Cargo.toml
[registries]
test-registry = { index = "https://test-registry.example.com" }

# Publish to test registry
cargo publish --registry test-registry
```

---

## Version Management

### Semantic Versioning (SemVer)

Format: `MAJOR.MINOR.PATCH`

```
1.2.3
│ │ │
│ │ └─ PATCH: Bug fixes (backward compatible)
│ └─── MINOR: New features (backward compatible)
└───── MAJOR: Breaking changes
```

### Version Ranges

Before 1.0.0:
- `0.1.0` → `0.2.0`: Can include breaking changes
- `0.1.0` → `0.1.1`: Bug fixes only

After 1.0.0:
- `1.0.0` → `2.0.0`: Breaking changes
- `1.0.0` → `1.1.0`: New features, backward compatible
- `1.0.0` → `1.0.1`: Bug fixes only

### Version Bumping Strategy

```toml
# Development phase
0.1.0  # Initial release
0.1.1  # Bug fixes
0.2.0  # New features or breaking changes
0.3.0  # More features

# First stable release
1.0.0  # Stable API commitment

# Maintenance
1.0.1  # Bug fix
1.1.0  # New feature (compatible)
2.0.0  # Breaking change
```

### Updating Versions

```bash
# 1. Edit Cargo.toml
# version = "0.2.0"

# 2. Update CHANGELOG.md
# ## [0.2.0] - 2025-01-15
# ### Added
# - New feature X
# ### Changed
# - Improved performance of Y

# 3. Commit changes
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to 0.2.0"

# 4. Publish
cargo publish

# 5. Tag release
git tag v0.2.0
git push origin v0.2.0
```

### Yanking Versions

If you publish a broken version:

```bash
# Yank the version
cargo yank --vers 0.1.1

# This prevents NEW projects from using it
# Existing Cargo.lock files still work
```

**When to Yank**:
- Security vulnerability discovered
- Critical bug that breaks functionality
- Accidental publication of broken code

**When NOT to Yank**:
- Minor bugs (publish a patch instead)
- Just to remove old versions
- To force upgrades

**Un-yanking**:
```bash
# If yanked by mistake
cargo yank --vers 0.1.1 --undo
```

### Version Constraints in Dependencies

```toml
[dependencies]
# Exact version
my_crate = "=0.1.0"

# Compatible versions (recommended)
my_crate = "0.1.0"     # >=0.1.0, <0.2.0
my_crate = "1.2.3"     # >=1.2.3, <2.0.0

# Minimum version
my_crate = ">=0.1.0"

# Range
my_crate = ">=0.1.0, <0.3.0"

# Wildcard
my_crate = "0.1.*"     # >=0.1.0, <0.2.0
```

---

## Feature Flags and Optional Dependencies

### Defining Features

```toml
[package]
name = "my_crate"
version = "0.1.0"
edition = "2021"

[features]
default = ["std"]           # Default features
std = []                    # Standard library support
serde = ["dep:serde"]      # Enable serde dependency
async = ["tokio"]          # Enable async with tokio
full = ["std", "serde", "async"]  # All features

[dependencies]
# Optional dependencies (not included by default)
serde = { version = "1.0", features = ["derive"], optional = true }
tokio = { version = "1.0", features = ["rt", "macros"], optional = true }

# Always included
thiserror = "1.0"
```

### Using Feature Flags in Code

```rust
// Conditional compilation
#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MyStruct {
    pub value: i32,
}

// Feature-gated modules
#[cfg(feature = "async")]
pub mod async_api {
    pub async fn do_async_work() {
        // Async implementation
    }
}

// Feature-gated functions
#[cfg(feature = "serde")]
pub fn serialize<T: serde::Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}
```

### Feature-Gated Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn basic_test() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_test() {
        // Only runs when serde feature is enabled
    }
}
```

### Publishing with Features

```bash
# Test all feature combinations
cargo test --no-default-features
cargo test --all-features
cargo test --features serde
cargo test --features "serde async"

# Publish (includes all feature definitions)
cargo publish

# Users enable features:
# [dependencies]
# my_crate = { version = "0.1", features = ["serde", "async"] }
```

### Feature Documentation

Document features in README and crate docs:

```rust
//! # Optional Features
//!
//! - `std` (enabled by default): Enable standard library support
//! - `serde`: Enable serialization with serde
//! - `async`: Enable async runtime support with tokio
//! - `full`: Enable all optional features
//!
//! ```toml
//! [dependencies]
//! my_crate = { version = "0.1", features = ["serde"] }
//! ```
```

### Best Practices

1. **Default Features**: Only enable features needed by most users
2. **Feature Names**: Use descriptive, lowercase names
3. **Dependencies**: Make heavy dependencies optional
4. **Testing**: Test with and without features
5. **Documentation**: Clearly document what each feature enables

---

## Publishing Workspaces

### Workspace Structure

```toml
# Workspace root Cargo.toml
[workspace]
members = [
    "my_crate_core",
    "my_crate_macros",
    "my_crate",
]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my_crate"

[workspace.dependencies]
# Shared dependencies
serde = { version = "1.0", features = ["derive"] }
```

### Member Crate Configuration

```toml
# my_crate_core/Cargo.toml
[package]
name = "my_crate_core"
version.workspace = true
edition.workspace = true
authors.workspace = true
license.workspace = true
repository.workspace = true
description = "Core functionality for my_crate"
keywords = ["core", "utility"]
categories = ["development-tools"]

[dependencies]
serde.workspace = true
```

### Publishing Workspace Crates

Publish in dependency order:

```bash
# 1. Publish core library (no internal dependencies)
cd my_crate_core
cargo publish

# 2. Publish macros (depends on core)
cd ../my_crate_macros
cargo publish

# Wait a few minutes for crates.io to update

# 3. Publish main crate (depends on core and macros)
cd ../my_crate
cargo publish
```

### Version Synchronization

Keep versions synchronized:

```toml
# my_crate/Cargo.toml
[dependencies]
my_crate_core = { version = "0.1.0", path = "../my_crate_core" }
my_crate_macros = { version = "0.1.0", path = "../my_crate_macros" }
```

When publishing new versions:
1. Update version in workspace `Cargo.toml`
2. Update version references in dependency declarations
3. Publish in correct order

### Workspace Publishing Script

```bash
#!/bin/bash
# publish_all.sh

set -e

VERSION="0.2.0"

echo "Publishing workspace version $VERSION"

# Update version in workspace
sed -i "s/^version = .*/version = \"$VERSION\"/" Cargo.toml

# Publish core
echo "Publishing my_crate_core..."
cd my_crate_core
cargo publish
cd ..

# Wait for crates.io
echo "Waiting for crates.io to update..."
sleep 60

# Publish macros
echo "Publishing my_crate_macros..."
cd my_crate_macros
cargo publish
cd ..

sleep 60

# Publish main crate
echo "Publishing my_crate..."
cd my_crate
cargo publish
cd ..

echo "All crates published successfully!"

# Tag release
git tag -a "v$VERSION" -m "Release v$VERSION"
git push origin "v$VERSION"
```

---

## Publishing Binary Crates

### Binary vs Library Crates

**Library Crate**: Code that other crates depend on (has `lib.rs`)
**Binary Crate**: Executable program (has `main.rs`)

Many crates are both:
```
my_tool/
├── Cargo.toml
├── src/
│   ├── lib.rs    # Library functionality
│   └── main.rs   # CLI wrapper
```

### Binary Crate Cargo.toml

```toml
[package]
name = "my_cli_tool"
version = "0.1.0"
edition = "2021"
description = "A command-line tool for doing awesome things"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/my_cli_tool"
keywords = ["cli", "tool", "utility"]
categories = ["command-line-utilities"]

# Binary crates should include Cargo.lock
# Add to version control and package
include = [
    "src/**/*",
    "Cargo.toml",
    "Cargo.lock",
    "README.md",
    "LICENSE",
]

[[bin]]
name = "my_tool"
path = "src/main.rs"

[dependencies]
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
```

### Installing Binary Crates

```bash
# Install from crates.io
cargo install my_cli_tool

# Install specific version
cargo install my_cli_tool --version 0.1.0

# Install with features
cargo install my_cli_tool --features extra

# Install from git
cargo install --git https://github.com/username/my_cli_tool

# Uninstall
cargo uninstall my_cli_tool
```

### Multiple Binaries

```toml
[[bin]]
name = "tool1"
path = "src/bin/tool1.rs"

[[bin]]
name = "tool2"
path = "src/bin/tool2.rs"
```

Directory structure:
```
src/
├── lib.rs
├── bin/
│   ├── tool1.rs
│   └── tool2.rs
```

### Cross-Platform Considerations

```rust
// Platform-specific code
#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows implementation
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux implementation
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS implementation
}
```

### CI/CD for Binaries

GitHub Actions example:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Publish to crates.io
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: cargo publish
```

---

## Post-Publishing Maintenance

### Monitoring Your Crate

**crates.io Statistics**:
- Downloads per version
- Recent downloads trend
- Total downloads

**docs.rs**:
- Documentation build status
- Build logs
- Supported platforms

### Responding to Issues

When users report bugs:

```bash
# 1. Fix the bug
# 2. Add test to prevent regression
# 3. Update version (patch)
# version = "0.1.1"

# 4. Update CHANGELOG.md
# ## [0.1.1] - 2025-01-20
# ### Fixed
# - Bug in feature X (#123)

# 5. Publish
cargo publish

# 6. Tag release
git tag v0.1.1
git push origin v0.1.1
```

### Deprecating Features

```rust
#[deprecated(since = "0.2.0", note = "use `new_function` instead")]
pub fn old_function() {
    // Old implementation
}

pub fn new_function() {
    // New implementation
}
```

In CHANGELOG.md:
```markdown
## [0.2.0] - 2025-01-25
### Deprecated
- `old_function` is deprecated, use `new_function` instead
```

### Deprecating Entire Crates

In README.md and crate documentation:

```markdown
# ⚠️ DEPRECATED

This crate is deprecated. Please use [`new_crate`](https://crates.io/crates/new_crate) instead.
```

```rust
//! # ⚠️ DEPRECATED
//!
//! This crate is deprecated. Use [`new_crate`] instead.
//!
//! [`new_crate`]: https://crates.io/crates/new_crate
```

### Transferring Ownership

To transfer a crate to another maintainer:

1. Add them as an owner on crates.io:
   ```bash
   cargo owner --add github:username
   ```

2. They can then publish updates

3. Remove yourself if desired:
   ```bash
   cargo owner --remove github:your_username
   ```

### Archiving Unmaintained Crates

If you can no longer maintain a crate:

1. Update README with maintenance status
2. Archive GitHub repository
3. Consider finding a new maintainer
4. Add deprecation notice if there's a replacement

---

## Best Practices

### 1. Documentation

✅ **Do**:
- Write comprehensive crate-level documentation
- Include examples in documentation
- Test all documentation examples
- Provide a detailed README
- Maintain a CHANGELOG

❌ **Don't**:
- Leave public items undocumented
- Write examples that don't compile
- Forget to update docs when API changes

### 2. Versioning

✅ **Do**:
- Follow semantic versioning strictly
- Document breaking changes clearly
- Use 0.x.y for pre-1.0 development
- Tag releases in git

❌ **Don't**:
- Make breaking changes in patch versions
- Re-publish the same version
- Yank versions unnecessarily

### 3. Testing

✅ **Do**:
- Achieve high test coverage
- Test all feature combinations
- Run clippy and fix warnings
- Format code with rustfmt
- Test documentation examples

❌ **Don't**:
- Publish untested code
- Ignore clippy warnings
- Skip testing edge cases

### 4. Dependencies

✅ **Do**:
- Use minimal dependencies
- Pin dependency versions conservatively
- Make heavy dependencies optional
- Document required features

❌ **Don't**:
- Add unnecessary dependencies
- Use deprecated crates
- Forget to specify version constraints
- Make all dependencies required

### 5. API Design

✅ **Do**:
- Design for extensibility
- Use builder patterns for complex configuration
- Provide sensible defaults
- Make common cases easy

❌ **Don't**:
- Change public API in minor versions
- Expose implementation details
- Use overly generic names
- Break backward compatibility casually

### 6. Licensing

✅ **Do**:
- Use standard licenses (MIT/Apache-2.0)
- Include LICENSE files
- Document dependency licenses
- Be consistent across workspace

❌ **Don't**:
- Forget to specify a license
- Use incompatible licenses
- Change licenses after publishing

### 7. Metadata

✅ **Do**:
- Write clear, concise descriptions
- Choose relevant keywords
- Select appropriate categories
- Provide repository link

❌ **Don't**:
- Use generic keywords
- Exaggerate capabilities
- Forget to update metadata

### 8. Publishing

✅ **Do**:
- Test thoroughly before publishing
- Use `--dry-run` first
- Verify package contents
- Tag releases in git

❌ **Don't**:
- Publish directly from feature branches
- Skip pre-publish checks
- Forget to commit changes

---

## Common Issues and Solutions

### Issue: "Crate name is already taken"

**Solution**:
- Choose a different, unique name
- Consider using a prefix (e.g., `myproject_utils`)
- Check crates.io for similar names

### Issue: "Missing required field: license"

**Solution**:
```toml
# Add to Cargo.toml
license = "MIT OR Apache-2.0"

# Or specify license file
license-file = "LICENSE"
```

### Issue: "Missing required field: description"

**Solution**:
```toml
# Add to Cargo.toml
description = "A brief description of what this crate does"
```

### Issue: "Failed to verify package"

**Causes**:
- Missing files in package
- Tests fail when built from package
- Path dependencies not resolved

**Solution**:
```bash
# Check package contents
cargo package --list

# Verify locally
cargo package --verify

# Check test output
cargo test --package your_crate
```

### Issue: "Documentation build failed"

**Solution**:
```bash
# Build docs locally
cargo doc --no-deps

# Check for errors
cargo rustdoc -- -D warnings

# Fix broken doc links
# Fix code examples in docs
```

### Issue: "Version X.Y.Z already exists"

**Solution**:
- You cannot re-publish the same version
- Bump version number in Cargo.toml
- Versions are immutable on crates.io

### Issue: "Package size exceeds limit"

**Solution**:
```toml
# Exclude large files
exclude = [
    "tests/fixtures/large_files/",
    "docs/images/",
    "*.mp4",
]
```

### Issue: "Authentication failed"

**Solution**:
```bash
# Re-login
cargo login <your-new-token>

# Check credentials file
cat ~/.cargo/credentials.toml

# Ensure token is valid on crates.io
```

### Issue: "Workspace members not found"

**Solution**:
- Ensure member crates are published first
- Wait a few minutes after publishing dependencies
- Check version numbers match

### Issue: "Yanked dependency"

**Solution**:
```toml
# Update to non-yanked version
[dependencies]
some_crate = "0.1.2"  # Instead of yanked 0.1.1
```

### Issue: "Path dependencies in published crate"

**Solution**:
```toml
# Replace path with version for publishing
[dependencies]
my_other_crate = "0.1.0"  # Instead of path = "../my_other_crate"

# Or use both for development
[dependencies]
my_other_crate = { version = "0.1.0", path = "../my_other_crate" }
```

---

## Advanced Topics

### Custom Registries

```toml
# .cargo/config.toml
[registries.my-registry]
index = "https://my-registry.example.com/index"

[registry]
default = "my-registry"
```

```bash
# Publish to custom registry
cargo publish --registry my-registry
```

### docs.rs Configuration

```toml
[package.metadata.docs.rs]
# Build with all features
all-features = true

# Or specific features
features = ["serde", "async"]

# Custom rustdoc arguments
rustdoc-args = ["--cfg", "docsrs"]

# Target platforms
targets = ["x86_64-unknown-linux-gnu", "wasm32-unknown-unknown"]
```

### Build Scripts

For crates with build.rs:

```toml
[package]
build = "build.rs"

[build-dependencies]
cc = "1.0"
```

Ensure build.rs is included in package.

### Badges

Add badges to README:

```markdown
[![Crates.io](https://img.shields.io/crates/v/my_crate.svg)](https://crates.io/crates/my_crate)
[![Documentation](https://docs.rs/my_crate/badge.svg)](https://docs.rs/my_crate)
[![License](https://img.shields.io/crates/l/my_crate.svg)](https://github.com/username/my_crate#license)
[![Downloads](https://img.shields.io/crates/d/my_crate.svg)](https://crates.io/crates/my_crate)
```

---

## Conclusion

Publishing to crates.io is a rewarding way to contribute to the Rust ecosystem. By following the guidelines in this tutorial, you can:

- Prepare professional, well-documented crates
- Publish with confidence using best practices
- Maintain your crates effectively
- Build a positive reputation in the community

Remember:
- **Documentation** is crucial for adoption
- **Testing** ensures reliability
- **Versioning** maintains trust
- **Maintenance** keeps users happy

Happy publishing! 🦀

---

## Resources

- **crates.io**: https://crates.io
- **docs.rs**: https://docs.rs
- **Cargo Book**: https://doc.rust-lang.org/cargo/
- **Rust API Guidelines**: https://rust-lang.github.io/api-guidelines/
- **Semantic Versioning**: https://semver.org
- **Choose a License**: https://choosealicense.com
- **Rust Packaging Guide**: https://doc.rust-lang.org/cargo/reference/publishing.html

---

## Examples in This Module

1. **first_publish_demo**: Basic library with minimal publishing requirements
2. **feature_flags_demo**: Library demonstrating optional features
3. **workspace_publish_demo**: Multi-crate workspace publishing
4. **binary_crate_demo**: CLI tool publishing example

Each example includes complete source code, documentation, and publishing configuration.
