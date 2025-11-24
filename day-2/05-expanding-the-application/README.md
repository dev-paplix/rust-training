# Expanding the Application in Rust

## Table of Contents
- [Introduction](#introduction)
- [Project Structure](#project-structure)
  - [Organizing Code](#organizing-code)
  - [Module System](#module-system)
  - [Binary vs Library Crates](#binary-vs-library-crates)
- [Adding Features](#adding-features)
  - [Planning Features](#planning-features)
  - [Implementing Models](#implementing-models)
  - [Business Logic Handlers](#business-logic-handlers)
  - [Utility Functions](#utility-functions)
- [Error Handling](#error-handling)
  - [Custom Error Types](#custom-error-types)
  - [Error Propagation](#error-propagation)
  - [Result Type Patterns](#result-type-patterns)
- [Configuration Management](#configuration-management)
  - [Configuration Files](#configuration-files)
  - [Environment Variables](#environment-variables)
  - [Config Structs](#config-structs)
- [File I/O](#file-io)
  - [Reading Files](#reading-files)
  - [Writing Files](#writing-files)
  - [JSON Serialization](#json-serialization)
  - [CSV Processing](#csv-processing)
- [Command-Line Interface](#command-line-interface)
  - [Basic CLI](#basic-cli)
  - [Using clap](#using-clap)
  - [Interactive Menus](#interactive-menus)
- [Testing](#testing)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
  - [Test Organization](#test-organization)
- [Design Patterns](#design-patterns)
  - [Builder Pattern](#builder-pattern)
  - [Factory Pattern](#factory-pattern)
  - [Repository Pattern](#repository-pattern)
  - [Strategy Pattern](#strategy-pattern)
- [Database Integration](#database-integration)
  - [SQLite with rusqlite](#sqlite-with-rusqlite)
  - [CRUD Operations](#crud-operations)
- [Logging](#logging)
- [Best Practices](#best-practices)
- [Examples](#examples)

## Introduction

As applications grow, proper organization becomes crucial. This guide covers how to expand a Rust application from a simple program to a well-structured, maintainable project with multiple modules, features, and components.

### Why Structure Matters

1. **Maintainability**: Easy to understand and modify
2. **Reusability**: Share code across different parts
3. **Testability**: Easier to write comprehensive tests
4. **Scalability**: Add features without complexity explosion
5. **Collaboration**: Multiple developers can work together

## Project Structure

### Organizing Code

A typical Rust project structure:

```
my_app/
├── Cargo.toml              # Project metadata and dependencies
├── Cargo.lock              # Locked dependency versions
├── src/
│   ├── main.rs            # Binary entry point
│   ├── lib.rs             # Library root (optional)
│   ├── models/            # Data structures
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── product.rs
│   ├── handlers/          # Business logic
│   │   ├── mod.rs
│   │   └── user_handler.rs
│   ├── utils/             # Helper functions
│   │   ├── mod.rs
│   │   ├── validators.rs
│   │   └── formatters.rs
│   └── errors.rs          # Error types
├── tests/                 # Integration tests
│   └── integration_test.rs
├── benches/               # Benchmarks
│   └── benchmarks.rs
├── examples/              # Example programs
│   └── demo.rs
└── README.md
```

### Module System

#### Declaring Modules

In `src/lib.rs`:
```rust
// Declare modules
pub mod models;
pub mod handlers;
pub mod utils;
pub mod errors;

// Re-export commonly used items
pub use models::{User, Product};
pub use handlers::{UserHandler, ProductHandler};
pub use errors::AppError;
```

In `src/models/mod.rs`:
```rust
pub mod user;
pub mod product;

// Re-export
pub use user::User;
pub use product::Product;
```

In `src/models/user.rs`:
```rust
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
    
    pub fn display(&self) {
        println!("User #{}: {} ({})", self.id, self.name, self.email);
    }
}
```

#### Using Modules

```rust
// In main.rs
use my_app::{User, UserHandler};

fn main() {
    let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string());
    let mut handler = UserHandler::new();
    handler.add_user(user);
}
```

### Binary vs Library Crates

#### Library Crate (`src/lib.rs`)
- Provides reusable functionality
- Can be used by other crates
- Contains the core logic

```rust
// src/lib.rs
pub mod models;
pub mod handlers;

pub use models::User;
pub use handlers::UserHandler;
```

#### Binary Crate (`src/main.rs`)
- Entry point of the application
- Uses the library crate
- Handles CLI and user interaction

```rust
// src/main.rs
use my_app::{User, UserHandler};

fn main() {
    let mut handler = UserHandler::new();
    // CLI logic here
}
```

#### Both Together

`Cargo.toml`:
```toml
[package]
name = "my_app"
version = "0.1.0"
edition = "2021"

# This creates both a library and a binary
[lib]
name = "my_app"
path = "src/lib.rs"

[[bin]]
name = "my_app"
path = "src/main.rs"
```

## Adding Features

### Planning Features

Before implementing, plan:
1. What data structures are needed?
2. What operations will be performed?
3. How will errors be handled?
4. What are the dependencies between features?

### Implementing Models

Models represent your data structures.

#### Simple Model
```rust
// src/models/product.rs
#[derive(Debug, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
    pub quantity: u32,
}

impl Product {
    pub fn new(id: u32, name: String, price: f64, quantity: u32) -> Self {
        Self { id, name, price, quantity }
    }
    
    pub fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
    
    pub fn is_in_stock(&self) -> bool {
        self.quantity > 0
    }
}
```

#### Model with Validation
```rust
#[derive(Debug, Clone)]
pub struct User {
    id: u32,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: u32, name: String, email: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        Ok(Self { id, name, email })
    }
    
    // Getters
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    // Setters with validation
    pub fn set_email(&mut self, email: String) -> Result<(), String> {
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        self.email = email;
        Ok(())
    }
}
```

#### Model with Relationships
```rust
#[derive(Debug, Clone)]
pub struct Order {
    pub id: u32,
    pub user_id: u32,
    pub items: Vec<OrderItem>,
    pub status: OrderStatus,
}

#[derive(Debug, Clone)]
pub struct OrderItem {
    pub product_id: u32,
    pub quantity: u32,
    pub price: f64,
}

#[derive(Debug, Clone)]
pub enum OrderStatus {
    Pending,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
}

impl Order {
    pub fn total(&self) -> f64 {
        self.items.iter()
            .map(|item| item.price * item.quantity as f64)
            .sum()
    }
    
    pub fn item_count(&self) -> usize {
        self.items.len()
    }
}
```

### Business Logic Handlers

Handlers contain the business logic for your application.

#### Basic Handler
```rust
// src/handlers/user_handler.rs
use crate::models::User;

pub struct UserHandler {
    users: Vec<User>,
    next_id: u32,
}

impl UserHandler {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_user(&mut self, mut user: User) -> u32 {
        let id = self.next_id;
        user.id = id;
        self.users.push(user);
        self.next_id += 1;
        id
    }
    
    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    pub fn get_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id == id)
    }
    
    pub fn update_user(&mut self, id: u32, updated_user: User) -> Result<(), String> {
        let user = self.get_user_mut(id)
            .ok_or("User not found")?;
        *user = updated_user;
        Ok(())
    }
    
    pub fn delete_user(&mut self, id: u32) -> Result<(), String> {
        let pos = self.users.iter()
            .position(|u| u.id == id)
            .ok_or("User not found")?;
        self.users.remove(pos);
        Ok(())
    }
    
    pub fn list_users(&self) -> &[User] {
        &self.users
    }
    
    pub fn search_by_name(&self, name: &str) -> Vec<&User> {
        self.users.iter()
            .filter(|u| u.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }
}
```

#### Handler with Statistics
```rust
impl UserHandler {
    pub fn count(&self) -> usize {
        self.users.len()
    }
    
    pub fn count_by_domain(&self, domain: &str) -> usize {
        self.users.iter()
            .filter(|u| u.email.ends_with(domain))
            .count()
    }
    
    pub fn get_statistics(&self) -> UserStatistics {
        let total = self.users.len();
        let domains = self.get_domain_distribution();
        
        UserStatistics {
            total_users: total,
            domains,
        }
    }
    
    fn get_domain_distribution(&self) -> std::collections::HashMap<String, usize> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        
        for user in &self.users {
            if let Some(domain) = user.email.split('@').nth(1) {
                *map.entry(domain.to_string()).or_insert(0) += 1;
            }
        }
        
        map
    }
}

pub struct UserStatistics {
    pub total_users: usize,
    pub domains: std::collections::HashMap<String, usize>,
}
```

### Utility Functions

Utilities provide helper functions used across the application.

#### Validators
```rust
// src/utils/validators.rs

pub fn validate_email(email: &str) -> bool {
    email.contains('@') && 
    email.contains('.') &&
    email.len() >= 5
}

pub fn validate_name(name: &str) -> bool {
    !name.is_empty() && 
    name.len() <= 100 &&
    name.chars().all(|c| c.is_alphabetic() || c.is_whitespace())
}

pub fn validate_phone(phone: &str) -> bool {
    phone.chars().all(|c| c.is_numeric() || c == '-' || c == ' ') &&
    phone.chars().filter(|c| c.is_numeric()).count() >= 10
}

pub fn validate_price(price: f64) -> bool {
    price >= 0.0 && price < 1_000_000.0
}

pub fn validate_quantity(quantity: u32) -> bool {
    quantity > 0 && quantity < 10000
}
```

#### Formatters
```rust
// src/utils/formatters.rs

pub fn format_currency(amount: f64) -> String {
    format!("${:.2}", amount)
}

pub fn format_phone(phone: &str) -> String {
    let digits: String = phone.chars().filter(|c| c.is_numeric()).collect();
    if digits.len() == 10 {
        format!("({}) {}-{}", 
                &digits[0..3], 
                &digits[3..6], 
                &digits[6..10])
    } else {
        phone.to_string()
    }
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len-3])
    }
}

pub fn format_list<T: std::fmt::Display>(items: &[T]) -> String {
    items.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(", ")
}
```

#### Date/Time Utilities
```rust
// src/utils/datetime.rs
use chrono::{DateTime, Local, NaiveDate};

pub fn current_timestamp() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
        .map_err(|e| format!("Invalid date format: {}", e))
}

pub fn days_between(start: &str, end: &str) -> Result<i64, String> {
    let start_date = parse_date(start)?;
    let end_date = parse_date(end)?;
    Ok((end_date - start_date).num_days())
}
```

## Error Handling

### Custom Error Types

Define application-specific errors.

```rust
// src/errors.rs
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    InvalidInput(String),
    NotFound(String),
    AlreadyExists(String),
    DatabaseError(String),
    IoError(String),
    ValidationError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::AlreadyExists(msg) => write!(f, "Already exists: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AppError::IoError(msg) => write!(f, "I/O error: {}", msg),
            AppError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversion from std::io::Error
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IoError(error.to_string())
    }
}
```

### Error Propagation

Using the `?` operator:

```rust
use crate::errors::AppError;

pub type AppResult<T> = Result<T, AppError>;

pub fn add_user(&mut self, name: String, email: String) -> AppResult<u32> {
    // Validate input
    if name.is_empty() {
        return Err(AppError::ValidationError("Name cannot be empty".to_string()));
    }
    
    if !validate_email(&email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }
    
    // Check if user already exists
    if self.users.iter().any(|u| u.email == email) {
        return Err(AppError::AlreadyExists(format!("User with email {} already exists", email)));
    }
    
    // Create and add user
    let user = User::new(self.next_id, name, email)?;
    let id = self.next_id;
    self.users.push(user);
    self.next_id += 1;
    
    Ok(id)
}
```

### Result Type Patterns

```rust
// Pattern 1: Unwrap with default
let value = result.unwrap_or(default_value);

// Pattern 2: Unwrap or else
let value = result.unwrap_or_else(|_| calculate_default());

// Pattern 3: Map error
let result = operation()
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

// Pattern 4: Match result
match operation() {
    Ok(value) => process(value),
    Err(e) => handle_error(e),
}

// Pattern 5: And then
result
    .and_then(|value| next_operation(value))
    .and_then(|value| final_operation(value))
```

## Configuration Management

### Configuration Files

#### TOML Configuration
```toml
# config.toml
[app]
name = "MyApp"
version = "1.0.0"
debug = false

[database]
path = "data.db"
max_connections = 10

[server]
host = "127.0.0.1"
port = 8080

[logging]
level = "info"
file = "app.log"
```

#### Loading Configuration
```rust
// src/config.rs
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub app: AppConfig,
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub name: String,
    pub version: String,
    pub debug: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub path: String,
    pub max_connections: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LoggingConfig {
    pub level: String,
    pub file: String,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn default() -> Self {
        Self {
            app: AppConfig {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                debug: false,
            },
            database: DatabaseConfig {
                path: "data.db".to_string(),
                max_connections: 10,
            },
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: "app.log".to_string(),
            },
        }
    }
}
```

### Environment Variables

```rust
use std::env;

pub fn load_from_env() -> Config {
    Config {
        app: AppConfig {
            name: env::var("APP_NAME").unwrap_or_else(|_| "MyApp".to_string()),
            version: env::var("APP_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
            debug: env::var("APP_DEBUG").unwrap_or_else(|_| "false".to_string()) == "true",
        },
        // ... other configs
    }
}
```

## File I/O

### Reading Files

```rust
use std::fs;
use std::io::{self, BufRead};

// Read entire file as string
pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

// Read file line by line
pub fn read_lines(path: &str) -> io::Result<Vec<String>> {
    let file = fs::File::open(path)?;
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

// Read binary file
pub fn read_binary(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}
```

### Writing Files

```rust
use std::fs::{self, File};
use std::io::{self, Write};

// Write string to file
pub fn write_file(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

// Append to file
pub fn append_file(path: &str, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    
    writeln!(file, "{}", content)
}

// Write lines
pub fn write_lines(path: &str, lines: &[String]) -> io::Result<()> {
    let content = lines.join("\n");
    fs::write(path, content)
}
```

### JSON Serialization

```rust
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserData {
    pub users: Vec<User>,
}

// Save to JSON
pub fn save_json(data: &UserData, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(data)?;
    fs::write(path, json)?;
    Ok(())
}

// Load from JSON
pub fn load_json(path: &str) -> Result<UserData, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let data: UserData = serde_json::from_str(&content)?;
    Ok(data)
}
```

### CSV Processing

```rust
use csv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserRecord {
    pub id: u32,
    pub name: String,
    pub email: String,
}

// Write CSV
pub fn write_csv(users: &[UserRecord], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = csv::Writer::from_path(path)?;
    
    for user in users {
        writer.serialize(user)?;
    }
    
    writer.flush()?;
    Ok(())
}

// Read CSV
pub fn read_csv(path: &str) -> Result<Vec<UserRecord>, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut users = Vec::new();
    
    for result in reader.deserialize() {
        let user: UserRecord = result?;
        users.push(user);
    }
    
    Ok(users)
}
```

## Command-Line Interface

### Basic CLI

```rust
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== User Management ===");
        println!("1. Add User");
        println!("2. List Users");
        println!("3. Search User");
        println!("0. Exit");
        print!("Choose: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => add_user(),
            "2" => list_users(),
            "3" => search_user(),
            "0" => break,
            _ => println!("Invalid choice"),
        }
    }
}
```

### Using clap

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "myapp")]
#[command(about = "A user management application", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new user
    Add {
        /// User name
        #[arg(short, long)]
        name: String,
        
        /// User email
        #[arg(short, long)]
        email: String,
    },
    
    /// List all users
    List,
    
    /// Search for users
    Search {
        /// Search query
        query: String,
    },
    
    /// Delete a user
    Delete {
        /// User ID
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Add { name, email } => {
            println!("Adding user: {} ({})", name, email);
        }
        Commands::List => {
            println!("Listing all users");
        }
        Commands::Search { query } => {
            println!("Searching for: {}", query);
        }
        Commands::Delete { id } => {
            println!("Deleting user with ID: {}", id);
        }
    }
}
```

## Testing

### Unit Tests

```rust
// In src/handlers/user_handler.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_user() {
        let mut handler = UserHandler::new();
        let user = User::new(1, "Alice".to_string(), "alice@example.com".to_string()).unwrap();
        let id = handler.add_user(user);
        assert_eq!(id, 1);
        assert_eq!(handler.count(), 1);
    }
    
    #[test]
    fn test_get_user() {
        let mut handler = UserHandler::new();
        let user = User::new(1, "Bob".to_string(), "bob@example.com".to_string()).unwrap();
        handler.add_user(user.clone());
        
        let found = handler.get_user(1);
        assert!(found.is_some());
        assert_eq!(found.unwrap().name(), "Bob");
    }
    
    #[test]
    fn test_delete_user() {
        let mut handler = UserHandler::new();
        let user = User::new(1, "Charlie".to_string(), "charlie@example.com".to_string()).unwrap();
        handler.add_user(user);
        
        assert!(handler.delete_user(1).is_ok());
        assert_eq!(handler.count(), 0);
    }
    
    #[test]
    fn test_search_by_name() {
        let mut handler = UserHandler::new();
        handler.add_user(User::new(1, "Alice".to_string(), "alice@example.com".to_string()).unwrap());
        handler.add_user(User::new(2, "Bob".to_string(), "bob@example.com".to_string()).unwrap());
        handler.add_user(User::new(3, "Alice Smith".to_string(), "asmith@example.com".to_string()).unwrap());
        
        let results = handler.search_by_name("Alice");
        assert_eq!(results.len(), 2);
    }
}
```

### Integration Tests

```rust
// tests/integration_test.rs
use my_app::{UserHandler, User};

#[test]
fn test_complete_user_workflow() {
    let mut handler = UserHandler::new();
    
    // Add users
    let user1 = User::new(1, "Alice".to_string(), "alice@example.com".to_string()).unwrap();
    let user2 = User::new(2, "Bob".to_string(), "bob@example.com".to_string()).unwrap();
    
    handler.add_user(user1);
    handler.add_user(user2);
    
    // Verify count
    assert_eq!(handler.count(), 2);
    
    // Search
    let results = handler.search_by_name("Alice");
    assert_eq!(results.len(), 1);
    
    // Delete
    assert!(handler.delete_user(1).is_ok());
    assert_eq!(handler.count(), 1);
}
```

## Design Patterns

### Builder Pattern

```rust
pub struct UserBuilder {
    id: Option<u32>,
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
    phone: Option<String>,
}

impl UserBuilder {
    pub fn new() -> Self {
        Self {
            id: None,
            name: None,
            email: None,
            age: None,
            phone: None,
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
    
    pub fn email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
    
    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }
    
    pub fn phone(mut self, phone: String) -> Self {
        self.phone = Some(phone);
        self
    }
    
    pub fn build(self) -> Result<User, String> {
        Ok(User {
            id: self.id.ok_or("ID required")?,
            name: self.name.ok_or("Name required")?,
            email: self.email.ok_or("Email required")?,
            age: self.age,
            phone: self.phone,
        })
    }
}

// Usage
let user = UserBuilder::new()
    .id(1)
    .name("Alice".to_string())
    .email("alice@example.com".to_string())
    .age(30)
    .build()?;
```

### Repository Pattern

```rust
pub trait Repository<T> {
    fn add(&mut self, item: T) -> Result<u32, String>;
    fn get(&self, id: u32) -> Option<&T>;
    fn update(&mut self, id: u32, item: T) -> Result<(), String>;
    fn delete(&mut self, id: u32) -> Result<(), String>;
    fn list(&self) -> Vec<&T>;
}

pub struct UserRepository {
    users: Vec<User>,
    next_id: u32,
}

impl Repository<User> for UserRepository {
    fn add(&mut self, mut user: User) -> Result<u32, String> {
        user.id = self.next_id;
        self.users.push(user);
        let id = self.next_id;
        self.next_id += 1;
        Ok(id)
    }
    
    fn get(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    fn update(&mut self, id: u32, updated: User) -> Result<(), String> {
        let user = self.users.iter_mut()
            .find(|u| u.id == id)
            .ok_or("User not found")?;
        *user = updated;
        Ok(())
    }
    
    fn delete(&mut self, id: u32) -> Result<(), String> {
        let pos = self.users.iter()
            .position(|u| u.id == id)
            .ok_or("User not found")?;
        self.users.remove(pos);
        Ok(())
    }
    
    fn list(&self) -> Vec<&User> {
        self.users.iter().collect()
    }
}
```

## Database Integration

### SQLite with rusqlite

```rust
use rusqlite::{Connection, Result, params};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }
    
    pub fn create_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                email TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        Ok(())
    }
    
    pub fn insert_user(&self, user: &User) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO users (name, email, created_at) VALUES (?1, ?2, ?3)",
            params![user.name(), user.email(), chrono::Local::now().to_rfc3339()],
        )?;
        Ok(self.conn.last_insert_rowid())
    }
    
    pub fn get_user(&self, id: i64) -> Result<User> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email FROM users WHERE id = ?1"
        )?;
        
        stmt.query_row(params![id], |row| {
            Ok(User::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ).unwrap())
        })
    }
    
    pub fn list_users(&self) -> Result<Vec<User>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, email FROM users ORDER BY id"
        )?;
        
        let users = stmt.query_map([], |row| {
            Ok(User::new(
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
            ).unwrap())
        })?
        .collect::<Result<Vec<_>>>()?;
        
        Ok(users)
    }
}
```

## Logging

```rust
use log::{debug, info, warn, error};
use env_logger;

fn main() {
    // Initialize logger
    env_logger::init();
    
    info!("Application started");
    
    // Your code
    debug!("Debug information");
    warn!("Warning message");
    error!("Error occurred");
}

// In Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.10"

// Run with: RUST_LOG=debug cargo run
```

## Best Practices

1. **Separation of Concerns**: Keep models, handlers, and utilities separate
2. **Error Handling**: Use `Result` for recoverable errors, custom error types
3. **Testing**: Write unit and integration tests
4. **Documentation**: Document public APIs with doc comments
5. **Code Organization**: Keep files under 500 lines, use clear naming
6. **Performance**: Use references, avoid unnecessary clones
7. **Security**: Validate all user input, sanitize data
8. **Consistency**: Follow Rust naming conventions and idioms

## Summary

Expanding an application requires:
- Proper code organization with modules
- Clear separation between models, handlers, and utilities
- Robust error handling
- Configuration management
- File I/O capabilities
- Testing at multiple levels
- Design patterns for maintainability
- Database integration when needed

By following these patterns and practices, you can build scalable, maintainable Rust applications that are easy to extend and modify.
