# Coding with Rust Modules

## Table of Contents

- [Introduction](#introduction)
- [Learning Objectives](#learning-objectives)
- [Understanding Modules](#understanding-modules)
- [Module Basics](#module-basics)
- [Privacy and Visibility](#privacy-and-visibility)
- [File-Based Modules](#file-based-modules)
- [Module Hierarchy](#module-hierarchy)
- [The `use` Keyword](#the-use-keyword)
- [Re-exporting with `pub use`](#re-exporting-with-pub-use)
- [External Crates and Dependencies](#external-crates-and-dependencies)
- [The Module System in Practice](#the-module-system-in-practice)
- [Best Practices](#best-practices)
- [Common Patterns](#common-patterns)
- [Practical Examples](#practical-examples)
- [Exercises](#exercises)
- [Summary](#summary)

---

## Introduction

Rust's module system allows you to organize code into logical units, control privacy, and manage namespaces. As your programs grow, organizing code into modules becomes essential for maintainability, reusability, and clarity.

The Rust module system includes:
- **Modules**: Organize code within a crate
- **Paths**: Name items in the module tree
- **`use` keyword**: Bring paths into scope
- **`pub` keyword**: Make items public
- **Crates**: A tree of modules that produces a library or executable

Understanding modules is crucial for building scalable Rust applications and working with the Rust ecosystem.

---

## Learning Objectives

By completing this lesson, you will be able to:

✅ Understand and create modules in Rust  
✅ Control visibility with `pub` and private items  
✅ Organize code across multiple files  
✅ Use paths to reference items in modules  
✅ Import items with the `use` keyword  
✅ Create and manage module hierarchies  
✅ Work with external crates  
✅ Apply best practices for module organization  
✅ Build real-world modular applications  

---

## Understanding Modules

### What are Modules?

Modules are Rust's way of organizing code into logical units. They help you:

1. **Organize code**: Group related functionality together
2. **Control privacy**: Hide implementation details
3. **Avoid name conflicts**: Create separate namespaces
4. **Enable reuse**: Share code across your project

### The Module Tree

Every Rust crate has an implicit root module:
- For binaries: `src/main.rs`
- For libraries: `src/lib.rs`

All other modules branch from this root.

### Crates vs Modules

**Crate**: The smallest amount of code the Rust compiler considers at a time
- Binary crate: Creates an executable
- Library crate: Creates a library

**Module**: A namespace within a crate

```
Crate (my_program)
└── Root module (main.rs or lib.rs)
    ├── Module A
    │   ├── Submodule A1
    │   └── Submodule A2
    └── Module B
```

---

## Module Basics

### Inline Modules

The simplest way to define a module:

```rust
// Define a module
mod greetings {
    pub fn hello() {
        println!("Hello!");
    }
    
    pub fn goodbye() {
        println!("Goodbye!");
    }
    
    // Private function (not accessible outside module)
    fn internal_message() {
        println!("This is private");
    }
}

fn main() {
    // Access module functions
    greetings::hello();
    greetings::goodbye();
    
    // This won't compile - function is private
    // greetings::internal_message();
}
```

### Nested Modules

Modules can contain other modules:

```rust
mod outer {
    pub mod inner {
        pub fn function() {
            println!("Called from nested module");
        }
    }
}

fn main() {
    outer::inner::function();
}
```

### Module with Structs and Enums

```rust
mod shapes {
    pub struct Circle {
        pub radius: f64,
    }
    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }
    
    impl Circle {
        pub fn new(radius: f64) -> Circle {
            Circle { radius }
        }
        
        pub fn area(&self) -> f64 {
            std::f64::consts::PI * self.radius * self.radius
        }
    }
    
    impl Rectangle {
        pub fn new(width: f64, height: f64) -> Rectangle {
            Rectangle { width, height }
        }
        
        pub fn area(&self) -> f64 {
            self.width * self.height
        }
    }
}

fn main() {
    let circle = shapes::Circle::new(5.0);
    let rectangle = shapes::Rectangle::new(4.0, 6.0);
    
    println!("Circle area: {}", circle.area());
    println!("Rectangle area: {}", rectangle.area());
}
```

---

## Privacy and Visibility

### Default Privacy

In Rust, everything is **private by default**:

```rust
mod my_module {
    // Private function
    fn private_function() {
        println!("This is private");
    }
    
    // Public function
    pub fn public_function() {
        println!("This is public");
        // Can call private functions from within the module
        private_function();
    }
}

fn main() {
    my_module::public_function();  // ✅ OK
    // my_module::private_function();  // ❌ Error: private function
}
```

### Making Items Public

Use `pub` to make items public:

```rust
mod library {
    // Public constant
    pub const MAX_SIZE: usize = 100;
    
    // Public struct
    pub struct Book {
        pub title: String,
        author: String,  // Private field
    }
    
    impl Book {
        // Public associated function
        pub fn new(title: String, author: String) -> Book {
            Book { title, author }
        }
        
        // Public method
        pub fn get_author(&self) -> &str {
            &self.author
        }
    }
}

fn main() {
    let book = library::Book::new(
        String::from("The Rust Book"),
        String::from("Steve Klabnik")
    );
    
    println!("Title: {}", book.title);  // ✅ OK - public field
    // println!("Author: {}", book.author);  // ❌ Error - private field
    println!("Author: {}", book.get_author());  // ✅ OK - public method
}
```

### Struct Field Privacy

```rust
mod person {
    pub struct Person {
        pub name: String,
        age: u32,  // Private
    }
    
    impl Person {
        pub fn new(name: String, age: u32) -> Person {
            Person { name, age }
        }
        
        pub fn get_age(&self) -> u32 {
            self.age
        }
        
        pub fn set_age(&mut self, age: u32) {
            if age > 0 && age < 150 {
                self.age = age;
            }
        }
    }
}

fn main() {
    let mut person = person::Person::new(String::from("Alice"), 30);
    println!("Name: {}", person.name);
    println!("Age: {}", person.get_age());
    
    person.set_age(31);
}
```

### Enum Variants are Public

When you make an enum public, all its variants are public:

```rust
mod traffic {
    pub enum Light {
        Red,
        Yellow,
        Green,
    }
    
    pub fn can_go(light: &Light) -> bool {
        match light {
            Light::Green => true,
            _ => false,
        }
    }
}

fn main() {
    let light = traffic::Light::Green;  // ✅ Variants are public
    println!("Can go: {}", traffic::can_go(&light));
}
```

---

## File-Based Modules

### Single File Module

For larger modules, move them to separate files.

**Project structure:**
```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── greetings.rs
```

**src/greetings.rs:**
```rust
pub fn hello() {
    println!("Hello from greetings module!");
}

pub fn goodbye() {
    println!("Goodbye!");
}
```

**src/main.rs:**
```rust
mod greetings;  // Declares the module (looks for greetings.rs)

fn main() {
    greetings::hello();
    greetings::goodbye();
}
```

### Module Directory

For modules with submodules, create a directory:

**Project structure:**
```
my_project/
├── Cargo.toml
└── src/
    ├── main.rs
    └── math/
        ├── mod.rs       # Module root
        ├── basic.rs     # Submodule
        └── advanced.rs  # Submodule
```

**src/math/mod.rs:**
```rust
pub mod basic;
pub mod advanced;
```

**src/math/basic.rs:**
```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
```

**src/math/advanced.rs:**
```rust
pub fn power(base: i32, exp: u32) -> i32 {
    base.pow(exp)
}

pub fn factorial(n: u32) -> u32 {
    (1..=n).product()
}
```

**src/main.rs:**
```rust
mod math;

fn main() {
    println!("5 + 3 = {}", math::basic::add(5, 3));
    println!("5 - 3 = {}", math::basic::subtract(5, 3));
    println!("2^8 = {}", math::advanced::power(2, 8));
    println!("5! = {}", math::advanced::factorial(5));
}
```

### Alternative: Named File for Module

Modern Rust also supports this structure:

```
my_project/
└── src/
    ├── main.rs
    ├── math.rs       # Module declaration
    └── math/         # Submodules
        ├── basic.rs
        └── advanced.rs
```

**src/math.rs:**
```rust
pub mod basic;
pub mod advanced;
```

This is the preferred modern approach!

---

## Module Hierarchy

### Absolute Paths

Start from the crate root using `crate::`:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
}
```

### Relative Paths

Start from the current module:

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

mod customer {
    pub fn eat_at_restaurant() {
        // Relative path using super
        super::front_of_house::hosting::add_to_waitlist();
    }
}

fn main() {
    customer::eat_at_restaurant();
}
```

### Using `super`

Navigate to parent module:

```rust
fn serve_order() {
    println!("Serving order");
}

mod back_of_house {
    pub fn cook_order() {
        println!("Cooking order");
        super::serve_order();  // Calls parent function
    }
}

fn main() {
    back_of_house::cook_order();
}
```

### Using `self`

Refer to the current module:

```rust
mod utilities {
    pub fn helper() {
        println!("Helper function");
    }
    
    pub fn main_function() {
        self::helper();  // Explicit reference to current module
        helper();        // Also works - implicit
    }
}

fn main() {
    utilities::main_function();
}
```

---

## The `use` Keyword

### Basic Usage

Bring items into scope:

```rust
mod math {
    pub mod operations {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    }
}

// Without use
fn example1() {
    let result = math::operations::add(5, 3);
    println!("Result: {}", result);
}

// With use
use math::operations;

fn example2() {
    let result = operations::add(5, 3);
    println!("Result: {}", result);
}

// Use specific function
use math::operations::multiply;

fn example3() {
    let result = multiply(5, 3);
    println!("Result: {}", result);
}

fn main() {
    example1();
    example2();
    example3();
}
```

### Idiomatic Use Patterns

**For functions**: Bring parent module into scope
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();  // Idiomatic
}
```

**For structs, enums, etc.**: Bring the type itself
```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
}
```

### Using `as` for Renaming

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
```

### Nested Paths

```rust
// Instead of:
// use std::io;
// use std::cmp::Ordering;

// Use nested paths:
use std::{io, cmp::Ordering};

// For same parent:
// use std::io;
// use std::io::Write;

use std::io::{self, Write};
```

### Glob Operator

Bring all public items into scope:

```rust
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    let mut set = HashSet::new();
}
```

**Warning**: Use sparingly! Can make code unclear.

---

## Re-exporting with `pub use`

Make items available to external code:

**src/lib.rs:**
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");
        }
    }
}

// Re-export for convenience
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

**Usage in another crate:**
```rust
use restaurant::hosting;  // Direct access instead of restaurant::front_of_house::hosting

fn main() {
    hosting::add_to_waitlist();
}
```

---

## External Crates and Dependencies

### Adding Dependencies

**Cargo.toml:**
```toml
[dependencies]
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
```

### Using External Crates

```rust
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let number: u32 = rng.gen_range(1..=100);
    println!("Random number: {}", number);
}
```

### Standard Library

No need to add to Cargo.toml:

```rust
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", "value");
}
```

---

## The Module System in Practice

See the practical examples in the code directories for complete implementations.

---

## Best Practices

### 1. Module Organization

```rust
// ✅ Good: Clear hierarchy
src/
├── main.rs
├── models/
│   ├── mod.rs
│   ├── user.rs
│   └── product.rs
├── services/
│   ├── mod.rs
│   └── auth.rs
└── utils/
    ├── mod.rs
    └── validation.rs

// ❌ Bad: Everything in one file
src/
└── main.rs (2000 lines)
```

### 2. Privacy by Default

```rust
// ✅ Good: Only expose what's necessary
pub struct User {
    pub username: String,
    password_hash: String,  // Private
}

// ❌ Bad: Everything public
pub struct User {
    pub username: String,
    pub password_hash: String,  // Should be private!
}
```

### 3. Use Idiomatic Paths

```rust
// ✅ Good
use std::collections::HashMap;

let mut map = HashMap::new();

// ❌ Bad
use std::collections::HashMap::new;

let mut map = new();  // Unclear what 'new' creates
```

### 4. Group Related Functionality

```rust
// ✅ Good
mod user {
    pub struct User { /* ... */ }
    
    impl User {
        pub fn new() -> Self { /* ... */ }
        pub fn authenticate() -> bool { /* ... */ }
    }
    
    pub fn hash_password(password: &str) -> String { /* ... */ }
}

// ❌ Bad: Scattered functionality
mod user_struct { /* ... */ }
mod user_methods { /* ... */ }
mod password_utils { /* ... */ }
```

### 5. Meaningful Module Names

```rust
// ✅ Good
mod database;
mod authentication;
mod user_management;

// ❌ Bad
mod db;
mod auth;
mod um;
```

---

## Common Patterns

### Facade Pattern

Simplify complex module structures:

```rust
// Internal modules
mod internal {
    pub mod database {
        pub fn connect() { /* ... */ }
    }
    
    pub mod cache {
        pub fn init() { /* ... */ }
    }
}

// Public interface
pub use internal::database::connect;
pub use internal::cache::init;
```

### Prelude Pattern

Common in libraries:

```rust
// src/prelude.rs
pub use crate::user::User;
pub use crate::error::Error;
pub use crate::result::Result;

// Users can import everything:
// use my_crate::prelude::*;
```

### Builder Pattern with Modules

```rust
mod builder {
    pub struct Config {
        pub host: String,
        pub port: u16,
        timeout: u64,
    }
    
    pub struct ConfigBuilder {
        host: String,
        port: u16,
        timeout: u64,
    }
    
    impl ConfigBuilder {
        pub fn new() -> Self {
            ConfigBuilder {
                host: String::from("localhost"),
                port: 8080,
                timeout: 30,
            }
        }
        
        pub fn host(mut self, host: &str) -> Self {
            self.host = host.to_string();
            self
        }
        
        pub fn port(mut self, port: u16) -> Self {
            self.port = port;
            self
        }
        
        pub fn build(self) -> Config {
            Config {
                host: self.host,
                port: self.port,
                timeout: self.timeout,
            }
        }
    }
}
```

---

## Practical Examples

See the code examples in this directory:

1. **library_system** - Complete library management system
2. **calculator_modules** - Modular calculator
3. **user_management** - User authentication system
4. **file_organizer** - File processing with modules
5. **game_engine** - Simple game with module organization

---

## Exercises

### Exercise 1: Create a Banking System
Build a modular banking application with:
- Account module (creation, balance check)
- Transaction module (deposit, withdraw)
- Customer module (user management)

### Exercise 2: Restaurant Management
Create modules for:
- Menu (items, prices)
- Orders (taking, processing)
- Kitchen (cooking, serving)
- Billing (calculating, processing payment)

### Exercise 3: E-commerce Platform
Organize code into:
- Products module
- Shopping cart module
- Payment processing module
- User authentication module

---

## Summary

Today you learned:

✅ **Module Basics**: Creating and organizing modules  
✅ **Privacy**: Controlling access with `pub`  
✅ **File Organization**: Multi-file modules  
✅ **Module Paths**: Absolute, relative, `super`, `self`  
✅ **The `use` Keyword**: Bringing items into scope  
✅ **Re-exporting**: Using `pub use`  
✅ **External Crates**: Working with dependencies  
✅ **Best Practices**: Idiomatic module organization  

### Key Takeaways

1. **Everything is private by default** - Use `pub` intentionally
2. **Modules organize code** - Group related functionality
3. **Files mirror module structure** - Clear project organization
4. **`use` brings items into scope** - Shorter, clearer code
5. **External crates extend functionality** - Leverage the ecosystem

### Next Steps

Tomorrow, we'll explore:
- **Ownership and Borrowing**: Rust's unique memory management
- **Lifetimes**: Managing reference validity
- **Smart Pointers**: Advanced memory concepts

### Additional Resources

- [The Rust Book - Chapter 7](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
- [Rust by Example - Modules](https://doc.rust-lang.org/rust-by-example/mod.html)
- [API Guidelines - Module Organization](https://rust-lang.github.io/api-guidelines/)

---

Happy coding! 🦀

*Last Updated: November 23, 2025*
