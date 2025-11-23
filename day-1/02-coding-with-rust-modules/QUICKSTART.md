# Coding with Rust Modules - Quick Start

## Running the Examples

### 1. Library System

```powershell
cd library_system
cargo run
```

Demonstrates:
- Module organization with `mod.rs`
- Public and private items
- Module visibility with `pub(crate)`
- Multiple related modules

### 2. Calculator Modules

```powershell
cd calculator_modules
cargo run
```

Demonstrates:
- Organizing functionality by domain
- Re-exporting with `pub use`
- Submodules (basic, advanced, statistics)
- Module testing

### 3. User Management

```powershell
cd user_management
cargo run
```

Demonstrates:
- Privacy and encapsulation
- Authentication system
- Crate-level privacy `pub(crate)`
- Secure data handling

## Running Tests

Each project includes unit tests:

```powershell
# Library System
cd library_system
cargo test

# Calculator
cd calculator_modules
cargo test

# User Management
cd user_management
cargo test
```

## Project Structure Examples

### Library System Structure
```
library_system/
├── Cargo.toml
└── src/
    ├── main.rs
    └── library/
        ├── mod.rs           # Module root
        ├── member.rs        # Member management
        └── transaction.rs   # Borrowing/returning
```

### Calculator Structure
```
calculator_modules/
├── Cargo.toml
└── src/
    ├── main.rs
    └── math/
        ├── mod.rs          # Math module root
        ├── basic.rs        # Basic operations
        ├── advanced.rs     # Advanced operations
        └── statistics.rs   # Statistical functions
```

### User Management Structure
```
user_management/
├── Cargo.toml
└── src/
    ├── main.rs
    ├── user.rs      # User struct and methods
    └── auth.rs      # Authentication service
```

## Key Concepts Demonstrated

### Privacy Levels
- **Private (default)**: Only accessible within module
- **`pub`**: Public, accessible from anywhere
- **`pub(crate)`**: Public within the crate only
- **`pub(super)`**: Public to parent module

### Module Patterns
- **Inline modules**: `mod name { ... }`
- **File modules**: `mod name;` (looks for `name.rs`)
- **Directory modules**: `mod name;` (looks for `name/mod.rs`)

### Path Types
- **Absolute**: `crate::module::item`
- **Relative**: `super::item` or `self::item`
- **Use statement**: `use crate::module::item;`

## Troubleshooting

### "Cannot find module"
Make sure your file structure matches your module declarations.

### "Private item"
Add `pub` keyword to make items accessible outside their module.

### "Circular dependency"
Restructure your modules to avoid circular imports.

## Next Steps

1. ✅ Study the code organization in each example
2. ✅ Try adding new modules to existing projects
3. ✅ Complete the exercises in README.md
4. ✅ Experiment with different privacy levels
5. ✅ Build your own modular application

Happy coding! 🦀
