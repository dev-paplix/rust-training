# Binary Crate Demo

A demonstration CLI tool for learning how to publish binary crates (executables) to crates.io.

## Overview

This project shows how to create, configure, and publish a command-line tool that users can install via `cargo install`. It demonstrates:

- Building a CLI with `clap`
- Publishing binary crates vs library crates
- Including `Cargo.lock` in binary distributions
- Cross-platform considerations
- Installation and usage patterns

## Installation

### From crates.io (after publishing)

```bash
cargo install binary_crate_demo
```

### From Source

```bash
git clone https://github.com/example/binary_crate_demo
cd binary_crate_demo
cargo install --path .
```

### For Development

```bash
cargo build --release
./target/release/demo-tool --help
```

## Usage

The tool provides several subcommands:

### Greet Command

Greet someone with a friendly message:

```bash
# Single greeting
demo-tool greet Alice

# Multiple greetings
demo-tool greet Bob --count 3
```

### Sum Command

Calculate the sum of numbers:

```bash
demo-tool sum 1 2 3 4 5
# Output: Result: 15

demo-tool sum 10 20 30
# Output: Result: 60
```

### JSON Command

Format JSON for better readability:

```bash
demo-tool json '{"name":"Alice","age":30,"city":"NYC"}'

# Output:
# {
#   "name": "Alice",
#   "age": 30,
#   "city": "NYC"
# }
```

### Info Command

Display system information:

```bash
demo-tool info

# Output:
# System Information
#   OS: linux
#   Architecture: x86_64
#   Tool Version: 0.1.0
```

## Features

### CLI Arguments Parsing

Uses `clap` with derive macros for easy argument parsing:

```rust
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
```

### Colored Output

Provides visually appealing terminal output with the `colored` crate:

```rust
println!("{} Hello, {}!", 
    "→".green(), 
    name.cyan().bold()
);
```

### Error Handling

Uses `anyhow` for ergonomic error handling:

```rust
fn main() -> Result<()> {
    // Commands can return errors easily
    greet_command(&name, count)?;
    Ok(())
}
```

### JSON Support

Serialize and deserialize data with `serde_json`:

```rust
let info = SystemInfo { /* ... */ };
let json = serde_json::to_string_pretty(&info)?;
```

## Building

### Debug Build

```bash
cargo build
./target/debug/demo-tool --version
```

### Release Build (Optimized)

```bash
cargo build --release
./target/release/demo-tool --version
```

Release builds are significantly smaller and faster.

## Testing

### Unit Tests

```bash
cargo test
```

### Integration Tests

```bash
# Tests that verify the CLI works end-to-end
cargo test --test '*'
```

### Manual Testing

```bash
# Help message
cargo run -- --help

# Test each command
cargo run -- greet Alice
cargo run -- sum 1 2 3
cargo run -- json '{"test":true}'
cargo run -- info
```

## Publishing Binary Crates

### Differences from Library Crates

| Aspect | Library Crate | Binary Crate |
|--------|--------------|--------------|
| Main file | `src/lib.rs` | `src/main.rs` |
| Install command | N/A | `cargo install <name>` |
| Usage | Added as dependency | Run as executable |
| Cargo.lock | Not included | **Should be included** |
| Target | Other Rust projects | End users |

### Why Include Cargo.lock?

For binary crates, `Cargo.lock` ensures:
- Reproducible builds
- Same dependencies for all users
- Consistent behavior across installations

In `Cargo.toml`:

```toml
include = [
    "src/**/*",
    "Cargo.toml",
    "Cargo.lock",  # ← Important for binaries!
    "README.md",
    "LICENSE-MIT",
    "LICENSE-APACHE",
]
```

### Publishing Checklist

- [ ] Binary name configured (`[[bin]]` section)
- [ ] `Cargo.lock` included in package
- [ ] README with installation and usage instructions
- [ ] LICENSE files included
- [ ] Help text is clear (`--help` output)
- [ ] Version number is correct
- [ ] All tests pass
- [ ] Cross-platform compatibility considered

