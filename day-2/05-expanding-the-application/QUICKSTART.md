# Expanding the Application - Quick Reference

## Application Structure Overview

| Component | Purpose | Key Files |
|-----------|---------|-----------|
| `src/main.rs` | Entry point | Main function, CLI |
| `src/lib.rs` | Library crate | Public API exports |
| `src/models/` | Data structures | Structs, enums |
| `src/handlers/` | Business logic | Core functionality |
| `src/utils/` | Helper functions | Utilities, validators |
| `Cargo.toml` | Configuration | Dependencies, metadata |

## Project Organization

### Module Structure
```rust
// src/lib.rs
pub mod models;
pub mod handlers;
pub mod utils;

// Re-export commonly used items
pub use models::User;
pub use handlers::UserHandler;
```

### File Organization
```
project/
├── Cargo.toml
├── src/
│   ├── main.rs           # Binary entry point
│   ├── lib.rs            # Library root
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── user_handler.rs
│   └── utils/
│       ├── mod.rs
│       └── validators.rs
└── tests/
    └── integration_test.rs
```

## Adding Features

### Step 1: Define Models
```rust
// src/models/user.rs
#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}
```

### Step 2: Create Handlers
```rust
// src/handlers/user_handler.rs
use crate::models::User;

pub struct UserHandler {
    users: Vec<User>,
}

impl UserHandler {
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }
    
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }
    
    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
}
```

### Step 3: Add Utilities
```rust
// src/utils/validators.rs
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn validate_name(name: &str) -> bool {
    !name.is_empty() && name.len() <= 100
}
```

## Error Handling

### Custom Error Types
```rust
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    NotFound(String),
    DatabaseError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
```

### Result Type Alias
```rust
pub type AppResult<T> = Result<T, AppError>;

pub fn get_user(id: u32) -> AppResult<User> {
    // Implementation
    Ok(user)
}
```

## Configuration Management

### Config File (TOML)
```toml
# config.toml
[app]
name = "MyApp"
version = "1.0.0"

[database]
host = "localhost"
port = 5432

[logging]
level = "info"
```

### Loading Config
```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
}

#[derive(Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
}
```

## Testing

### Unit Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_user() {
        let mut handler = UserHandler::new();
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
        handler.add_user(user);
        assert_eq!(handler.users.len(), 1);
    }
}
```

### Integration Tests
```rust
// tests/integration_test.rs
use my_app::UserHandler;

#[test]
fn test_user_workflow() {
    let mut handler = UserHandler::new();
    // Test complete workflow
}
```

## CLI Enhancement

### Command-Line Arguments
```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <command>", args[0]);
        return;
    }
    
    match args[1].as_str() {
        "add" => { /* handle add */ },
        "list" => { /* handle list */ },
        _ => println!("Unknown command"),
    }
}
```

### Using clap (recommended)
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    List,
}
```

## File I/O

### Reading Files
```rust
use std::fs;

fn read_data(path: &str) -> std::io::Result<String> {
    fs::read_to_string(path)
}
```

### Writing Files
```rust
use std::fs::File;
use std::io::Write;

fn write_data(path: &str, data: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_bytes())
}
```

### JSON Serialization
```rust
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Data {
    items: Vec<String>,
}

fn save_json(data: &Data, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(path, json)
}

fn load_json(path: &str) -> Result<Data, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&content)?)
}
```

## Common Patterns

### Builder Pattern
```rust
pub struct UserBuilder {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            email: None,
        }
    }
    
    pub fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    
    pub fn build(self) -> Result<User, String> {
        Ok(User {
            id: self.id.ok_or("ID required")?,
            name: self.name.ok_or("Name required")?,
            email: self.email.ok_or("Email required")?,
        })
    }
}
```

### Factory Pattern
```rust
pub trait ProductFactory {
    fn create(&self, name: &str) -> Box<dyn Product>;
}

pub struct ConcreteFactory;

impl ProductFactory for ConcreteFactory {
    fn create(&self, name: &str) -> Box<dyn Product> {
        match name {
            "A" => Box::new(ProductA),
            "B" => Box::new(ProductB),
            _ => Box::new(DefaultProduct),
        }
    }
}
```

