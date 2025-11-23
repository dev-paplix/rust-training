# Day 1: Creating a Simple Application in Rust

## Table of Contents

- [Introduction](#introduction)
- [Learning Objectives](#learning-objectives)
- [Understanding Cargo](#understanding-cargo)
- [Creating Your First Rust Project](#creating-your-first-rust-project)
- [Anatomy of a Rust Project](#anatomy-of-a-rust-project)
- [Understanding main.rs](#understanding-mainrs)
- [Variables and Mutability](#variables-and-mutability)
- [Basic Data Types](#basic-data-types)
- [Functions in Rust](#functions-in-rust)
- [Control Flow](#control-flow)
- [User Input and Output](#user-input-and-output)
- [Building Practical Applications](#building-practical-applications)
- [Error Handling Basics](#error-handling-basics)
- [Comments and Documentation](#comments-and-documentation)
- [Best Practices](#best-practices)
- [Exercises](#exercises)
- [Summary](#summary)

---

## Introduction

Welcome to Day 1 of your Rust journey! Today, we'll dive into creating simple yet functional applications in Rust. You'll learn the fundamentals of the Rust programming language, understand how to structure your projects, and build several practical applications from scratch.

Rust is a systems programming language that emphasizes:
- **Safety**: Memory safety without garbage collection
- **Speed**: Zero-cost abstractions and performance comparable to C/C++
- **Concurrency**: Fearless concurrency through ownership and borrowing

By the end of this session, you'll have created multiple working Rust applications and understand the core concepts needed to build more complex programs.

---

## Learning Objectives

By completing this lesson, you will be able to:

✅ Create and manage Rust projects using Cargo  
✅ Understand the structure of a Rust program  
✅ Work with variables, data types, and mutability  
✅ Write and call functions effectively  
✅ Implement control flow (if/else, loops)  
✅ Handle user input and output  
✅ Apply basic error handling techniques  
✅ Follow Rust coding conventions and best practices  

---

## Understanding Cargo

**Cargo** is Rust's build system and package manager. It handles:
- Creating new projects
- Building your code
- Managing dependencies
- Running tests
- Generating documentation
- Publishing packages

### Essential Cargo Commands

```bash
# Create a new binary project
cargo new project_name

# Create a new library project
cargo new --lib library_name

# Build the project (debug mode)
cargo build

# Build with optimizations (release mode)
cargo build --release

# Build and run the project
cargo run

# Check code without building binary
cargo check

# Run tests
cargo test

# Update dependencies
cargo update

# Generate documentation
cargo doc --open
```

### Cargo vs rustc

While you can compile Rust code directly with `rustc`:
```bash
rustc main.rs
./main  # Linux/macOS
.\main.exe  # Windows
```

Cargo is preferred because it:
- Manages dependencies automatically
- Provides consistent project structure
- Handles complex build configurations
- Integrates with the Rust ecosystem

---

## Creating Your First Rust Project

Let's create our first project!

### Step 1: Create a New Project

```bash
cargo new hello_rust
cd hello_rust
```

### Step 2: Examine the Project Structure

```
hello_rust/
├── Cargo.toml          # Project manifest
├── src/
│   └── main.rs        # Main source file
└── .gitignore         # Git ignore file
```

### Step 3: Understanding Cargo.toml

`Cargo.toml` is the manifest file that contains metadata about your project:

```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
# External crates (libraries) go here
```

**Key Sections:**
- `[package]`: Project metadata
- `[dependencies]`: External libraries your project uses
- `edition`: Rust edition (2015, 2018, 2021, 2024)

### Step 4: Run Your First Program

```bash
cargo run
```

Expected output:
```
   Compiling hello_rust v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 1.23s
     Running `target/debug/hello_rust`
Hello, world!
```

---

## Anatomy of a Rust Project

### Directory Structure

After building, your project structure will look like:

```
hello_rust/
├── Cargo.toml
├── Cargo.lock          # Dependency lock file (auto-generated)
├── src/
│   └── main.rs
└── target/             # Build artifacts
    ├── debug/          # Debug build
    │   └── hello_rust  # Executable
    └── release/        # Release build (after cargo build --release)
```

### Important Files

**Cargo.lock**: 
- Auto-generated, tracks exact dependency versions
- Commit to version control for binary projects
- Usually not committed for library projects

**target/ directory**:
- Contains compiled artifacts
- Should be in `.gitignore` (Cargo does this automatically)
- Can be safely deleted (run `cargo clean`)

---

## Understanding main.rs

Let's dissect the default `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

### Breaking It Down

1. **`fn`**: Keyword to define a function
2. **`main`**: Special function - entry point of the program
3. **`()`**: Empty parameter list
4. **`{}`**: Function body
5. **`println!`**: Macro (note the `!`) that prints to console with newline
6. **`;`**: Statement terminator

### Key Concepts

**The main Function:**
- Every executable Rust program must have a `main` function
- Execution starts here
- Returns an exit code (0 for success by default)

**Macros vs Functions:**
- `println!` is a macro (notice the `!`)
- Macros are expanded at compile-time
- More powerful than functions for certain tasks

---

## Variables and Mutability

### Immutable by Default

In Rust, variables are **immutable** by default:

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    
    // x = 6;  // ❌ ERROR: cannot assign twice to immutable variable
}
```

### Mutable Variables

Use `mut` keyword for mutable variables:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;  // ✅ OK
    println!("The value of x is now: {}", x);
}
```

### Constants

Constants are always immutable and must have a type annotation:

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159265359;

fn main() {
    println!("Maximum points: {}", MAX_POINTS);
}
```

**Differences between `let` and `const`:**
- Constants use `const` keyword
- Constants must have type annotation
- Constants can be declared in any scope (including global)
- Constants must be set to constant expressions (no runtime computation)
- Convention: SCREAMING_SNAKE_CASE

### Shadowing

You can declare a new variable with the same name:

```rust
fn main() {
    let x = 5;
    let x = x + 1;      // shadows previous x
    let x = x * 2;      // shadows again
    
    println!("The value of x is: {}", x);  // 12
    
    // Can even change type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);
}
```

**Benefits of Shadowing:**
- Transform values without `mut`
- Can change type
- Useful in nested scopes

---

## Basic Data Types

Rust is a **statically typed** language - all types must be known at compile time.

### Scalar Types

Represent a single value.

#### Integer Types

**Signed integers** (can be negative):
```rust
let a: i8 = -128;           // -128 to 127
let b: i16 = -32768;        // -32,768 to 32,767
let c: i32 = -2147483648;   // -2³¹ to 2³¹-1 (default)
let d: i64 = 100;           // -2⁶³ to 2⁶³-1
let e: i128 = 100;          // -2¹²⁷ to 2¹²⁷-1
let f: isize = 100;         // Architecture dependent (32 or 64 bit)
```

**Unsigned integers** (positive only):
```rust
let a: u8 = 255;            // 0 to 255
let b: u16 = 65535;         // 0 to 65,535
let c: u32 = 100;           // 0 to 2³²-1
let d: u64 = 100;           // 0 to 2⁶⁴-1
let e: u128 = 100;          // 0 to 2¹²⁸-1
let f: usize = 100;         // Architecture dependent
```

**Number literals:**
```rust
let decimal = 98_222;       // Underscores for readability
let hex = 0xff;             // Hexadecimal
let octal = 0o77;           // Octal
let binary = 0b1111_0000;   // Binary
let byte = b'A';            // u8 only (byte literal)
```

#### Floating-Point Types

```rust
let x: f32 = 2.0;           // Single precision
let y: f64 = 3.14159;       // Double precision (default)
```

#### Boolean Type

```rust
let t: bool = true;
let f: bool = false;

// Often used in conditionals
let is_active = true;
if is_active {
    println!("Active!");
}
```

#### Character Type

```rust
let c: char = 'z';
let z: char = 'ℤ';
let heart_eyed_cat: char = '😻';

// char is 4 bytes - represents Unicode Scalar Value
println!("Character: {}", heart_eyed_cat);
```

### Compound Types

Group multiple values.

#### Tuples

Fixed-size, heterogeneous collections:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
    
    // Access by index
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    
    println!("{}, {}, {}", five_hundred, six_point_four, one);
}
```

**Unit type** (empty tuple):
```rust
let unit: () = ();  // Used when no value is returned
```

#### Arrays

Fixed-size, homogeneous collections:

```rust
fn main() {
    // Type annotation: [type; length]
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Initialize with same value
    let b = [3; 5];  // [3, 3, 3, 3, 3]
    
    // Access elements
    let first = a[0];
    let second = a[1];
    
    println!("First: {}, Second: {}", first, second);
    
    // Array length
    println!("Array length: {}", a.len());
}
```

**Arrays vs Vectors:**
- Arrays: Fixed size, stack-allocated
- Vectors: Dynamic size, heap-allocated (covered later)

---

## Functions in Rust

### Function Definition

```rust
fn greet() {
    println!("Hello from a function!");
}

fn main() {
    greet();
}
```

### Parameters

```rust
fn greet_user(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    greet_user("Alice");
    greet_user("Bob");
}
```

**Multiple parameters:**
```rust
fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {}{}", value, unit);
}

fn main() {
    print_labeled_measurement(5, 'h');
    print_labeled_measurement(10, 'm');
}
```

### Return Values

```rust
fn five() -> i32 {
    5  // No semicolon - this is an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1  // Expression, returns value
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
    
    let y = plus_one(5);
    println!("The value of y is: {}", y);
}
```

**Return statement:**
```rust
fn early_return(x: i32) -> i32 {
    if x < 0 {
        return 0;  // Early return with explicit return keyword
    }
    x * 2  // Implicit return
}
```

### Statements vs Expressions

**Statement**: Performs an action, no return value
```rust
let y = 6;  // Statement
```

**Expression**: Evaluates to a value
```rust
5 + 6       // Expression
{
    let x = 3;
    x + 1   // Expression (no semicolon)
}
```

**Key difference:**
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1  // Expression - returns 4
    };
    println!("y = {}", y);  // y = 4
    
    let z = {
        let x = 3;
        x + 1;  // Statement (semicolon!) - returns ()
    };
    // println!("z = {}", z);  // ERROR: () cannot be formatted
}
```

---

## Control Flow

### if Expressions

```rust
fn main() {
    let number = 7;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

**Multiple conditions with else if:**
```rust
fn main() {
    let number = 6;
    
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

**if as an expression:**
```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    
    println!("The value of number is: {}", number);
}
```

### Loops

#### loop - Infinite Loop

```rust
fn main() {
    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("Counter: {}", counter);
        
        if counter == 10 {
            break;
        }
    }
}
```

**Returning values from loop:**
```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);  // 20
}
```

**Loop labels:**
```rust
fn main() {
    let mut count = 0;
    'outer: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
```

#### while - Conditional Loop

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
}
```

#### for - Iterating Over Collections

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {}", element);
    }
}
```

**Range iteration:**
```rust
fn main() {
    // Exclusive range
    for number in 1..4 {
        println!("{}", number);  // 1, 2, 3
    }
    
    // Inclusive range
    for number in 1..=4 {
        println!("{}", number);  // 1, 2, 3, 4
    }
    
    // Reverse
    for number in (1..4).rev() {
        println!("{}", number);  // 3, 2, 1
    }
}
```

---

## User Input and Output

### Printing to Console

#### println! Macro

```rust
fn main() {
    // Basic printing
    println!("Hello, world!");
    
    // With placeholders
    let name = "Alice";
    let age = 30;
    println!("Name: {}, Age: {}", name, age);
    
    // Positional arguments
    println!("{0} is {1} years old. {0} likes programming.", name, age);
    
    // Named arguments
    println!("{name} is {age} years old", name = "Bob", age = 25);
    
    // Debug formatting
    let point = (3, 5);
    println!("Point: {:?}", point);
    
    // Pretty debug
    println!("Point: {:#?}", point);
}
```

#### print! Macro (No Newline)

```rust
fn main() {
    print!("This is on the same line. ");
    print!("This too!\n");
}
```

#### eprintln! (Error Output)

```rust
fn main() {
    eprintln!("This goes to stderr");
    println!("This goes to stdout");
}
```

### Reading User Input

```rust
use std::io;

fn main() {
    println!("Please enter your name:");
    
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    println!("Hello, {}!", input.trim());
}
```

**Breaking it down:**
1. `use std::io;` - Imports the I/O library
2. `String::new()` - Creates a new, empty String
3. `io::stdin()` - Gets handle to standard input
4. `read_line(&mut input)` - Reads a line into `input`
5. `.expect()` - Handles potential errors
6. `input.trim()` - Removes whitespace/newline

---

## Building Practical Applications

Let's build some real applications! See the code examples in this directory.

### Application 1: Temperature Converter

Converts between Celsius and Fahrenheit.

**File:** `temperature_converter.rs`

### Application 2: Simple Calculator

Performs basic arithmetic operations.

**File:** `calculator.rs`

### Application 3: Number Guessing Game

Interactive game where users guess a random number.

**File:** `guessing_game.rs`

### Application 4: Grade Calculator

Calculates letter grades from numeric scores.

**File:** `grade_calculator.rs`

### Application 5: BMI Calculator

Calculates Body Mass Index.

**File:** `bmi_calculator.rs`

### Application 6: Todo List

Simple command-line todo list manager.

**File:** `todo_list.rs`

---

## Error Handling Basics

### Using expect()

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");  // Program panics if error occurs
}
```

### Using match for Result

```rust
use std::io;

fn main() {
    let mut input = String::new();
    
    match io::stdin().read_line(&mut input) {
        Ok(_) => println!("Successfully read: {}", input.trim()),
        Err(e) => println!("Error reading input: {}", e),
    }
}
```

### Parsing with Error Handling

```rust
fn main() {
    let number_str = "42";
    
    match number_str.parse::<i32>() {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Failed to parse: {}", e),
    }
}
```

---

## Comments and Documentation

### Regular Comments

```rust
// This is a single-line comment

/*
 * This is a multi-line comment
 * It can span multiple lines
 */

fn main() {
    // Calculate the answer
    let answer = 42;  // Inline comment
    println!("The answer is {}", answer);
}
```

### Documentation Comments

```rust
/// Calculates the square of a number.
///
/// # Examples
///
/// ```
/// let result = square(5);
/// assert_eq!(result, 25);
/// ```
fn square(x: i32) -> i32 {
    x * x
}

//! This is a crate-level documentation comment
//! Used at the top of lib.rs or main.rs
```

---

## Best Practices

### Naming Conventions

```rust
// Constants and statics: SCREAMING_SNAKE_CASE
const MAX_POINTS: u32 = 100_000;

// Functions and variables: snake_case
fn calculate_total(item_price: f64, tax_rate: f64) -> f64 {
    item_price * (1.0 + tax_rate)
}

// Types and traits: PascalCase
struct UserAccount {
    user_name: String,
    account_balance: f64,
}
```

### Code Organization

```rust
// Imports at the top
use std::io;

// Constants
const VERSION: &str = "1.0.0";

// Main function
fn main() {
    // Code here
}

// Helper functions below
fn helper_function() {
    // Code here
}
```

### Error Messages

```rust
// ❌ Bad: Generic error message
.expect("Error");

// ✅ Good: Specific error message
.expect("Failed to read user input from stdin");
```

### Type Inference vs Explicit Types

```rust
// Type inference (preferred when obvious)
let x = 5;
let name = "Alice";

// Explicit types (when needed for clarity)
let mut counter: u32 = 0;
let price: f64 = 19.99;
```

---

## Exercises

### Exercise 1: Age Calculator
Create a program that:
1. Asks for the user's birth year
2. Calculates their age in 2025
3. Displays the result

### Exercise 2: Area Calculator
Create a program that:
1. Asks for shape type (circle, rectangle, triangle)
2. Requests appropriate dimensions
3. Calculates and displays the area

### Exercise 3: Palindrome Checker
Create a program that:
1. Accepts a word from the user
2. Checks if it's a palindrome
3. Displays the result

### Exercise 4: Fibonacci Sequence
Create a program that:
1. Asks how many Fibonacci numbers to generate
2. Generates and displays the sequence

### Exercise 5: Unit Converter
Create a program that converts between:
- Kilometers and miles
- Kilograms and pounds
- Celsius and Fahrenheit

---

## Summary

Today you learned:

✅ **Cargo Basics**: Creating and managing Rust projects  
✅ **Project Structure**: Understanding Cargo.toml and src/ directory  
✅ **Variables**: Immutability, mutability, constants, and shadowing  
✅ **Data Types**: Scalars (integers, floats, bool, char) and compounds (tuples, arrays)  
✅ **Functions**: Declaration, parameters, return values, expressions vs statements  
✅ **Control Flow**: if/else, loop, while, for  
✅ **I/O**: Reading input and printing output  
✅ **Error Handling**: Using expect() and match  
✅ **Best Practices**: Naming conventions and code organization  

### Key Takeaways

1. **Immutability by default** makes Rust code safer and more predictable
2. **Type safety** catches errors at compile-time
3. **Expressions vs statements** is a fundamental concept in Rust
4. **Cargo** is your best friend for project management
5. **Error handling** is built into the type system

### Next Steps

Tomorrow, we'll explore:
- **Ownership and Borrowing**: Rust's most unique feature
- **Modules**: Organizing code across files
- **Advanced data structures**: Vectors, HashMaps, etc.

### Additional Resources

- [The Rust Book - Chapter 3](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings Exercises](https://github.com/rust-lang/rustlings)

---

## Practice Projects

Try building these on your own:

1. **Password Strength Checker**: Evaluate password strength
2. **Tip Calculator**: Calculate tips and split bills
3. **Roman Numeral Converter**: Convert between Arabic and Roman numerals
4. **Word Counter**: Count words, characters, and lines in text
5. **Rock Paper Scissors**: Play against the computer

Happy coding! 🦀

---

*Last Updated: November 23, 2025*