### Publishing Steps

```bash
# 1. Test thoroughly
cargo test

# 2. Build release version
cargo build --release

# 3. Test the release build
./target/release/demo-tool --help

# 4. Package the crate
cargo package

# 5. Verify package contents
cargo package --list

# 6. Dry run
cargo publish --dry-run

# 7. Publish!
cargo publish

# 8. Test installation
cargo install binary_crate_demo

# 9. Verify it works
demo-tool --version
demo-tool --help

# 10. Tag the release
git tag v0.1.0
git push origin v0.1.0
```

## Cross-Platform Considerations

### OS-Specific Code

```rust
#[cfg(target_os = "windows")]
fn platform_specific() {
    // Windows-specific code
}

#[cfg(target_os = "linux")]
fn platform_specific() {
    // Linux-specific code
}

#[cfg(target_os = "macos")]
fn platform_specific() {
    // macOS-specific code
}
```

### Path Handling

Always use `std::path::PathBuf` for cross-platform paths:

```rust
use std::path::PathBuf;

let path = PathBuf::from("config").join("settings.json");
```

### Line Endings

Be careful with line endings (`\n` vs `\r\n`) when reading/writing files.

## Binary Distribution

### Via cargo install

Users can install directly from crates.io:

```bash
cargo install binary_crate_demo
```

The binary will be installed to `~/.cargo/bin/demo-tool`.

### Pre-built Binaries (Advanced)

For better user experience, consider providing pre-built binaries:

1. Use GitHub Actions to build for multiple platforms
2. Upload binaries to GitHub Releases
3. Users can download without Rust toolchain

Example platforms:
- `x86_64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`
- `x86_64-apple-darwin`
- `aarch64-apple-darwin` (Apple Silicon)

## Updating the Tool

When publishing updates:

```bash
# 1. Update version in Cargo.toml
# version = "0.2.0"

# 2. Update CHANGELOG.md
# Document changes

# 3. Test
cargo test

# 4. Publish
cargo publish

# Users update with:
cargo install binary_crate_demo --force
```

## Uninstallation

Users can uninstall with:

```bash
cargo uninstall binary_crate_demo
```

## Configuration Files

For tools that need configuration, consider:

- `~/.config/<tool-name>/config.toml` (Linux/macOS)
- `%APPDATA%\<tool-name>\config.toml` (Windows)

Use the `directories` crate for cross-platform config paths:

```rust
use directories::ProjectDirs;

if let Some(proj_dirs) = ProjectDirs::from("com", "example", "demo-tool") {
    let config_dir = proj_dirs.config_dir();
    // Use config_dir...
}
```

## Best Practices

### 1. Clear Help Text

```bash
demo-tool --help
# Should clearly explain what the tool does and how to use it
```

### 2. Versioning

```bash
demo-tool --version
# Should show version number
```

### 3. Error Messages

Provide helpful error messages:

```rust
anyhow::bail!("File not found: {}. Try creating it first.", path);
```

### 4. Exit Codes

Return appropriate exit codes:

```rust
fn main() -> Result<()> {
    // If an error occurs, exits with code 1
    // If successful, exits with code 0
    Ok(())
}
```

### 5. Documentation

- Include a comprehensive README
- Add examples in `--help` output
- Create a man page (advanced)

## Dependencies

This tool uses:

- `clap` - Command-line argument parsing
- `colored` - Terminal colors
- `serde` / `serde_json` - JSON serialization
- `anyhow` - Error handling

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.

## Contributing

This is a demonstration project for educational purposes.

## Learning Resources

- [The Cargo Book - Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Command Line Apps in Rust](https://rust-cli.github.io/book/)
- [clap Documentation](https://docs.rs/clap/)
- [Building CLI Applications](https://rust-lang-nursery.github.io/cli-wg/)
