# Publishing to crates.io - Quick Reference

## Prerequisites

### Create crates.io Account
1. Visit [crates.io](https://crates.io)
2. Sign in with GitHub account
3. Go to Account Settings → API Tokens
4. Generate new token
5. Save token securely

### Login to crates.io
```bash
cargo login <your-api-token>
# Token saved to ~/.cargo/credentials.toml
```

## Publishing Checklist

| Step | Command | Purpose |
|------|---------|---------|
| 1. Complete Cargo.toml | Edit file | Add metadata |
| 2. Add LICENSE file | Create file | Choose license |
| 3. Write README.md | Create file | Documentation |
| 4. Run tests | `cargo test` | Verify functionality |
| 5. Check documentation | `cargo doc --open` | Review docs |
| 6. Package crate | `cargo package` | Create .crate file |
| 7. Dry run publish | `cargo publish --dry-run` | Test publishing |
| 8. Publish | `cargo publish` | Upload to crates.io |

## Required Cargo.toml Fields

```toml
[package]
name = "my_crate"              # Required: unique name
version = "0.1.0"              # Required: semver format
edition = "2021"               # Required: Rust edition
authors = ["Your Name <you@example.com>"]  # Optional but recommended
description = "A brief description"         # Required for publishing
license = "MIT OR Apache-2.0"              # Required for publishing
repository = "https://github.com/user/repo" # Recommended
documentation = "https://docs.rs/my_crate"  # Optional
homepage = "https://example.com"            # Optional
readme = "README.md"                        # Recommended
keywords = ["keyword1", "keyword2"]         # Max 5, lowercase
categories = ["development-tools"]          # From crates.io list
```

## Version Management (Semantic Versioning)

```
MAJOR.MINOR.PATCH

Examples:
0.1.0  → Initial development
0.1.1  → Bug fix
0.2.0  → New feature (backward compatible)
1.0.0  → First stable release
2.0.0  → Breaking changes
```

### Version Bumping
```bash
# Manual: Edit Cargo.toml version field
# Then publish
cargo publish
```

## Common Publishing Commands

```bash
# Login (one-time)
cargo login <token>

# Package locally
cargo package

# List package contents
cargo package --list

# Test publishing
cargo publish --dry-run

# Publish to crates.io
cargo publish

# Yank a version (prevents new usage)
cargo yank --vers 0.1.0

# Un-yank a version
cargo yank --vers 0.1.0 --undo
```

## License Options

### Common Licenses
```toml
# Dual license (recommended)
license = "MIT OR Apache-2.0"

# Single license
license = "MIT"
license = "Apache-2.0"
license = "GPL-3.0"
license = "BSD-3-Clause"

# Custom license file
license-file = "LICENSE.txt"
```

### LICENSE File Template (MIT)
```
MIT License

Copyright (c) 2025 Your Name

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction...
```

## README.md Template

```markdown
# Crate Name

Brief description of your crate.

## Installation

\`\`\`toml
[dependencies]
my_crate = "0.1.0"
\`\`\`

## Quick Start

\`\`\`rust
use my_crate::SomeType;

fn main() {
    let instance = SomeType::new();
    // Example usage
}
\`\`\`

## Documentation

See [docs.rs](https://docs.rs/my_crate) for full documentation.

## License

Licensed under MIT OR Apache-2.0
```

## Documentation Requirements

### Crate-level Documentation
```rust
//! # My Crate
//! 
//! Brief description.
//! 
//! # Examples
//! 
//! ```
//! use my_crate::function;
//! 
//! let result = function();
//! ```
```

### Function Documentation
```rust
/// Does something useful
///
/// # Examples
///
/// ```
/// use my_crate::my_function;
/// assert_eq!(my_function(5), 10);
/// ```
///
/// # Errors
///
/// Returns `Err` if input is invalid.
pub fn my_function(n: i32) -> Result<i32, Error> {
    Ok(n * 2)
}
```

## Testing Before Publishing

```bash
# Run all tests
cargo test

# Test documentation
cargo test --doc

# Check for warnings
cargo clippy

# Format code
cargo fmt

# Build documentation
cargo doc --no-deps --open

# Verify package contents
cargo package --list
```

## Publishing Workflow

```bash
# 1. Update version in Cargo.toml
# version = "0.2.0"

# 2. Run full test suite
cargo test --all-features

# 3. Check clippy warnings
cargo clippy -- -D warnings

# 4. Format code
cargo fmt

# 5. Build and check docs
cargo doc --no-deps

# 6. Package and verify
cargo package --list

# 7. Dry run
cargo publish --dry-run

# 8. Publish!
cargo publish

# 9. Tag release in git
git tag v0.2.0
git push origin v0.2.0
```

## Feature Flags

```toml
[features]
default = ["std"]
std = []
serde = ["dep:serde"]
full = ["std", "serde"]

[dependencies]
serde = { version = "1.0", optional = true }
```

```bash
# Publish with all features
cargo publish --all-features

# Test all feature combinations
cargo test --no-default-features
cargo test --all-features
```

## Categories (crates.io)

Select from official categories:
- `algorithms`
- `api-bindings`
- `async`
- `authentication`
- `command-line-utilities`
- `config`
- `cryptography`
- `database`
- `data-structures`
- `date-and-time`
- `development-tools`
- `encoding`
- `filesystem`
- `game-development`
- `network-programming`
- `parser-implementations`
- `web-programming`

## Keywords Best Practices

```toml
# Good keywords (max 5)
keywords = ["cli", "parser", "json", "utility"]

# Rules:
# - Lowercase only
# - No spaces (use hyphens)
# - Max 5 keywords
# - Be specific and relevant
```

## Common Publishing Errors

| Error | Cause | Solution |
|-------|-------|----------|
| Name already taken | Crate exists | Choose different name |
| Missing license | No license field | Add `license` or `license-file` |
| Missing description | No description | Add description in Cargo.toml |
| Version exists | Re-publishing | Bump version number |
| Test failures | Broken code | Fix tests |
| Too large | Package > 10MB | Use `.cargo_vcs_info.json` exclude |

## Excluding Files from Package

```toml
[package]
exclude = [
    ".github/",
    "tests/fixtures/large_data/",
    "*.swp",
    ".DS_Store",
]

# Or use include (whitelist)
include = [
    "src/**/*",
    "Cargo.toml",
    "README.md",
    "LICENSE",
]
```

## Yanking Versions

```bash
# Yank version (prevents new projects from using it)
cargo yank --vers 0.1.0

# Existing users can still use it
# Just prevents new Cargo.lock entries

# Un-yank if mistake
cargo yank --vers 0.1.0 --undo
```

## Post-Publishing

### Verify Publication
```bash
# View on crates.io
https://crates.io/crates/your_crate

# Check docs.rs
https://docs.rs/your_crate

# Test installation
cargo install your_crate
```

### Update GitHub
```bash
# Tag the release
git tag -a v0.1.0 -m "Release v0.1.0"
git push origin v0.1.0

# Create GitHub release
# - Go to Releases
# - Create from tag
# - Add changelog
```

## Maintenance

### Publishing Updates
```bash
# 1. Fix bugs or add features
# 2. Update version in Cargo.toml
# 3. Update CHANGELOG.md
# 4. Run tests
cargo test

# 5. Publish
cargo publish

# 6. Tag in git
git tag v0.1.1
git push origin v0.1.1
```

### Deprecating Crates
```toml
# In README.md and crate docs
# DEPRECATED: Use `better_crate` instead
```

## Best Practices

1. **Semantic Versioning**
   - Follow SemVer strictly
   - 0.x.y for development
   - 1.0.0 for stable API

2. **Documentation**
   - Document all public items
   - Include examples
   - Test documentation examples

3. **Testing**
   - Comprehensive test coverage
   - Test all feature combinations
   - CI/CD integration

4. **Licensing**
   - Use dual MIT/Apache-2.0
   - Include LICENSE file
   - Document dependencies' licenses

5. **README**
   - Clear installation instructions
   - Quick start example
   - Link to full documentation

6. **Changelog**
   - Maintain CHANGELOG.md
   - Document breaking changes
   - Credit contributors

## Quick Troubleshooting

```bash
# Can't publish - dirty git
git status
git add .
git commit -m "Release v0.1.0"

# Can't publish - tests fail
cargo test
# Fix issues

# Can't publish - wrong token
cargo login <new-token>

# Can't publish - name taken
# Change name in Cargo.toml

# Can't publish - too large
cargo package --list
# Add to exclude in Cargo.toml
```

## Useful Links

- crates.io: https://crates.io
- docs.rs: https://docs.rs
- Cargo Book: https://doc.rust-lang.org/cargo/
- SemVer: https://semver.org
- Choose License: https://choosealicense.com
- Categories: https://crates.io/categories
- Keywords: https://crates.io/keywords

## Example Complete Cargo.toml

```toml
[package]
name = "awesome_crate"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "An awesome crate that does awesome things"
license = "MIT OR Apache-2.0"
repository = "https://github.com/username/awesome_crate"
documentation = "https://docs.rs/awesome_crate"
homepage = "https://awesome-crate.io"
readme = "README.md"
keywords = ["awesome", "utility", "tool"]
categories = ["development-tools"]
exclude = [".github/", "tests/fixtures/"]

[dependencies]
serde = { version = "1.0", features = ["derive"], optional = true }

[dev-dependencies]
criterion = "0.5"

[features]
default = []
serde_support = ["serde"]

[[example]]
name = "basic"
path = "examples/basic.rs"

[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```