### Repository Pattern
```rust
pub trait Repository<T> {
    fn add(&mut self, item: T) -> Result<(), String>;
    fn get(&self, id: u32) -> Option<&T>;
    fn update(&mut self, id: u32, item: T) -> Result<(), String>;
    fn delete(&mut self, id: u32) -> Result<(), String>;
    fn list(&self) -> Vec<&T>;
}
```

## Logging

### Using log crate
```rust
use log::{debug, info, warn, error};

fn process_data() {
    info!("Starting data processing");
    debug!("Processing item: {}", item);
    warn!("Low memory warning");
    error!("Failed to process: {}", error);
}
```

### Setup Logging
```rust
// In main.rs
use env_logger;

fn main() {
    env_logger::init();
    info!("Application started");
}
```

## Database Integration

### Using rusqlite
```rust
use rusqlite::{Connection, Result};

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn insert_user(conn: &Connection, user: &User) -> Result<()> {
    conn.execute(
        "INSERT INTO users (id, name, email) VALUES (?1, ?2, ?3)",
        [&user.id.to_string(), &user.name, &user.email],
    )?;
    Ok(())
}
```

## Performance Tips

1. **Use Vec with capacity**
   ```rust
   let mut items = Vec::with_capacity(1000);
   ```

2. **Avoid unnecessary clones**
   ```rust
   // Bad
   fn process(data: Vec<String>) { }
   
   // Good
   fn process(data: &[String]) { }
   ```

3. **Use references in loops**
   ```rust
   for item in &items {  // Borrow, don't move
       println!("{}", item);
   }
   ```

4. **Cache computed values**
   ```rust
   let len = items.len();  // Cache length
   for i in 0..len {
       // Use cached len
   }
   ```

## Best Practices

### 1. Separation of Concerns
- Keep models, handlers, and utilities separate
- Each module has a single responsibility
- Use clear naming conventions

### 2. Error Handling
- Use `Result` for recoverable errors
- Use `panic!` only for unrecoverable errors
- Provide meaningful error messages

### 3. Documentation
```rust
/// Adds a new user to the system
///
/// # Arguments
/// * `user` - The user to add
///
/// # Returns
/// * `Ok(())` if successful
/// * `Err(AppError)` if validation fails
pub fn add_user(&mut self, user: User) -> AppResult<()> {
    // Implementation
}
```

### 4. Code Organization
- Keep files under 500 lines
- Group related functionality
- Use meaningful file and module names

### 5. Testing
- Write tests for public APIs
- Test edge cases
- Use descriptive test names

## Common Dependencies

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
clap = { version = "4.0", features = ["derive"] }
log = "0.4"
env_logger = "0.10"
rusqlite = "0.29"
chrono = "0.4"
uuid = { version = "1.0", features = ["v4"] }
```

## Quick Commands

```bash
# Create new module
mkdir src/models
touch src/models/mod.rs

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run

# Build release
cargo build --release

# Check without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Troubleshooting

### Issue: Module not found
**Solution**: Ensure module is declared in parent `mod.rs` or `lib.rs`
```rust
pub mod models;
pub mod handlers;
```

### Issue: Borrow checker errors
**Solution**: Review ownership rules, use references appropriately
```rust
// Bad
let data = process(items);  // Moves items

// Good
let data = process(&items);  // Borrows items
```

### Issue: Trait not in scope
**Solution**: Import trait explicitly
```rust
use std::io::Write;  // Import Write trait
```

### Issue: Cannot find type in scope
**Solution**: Check module visibility and imports
```rust
pub use models::User;  // Re-export for easier access
```

## Additional Resources

- [The Rust Book - Managing Growing Projects](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- [API Guidelines](https://rust-lang.github.io/api-guidelines/)
