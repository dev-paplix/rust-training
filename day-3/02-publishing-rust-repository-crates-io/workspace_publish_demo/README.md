# Workspace Publishing Demo

A demonstration of publishing multiple related Rust crates from a single workspace.

## Overview

This workspace contains three related crates that demonstrate how to:

- Structure a workspace with multiple publishable crates
- Manage shared dependencies and metadata
- Handle inter-crate dependencies with proper versioning
- Publish crates in the correct order

## Workspace Structure

```
workspace_publish_demo/
├── Cargo.toml          # Workspace configuration
├── myapp_core/         # Core library (no internal dependencies)
├── myapp_utils/        # Utilities (depends on myapp_core)
└── myapp/              # Main library (depends on core and utils)
```

## Crates

### myapp_core

**Purpose**: Core functionality including data structures and traits

**Dependencies**: None (only external crates: serde, thiserror)

**Publish First**: This crate should be published first since others depend on it.

```toml
[dependencies]
myapp_core = "0.1.0"
```

### myapp_utils

**Purpose**: Utility functions that work with core types

**Dependencies**: `myapp_core`

**Publish Second**: Publish after `myapp_core` is available on crates.io.

```toml
[dependencies]
myapp_utils = "0.1.0"
```

### myapp

**Purpose**: Main high-level API combining core and utils

**Dependencies**: `myapp_core`, `myapp_utils`

**Publish Last**: Publish after both dependencies are available.

```toml
[dependencies]
myapp = "0.1.0"
```

## Publishing Workflow

### 1. Prepare All Crates

```bash
# Test entire workspace
cargo test --workspace

# Check all crates
cargo check --workspace

# Format code
cargo fmt --all

# Run clippy
cargo clippy --workspace
```

### 2. Publish in Dependency Order

```bash
# Step 1: Publish core (no dependencies)
cd myapp_core
cargo publish
cd ..

# Step 2: Wait for crates.io to update (1-2 minutes)
# Then publish utils (depends on core)
cd myapp_utils
cargo publish
cd ..

# Step 3: Wait again, then publish main crate
cd myapp
cargo publish
cd ..
```

### 3. Tag the Release

```bash
# Tag the workspace version
git tag v0.1.0
git push origin v0.1.0
```

## Workspace Configuration

### Shared Metadata

```toml
[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["Rust Training <training@example.com>"]
license = "MIT OR Apache-2.0"
repository = "https://github.com/example/myapp"
```

### Shared Dependencies

```toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
thiserror = "1.0"
```

### Member Crates Use Workspace Values

```toml
[package]
name = "myapp_core"
version.workspace = true
edition.workspace = true
# ... other workspace-inherited fields

[dependencies]
serde.workspace = true
thiserror.workspace = true
```

## Version Management

### Synchronized Versioning

All crates in this workspace use the same version number for simplicity:

- `myapp_core`: 0.1.0
- `myapp_utils`: 0.1.0  
- `myapp`: 0.1.0

When releasing a new version:

1. Update `version` in workspace `Cargo.toml`
2. Update version references in inter-crate dependencies
3. Publish in dependency order

### Example Version Update

```toml
# workspace Cargo.toml
[workspace.package]
version = "0.2.0"  # Update here

# myapp_utils/Cargo.toml
[dependencies]
myapp_core = { version = "0.2.0", path = "../myapp_core" }  # Update

# myapp/Cargo.toml
[dependencies]
myapp_core = { version = "0.2.0", path = "../myapp_core" }  # Update
myapp_utils = { version = "0.2.0", path = "../myapp_utils" }  # Update
```

## Path Dependencies During Development

During development, use path dependencies:

```toml
[dependencies]
myapp_core = { version = "0.1.0", path = "../myapp_core" }
```

**Benefits**:
- Changes to `myapp_core` immediately available
- No need to publish for local testing
- Cargo removes `path` when packaging

## Testing

```bash
# Test entire workspace
cargo test --workspace

# Test specific crate
cargo test -p myapp_core
cargo test -p myapp_utils
cargo test -p myapp

# Test with documentation
cargo test --workspace --doc
```

## Building Documentation

```bash
# Build docs for entire workspace
cargo doc --workspace --open

# Build docs without dependencies
cargo doc --workspace --no-deps
```

## Publishing Script

Create a script to automate the publishing process:

```bash
#!/bin/bash
set -e

VERSION="0.1.0"

echo "Publishing workspace version $VERSION"

# Publish core
echo "Publishing myapp_core..."
cd myapp_core
cargo publish
cd ..

echo "Waiting for crates.io to update..."
sleep 120

# Publish utils
echo "Publishing myapp_utils..."
cd myapp_utils
cargo publish
cd ..

echo "Waiting for crates.io to update..."
sleep 120

# Publish main
echo "Publishing myapp..."
cd myapp
cargo publish
cd ..

echo "All crates published!"

# Tag release
git tag "v$VERSION"
git push origin "v$VERSION"
```

## Common Issues

### Issue: "crate not found"

When publishing `myapp_utils`, if it can't find `myapp_core`:

**Cause**: crates.io hasn't indexed the newly published `myapp_core` yet

**Solution**: Wait 1-2 minutes and try again

### Issue: Path dependencies in published crate

**Solution**: Cargo automatically handles path dependencies when packaging. They're converted to version-only dependencies.

### Issue: Version mismatch

**Cause**: Dependency version doesn't match published version

**Solution**: Ensure version numbers in `Cargo.toml` dependencies match published versions

## Best Practices

1. **Synchronized Versioning**: Keep all workspace crates at the same version
2. **Dependency Order**: Always publish in dependency order (core → utils → main)
3. **Testing**: Test entire workspace before publishing any crate
4. **Documentation**: Document inter-crate relationships in README
5. **Changelog**: Maintain CHANGELOG.md for each crate
6. **Git Tags**: Tag releases with workspace version

## License

All crates in this workspace are licensed under MIT OR Apache-2.0.

## Learning Resources

- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Publishing from Workspaces](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Workspace Dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace)
