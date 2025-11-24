# Rust Operators

## Table of Contents

- [Introduction](#introduction)
- [Learning Objectives](#learning-objectives)
- [Arithmetic Operators](#arithmetic-operators)
- [Comparison Operators](#comparison-operators)
- [Logical Operators](#logical-operators)
- [Bitwise Operators](#bitwise-operators)
- [Assignment Operators](#assignment-operators)
- [Range Operators](#range-operators)
- [Type Cast Operator](#type-cast-operator)
- [Operator Precedence](#operator-precedence)
- [Operator Overloading](#operator-overloading)
- [Special Operators](#special-operators)
- [Overflow Handling](#overflow-handling)
- [Best Practices](#best-practices)
- [Practical Examples](#practical-examples)
- [Exercises](#exercises)
- [Summary](#summary)

---

## Introduction

Operators are symbols that tell the compiler to perform specific mathematical, logical, or bitwise operations. Rust provides a comprehensive set of operators similar to other programming languages, but with additional safety guarantees and precise semantics.

Understanding operators is fundamental to writing effective Rust code. This lesson covers all operator types, their behaviors, edge cases, and best practices for using them safely and efficiently.

**Key Features of Rust Operators:**
- **Type Safety**: Operations checked at compile time
- **No Implicit Conversions**: Explicit type casting required
- **Overflow Protection**: Different modes for debug/release
- **Trait-Based**: Operators implemented via traits

---

## Learning Objectives

By completing this lesson, you will be able to:

✅ Use arithmetic operators for mathematical operations  
✅ Compare values with comparison operators  
✅ Build logical expressions with boolean operators  
✅ Manipulate bits with bitwise operators  
✅ Apply assignment and compound assignment operators  
✅ Work with range operators for iteration  
✅ Perform safe type conversions  
✅ Understand and apply operator precedence  
✅ Handle arithmetic overflow correctly  
✅ Overload operators for custom types  

---

## Arithmetic Operators

Arithmetic operators perform mathematical calculations on numeric types.

### Basic Arithmetic

```rust
fn main() {
    let a = 10;
    let b = 3;
    
    // Addition
    let sum = a + b;  // 13
    println!("{} + {} = {}", a, b, sum);
    
    // Subtraction
    let difference = a - b;  // 7
    println!("{} - {} = {}", a, b, difference);
    
    // Multiplication
    let product = a * b;  // 30
    println!("{} * {} = {}", a, b, product);
    
    // Division (integer division)
    let quotient = a / b;  // 3
    println!("{} / {} = {}", a, b, quotient);
    
    // Remainder (modulo)
    let remainder = a % b;  // 1
    println!("{} % {} = {}", a, b, remainder);
}
```

### Integer vs Floating-Point

```rust
fn main() {
    // Integer division
    let int_result = 10 / 3;  // 3 (truncated)
    println!("Integer division: {}", int_result);
    
    // Floating-point division
    let float_result = 10.0 / 3.0;  // 3.3333...
    println!("Float division: {}", float_result);
    
    // Mixed types require casting
    let mixed = 10 as f64 / 3.0;  // 3.3333...
    println!("Mixed division: {}", mixed);
}
```

### Unary Operators

```rust
fn main() {
    let x = 5;
    
    // Negation
    let neg = -x;  // -5
    println!("Negation: {}", neg);
    
    // Note: There's no unary plus in Rust
    // let pos = +x;  // Error!
    
    // Floating-point negation
    let f = 3.14;
    let neg_f = -f;  // -3.14
    println!("Float negation: {}", neg_f);
}
```

### Checked Arithmetic

```rust
fn main() {
    let a: u8 = 200;
    let b: u8 = 100;
    
    // Checked operations return Option
    match a.checked_add(b) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow occurred!"),
    }
    
    // Other checked operations
    let checked_sub = a.checked_sub(b);  // Some(100)
    let checked_mul = a.checked_mul(b);  // None (overflow)
    let checked_div = a.checked_div(b);  // Some(2)
    
    // Checked division by zero
    match a.checked_div(0) {
        Some(result) => println!("Result: {}", result),
        None => println!("Division by zero!"),
    }
}
```

### Saturating Arithmetic

```rust
fn main() {
    let a: u8 = 250;
    let b: u8 = 10;
    
    // Saturating operations clamp at bounds
    let result = a.saturating_add(b);  // 255 (max u8)
    println!("Saturating add: {}", result);
    
    let result = a.saturating_sub(255);  // 0 (min u8)
    println!("Saturating sub: {}", result);
    
    let result = a.saturating_mul(2);  // 255 (max u8)
    println!("Saturating mul: {}", result);
}
```

### Wrapping Arithmetic

```rust
fn main() {
    let a: u8 = 255;
    
    // Wrapping operations wrap around
    let result = a.wrapping_add(1);  // 0
    println!("Wrapping add: {}", result);
    
    let result = a.wrapping_mul(2);  // 254
    println!("Wrapping mul: {}", result);
    
    let b: i8 = 127;
    let result = b.wrapping_add(1);  // -128
    println!("Wrapping signed: {}", result);
}
```

### Overflowing Arithmetic

```rust
fn main() {
    let a: u8 = 255;
    
    // Overflowing operations return (result, overflow_flag)
    let (result, overflow) = a.overflowing_add(1);
    println!("Result: {}, Overflow: {}", result, overflow);  // 0, true
    
    let (result, overflow) = a.overflowing_sub(10);
    println!("Result: {}, Overflow: {}", result, overflow);  // 245, false
}
```

---

## Comparison Operators

Comparison operators compare values and return boolean results.

### Equality Operators

```rust
fn main() {
    let a = 5;
    let b = 5;
    let c = 10;
    
    // Equal to
    println!("a == b: {}", a == b);  // true
    println!("a == c: {}", a == c);  // false
    
    // Not equal to
    println!("a != b: {}", a != b);  // false
    println!("a != c: {}", a != c);  // true
    
    // String comparison
    let s1 = String::from("hello");
    let s2 = String::from("hello");
    println!("s1 == s2: {}", s1 == s2);  // true
    
    // Reference comparison
    let r1 = &s1;
    let r2 = &s2;
    println!("r1 == r2: {}", r1 == r2);  // true (compares values)
}
```

### Ordering Operators

```rust
fn main() {
    let a = 10;
    let b = 20;
    
    // Less than
    println!("{} < {}: {}", a, b, a < b);  // true
    
    // Greater than
    println!("{} > {}: {}", a, b, a > b);  // false
    
    // Less than or equal to
    println!("{} <= {}: {}", a, b, a <= b);  // true
    
    // Greater than or equal to
    println!("{} >= {}: {}", a, b, a >= b);  // false
    
    // Chaining comparisons
    let x = 15;
    if a < x && x < b {
        println!("{} is between {} and {}", x, a, b);
    }
}
```

### Comparing Different Types

```rust
fn main() {
    // ❌ This doesn't work - different types
    // let result = 5 == 5.0;  // Error!
    
    // ✅ Cast to same type
    let result = 5 as f64 == 5.0;  // true
    println!("Result: {}", result);
    
    // Comparing characters
    let ch1 = 'a';
    let ch2 = 'z';
    println!("'a' < 'z': {}", ch1 < ch2);  // true
    
    // String slices
    let s1 = "apple";
    let s2 = "banana";
    println!("'apple' < 'banana': {}", s1 < s2);  // true (lexicographic)
}
```

### Floating-Point Comparisons

```rust
fn main() {
    let a = 0.1 + 0.2;
    let b = 0.3;
    
    // Direct comparison may fail due to precision
    println!("Direct: {}", a == b);  // Often false!
    
    // Use epsilon comparison
    const EPSILON: f64 = 1e-10;
    let close_enough = (a - b).abs() < EPSILON;
    println!("Epsilon: {}", close_enough);  // true
    
    // Special float values
    let nan = f64::NAN;
    println!("NaN == NaN: {}", nan == nan);  // false (always!)
    println!("NaN is NaN: {}", nan.is_nan());  // true
    
    let inf = f64::INFINITY;
    println!("Infinity > 0: {}", inf > 0.0);  // true
}
```

---

## Logical Operators

Logical operators work with boolean values.

### Boolean AND, OR, NOT

```rust
fn main() {
    let t = true;
    let f = false;
    
    // Logical AND
    println!("true && true: {}", t && t);    // true
    println!("true && false: {}", t && f);   // false
    println!("false && false: {}", f && f);  // false
    
    // Logical OR
    println!("true || true: {}", t || t);    // true
    println!("true || false: {}", t || f);   // true
    println!("false || false: {}", f || f);  // false
    
    // Logical NOT
    println!("!true: {}", !t);   // false
    println!("!false: {}", !f);  // true
}
```

### Short-Circuit Evaluation

```rust
fn expensive_function() -> bool {
    println!("Expensive function called!");
    true
}

fn main() {
    // AND: stops at first false
    println!("Testing AND...");
    if false && expensive_function() {
        // expensive_function() is NOT called
    }
    
    println!("\nTesting AND with true...");
    if true && expensive_function() {
        // expensive_function() IS called
    }
    
    // OR: stops at first true
    println!("\nTesting OR...");
    if true || expensive_function() {
        // expensive_function() is NOT called
    }
    
    println!("\nTesting OR with false...");
    if false || expensive_function() {
        // expensive_function() IS called
    }
}
```

### Complex Logical Expressions

```rust
fn main() {
    let age = 25;
    let has_license = true;
    let has_insurance = true;
    
    // Can drive if adult AND (has license OR has insurance)
    let can_drive = age >= 18 && (has_license || has_insurance);
    println!("Can drive: {}", can_drive);
    
    // De Morgan's Laws
    let a = true;
    let b = false;
    
    // !(a && b) == !a || !b
    println!("!(a && b): {}", !(a && b));
    println!("!a || !b: {}", !a || !b);
    
    // !(a || b) == !a && !b
    println!("!(a || b): {}", !(a || b));
    println!("!a && !b: {}", !a && !b);
}
```

---

## Bitwise Operators

Bitwise operators manipulate individual bits.

### Basic Bitwise Operations

```rust
fn main() {
    let a: u8 = 0b1100;  // 12 in decimal
    let b: u8 = 0b1010;  // 10 in decimal
    
    // Bitwise AND
    let and_result = a & b;  // 0b1000 = 8
    println!("{:04b} & {:04b} = {:04b} ({})", a, b, and_result, and_result);
    
    // Bitwise OR
    let or_result = a | b;  // 0b1110 = 14
    println!("{:04b} | {:04b} = {:04b} ({})", a, b, or_result, or_result);
    
    // Bitwise XOR
    let xor_result = a ^ b;  // 0b0110 = 6
    println!("{:04b} ^ {:04b} = {:04b} ({})", a, b, xor_result, xor_result);
    
    // Bitwise NOT
    let not_result = !a;  // 0b11110011 = 243 (for u8)
    println!("!{:08b} = {:08b} ({})", a, not_result, not_result);
}
```

### Bit Shifting

```rust
fn main() {
    let x: u8 = 5;  // 0b0101
    
    // Left shift (multiply by 2^n)
    let left = x << 1;  // 0b1010 = 10
    println!("{} << 1 = {}", x, left);
    
    let left2 = x << 2;  // 0b10100 = 20
    println!("{} << 2 = {}", x, left2);
    
    // Right shift (divide by 2^n)
    let right = x >> 1;  // 0b0010 = 2
    println!("{} >> 1 = {}", x, right);
    
    let right2 = x >> 2;  // 0b0001 = 1
    println!("{} >> 2 = {}", x, right2);
    
    // Signed right shift preserves sign bit
    let neg: i8 = -8;  // 0b11111000
    let shifted = neg >> 1;  // 0b11111100 = -4
    println!("{} >> 1 = {}", neg, shifted);
}
```

### Practical Bit Manipulation

```rust
fn main() {
    // Setting a bit
    let mut flags: u8 = 0b0000;
    let position = 2;
    flags |= 1 << position;  // Set bit 2
    println!("After setting bit {}: {:08b}", position, flags);
    
    // Clearing a bit
    flags &= !(1 << position);  // Clear bit 2
    println!("After clearing bit {}: {:08b}", position, flags);
    
    // Toggling a bit
    flags ^= 1 << position;  // Toggle bit 2
    println!("After toggling bit {}: {:08b}", position, flags);
    
    // Checking a bit
    let is_set = (flags & (1 << position)) != 0;
    println!("Bit {} is set: {}", position, is_set);
}
```

### Flags and Permissions

```rust
// Permission flags
const READ: u8 = 0b0001;     // 1
const WRITE: u8 = 0b0010;    // 2
const EXECUTE: u8 = 0b0100;  // 4
const DELETE: u8 = 0b1000;   // 8

fn main() {
    // Combine permissions
    let mut permissions = READ | WRITE;
    println!("Initial permissions: {:04b}", permissions);
    
    // Add execute permission
    permissions |= EXECUTE;
    println!("After adding execute: {:04b}", permissions);
    
    // Check for write permission
    let can_write = (permissions & WRITE) != 0;
    println!("Can write: {}", can_write);
    
    // Remove write permission
    permissions &= !WRITE;
    println!("After removing write: {:04b}", permissions);
    
    // Check multiple permissions
    let can_read_and_execute = (permissions & (READ | EXECUTE)) == (READ | EXECUTE);
    println!("Can read and execute: {}", can_read_and_execute);
}
```

---

## Assignment Operators

Assignment operators assign values to variables.

### Basic Assignment

```rust
fn main() {
    // Simple assignment
    let mut x = 5;
    println!("x = {}", x);
    
    // Reassignment (variable must be mutable)
    x = 10;
    println!("x = {}", x);
    
    // Multiple assignments
    let mut a = 1;
    let mut b = 2;
    let temp = a;
    a = b;
    b = temp;
    println!("a = {}, b = {}", a, b);
}
```

### Compound Assignment

```rust
fn main() {
    let mut x = 10;
    
    // Addition assignment
    x += 5;  // x = x + 5
    println!("After +=: {}", x);  // 15
    
    // Subtraction assignment
    x -= 3;  // x = x - 3
    println!("After -=: {}", x);  // 12
    
    // Multiplication assignment
    x *= 2;  // x = x * 2
    println!("After *=: {}", x);  // 24
    
    // Division assignment
    x /= 4;  // x = x / 4
    println!("After /=: {}", x);  // 6
    
    // Remainder assignment
    x %= 4;  // x = x % 4
    println!("After %=: {}", x);  // 2
}
```

### Bitwise Compound Assignment

```rust
fn main() {
    let mut x: u8 = 0b1100;
    
    // Bitwise AND assignment
    x &= 0b1010;  // x = x & 0b1010
    println!("After &=: {:04b}", x);  // 0b1000
    
    // Bitwise OR assignment
    x |= 0b0101;  // x = x | 0b0101
    println!("After |=: {:04b}", x);  // 0b1101
    
    // Bitwise XOR assignment
    x ^= 0b1111;  // x = x ^ 0b1111
    println!("After ^=: {:04b}", x);  // 0b0010
    
    // Left shift assignment
    x <<= 2;  // x = x << 2
    println!("After <<=: {:04b}", x);  // 0b1000
    
    // Right shift assignment
    x >>= 1;  // x = x >> 1
    println!("After >>=: {:04b}", x);  // 0b0100
}
```

---

## Range Operators

Range operators create range types for iteration and pattern matching.

### Range Types

```rust
fn main() {
    // Exclusive range (a..b) - includes a, excludes b
    let range1 = 0..5;
    println!("Exclusive range 0..5:");
    for i in range1 {
        print!("{} ", i);  // 0 1 2 3 4
    }
    println!();
    
    // Inclusive range (a..=b) - includes both a and b
    let range2 = 0..=5;
    println!("Inclusive range 0..=5:");
    for i in range2 {
        print!("{} ", i);  // 0 1 2 3 4 5
    }
    println!();
    
    // Unbounded ranges
    let range3 = ..5;   // from start to 5 (exclusive)
    let range4 = ..=5;  // from start to 5 (inclusive)
    let range5 = 2..;   // from 2 to end
    let range6 = ..;    // full range
    
    // Using ranges with slices
    let arr = [10, 20, 30, 40, 50];
    println!("arr[1..4]: {:?}", &arr[1..4]);    // [20, 30, 40]
    println!("arr[..3]: {:?}", &arr[..3]);      // [10, 20, 30]
    println!("arr[2..]: {:?}", &arr[2..]);      // [30, 40, 50]
    println!("arr[..]: {:?}", &arr[..]);        // [10, 20, 30, 40, 50]
}
```

### Range in Pattern Matching

```rust
fn main() {
    let score = 85;
    
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    };
    
    println!("Score {} is grade {}", score, grade);
    
    // Character ranges
    let ch = 'm';
    match ch {
        'a'..='z' => println!("Lowercase letter"),
        'A'..='Z' => println!("Uppercase letter"),
        '0'..='9' => println!("Digit"),
        _ => println!("Other character"),
    }
}
```

### Range Methods

```rust
fn main() {
    let range = 1..10;
    
    // Check if value is in range
    println!("Contains 5: {}", range.contains(&5));  // true
    println!("Contains 10: {}", range.contains(&10));  // false
    
    // Range properties
    let inclusive = 1..=10;
    println!("Start: {:?}", inclusive.start());
    println!("End: {:?}", inclusive.end());
    
    // Converting to iterator
    let sum: i32 = (1..=10).sum();
    println!("Sum 1 to 10: {}", sum);  // 55
}
```

---

## Type Cast Operator

The `as` operator performs explicit type conversions.

### Numeric Conversions

```rust
fn main() {
    // Integer to integer
    let x: i32 = 42;
    let y: i64 = x as i64;
    println!("i32 to i64: {}", y);
    
    // Integer to float
    let a: i32 = 10;
    let b: f64 = a as f64;
    println!("i32 to f64: {}", b);  // 10.0
    
    // Float to integer (truncates)
    let c: f64 = 3.14;
    let d: i32 = c as i32;
    println!("f64 to i32: {}", d);  // 3
    
    // Narrowing conversions (be careful!)
    let large: i32 = 1000;
    let small: i8 = large as i8;  // Wraps around!
    println!("i32 to i8: {}", small);  // -24 (overflow)
}
```

### Character and Integer

```rust
fn main() {
    // Character to integer
    let ch = 'A';
    let code = ch as u32;
    println!("'{}' as u32: {}", ch, code);  // 65
    
    // Integer to character
    let num = 65u32;
    let character = num as u8 as char;  // Cast through u8
    println!("{} as char: '{}'", num, character);  // 'A'
    
    // Unicode
    let emoji = '😀';
    let unicode = emoji as u32;
    println!("'{}' as u32: {}", emoji, unicode);  // 128512
}
```

### Pointer Casts

```rust
fn main() {
    let x = 42;
    let ptr = &x as *const i32;
    println!("Pointer address: {:p}", ptr);
    
    // Raw pointer to integer
    let addr = ptr as usize;
    println!("Address as usize: {:#x}", addr);
}
```

---

## Operator Precedence

Understanding operator precedence prevents logic errors.

### Precedence Table

```rust
fn main() {
    // Unary operators have highest precedence
    let x = -2 + 3;  // (-2) + 3 = 1
    println!("x = {}", x);
    
    // Multiplication before addition
    let y = 2 + 3 * 4;  // 2 + (3 * 4) = 14
    println!("y = {}", y);
    
    // Division before subtraction
    let z = 10 - 6 / 2;  // 10 - (6 / 2) = 7
    println!("z = {}", z);
    
    // Comparison before logical AND
    let a = 5 > 3 && 10 < 20;  // (5 > 3) && (10 < 20) = true
    println!("a = {}", a);
    
    // Logical AND before logical OR
    let b = false || true && false;  // false || (true && false) = false
    println!("b = {}", b);
    
    // Use parentheses for clarity
    let c = (2 + 3) * 4;  // Explicit grouping
    println!("c = {}", c);  // 20
}
```

### Precedence from Highest to Lowest

1. Unary: `!`, `-`
2. Multiplicative: `*`, `/`, `%`
3. Additive: `+`, `-`
4. Shift: `<<`, `>>`
5. Bitwise AND: `&`
6. Bitwise XOR: `^`
7. Bitwise OR: `|`
8. Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
9. Logical AND: `&&`
10. Logical OR: `||`
11. Range: `..`, `..=`
12. Assignment: `=`, `+=`, `-=`, etc.

---

## Operator Overloading

Rust allows custom types to implement operator behavior via traits.

### Implementing Add Trait

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;  // Uses our Add implementation
    println!("p1 + p2 = {:?}", p3);  // Point { x: 4, y: 6 }
}
```

### Other Operator Traits

```rust
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl Add for Vector2D {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector2D { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Vector2D {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Vector2D { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<f64> for Vector2D {
    type Output = Self;
    fn mul(self, scalar: f64) -> Self {
        Vector2D { x: self.x * scalar, y: self.y * scalar }
    }
}

fn main() {
    let v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    
    let sum = v1 + v2;
    let diff = v1 - v2;
    let scaled = v1 * 2.0;
    
    println!("Sum: {:?}", sum);
    println!("Diff: {:?}", diff);
    println!("Scaled: {:?}", scaled);
}
```

---

## Special Operators

### Dereference Operator

```rust
fn main() {
    let x = 5;
    let y = &x;  // Reference to x
    
    // Dereference to get value
    println!("*y = {}", *y);  // 5
    
    // Auto-dereferencing in method calls
    let s = String::from("hello");
    let len = s.len();  // Auto-dereferences if needed
}
```

### Reference Operator

```rust
fn main() {
    let x = 5;
    
    // Immutable reference
    let r1 = &x;
    println!("r1 = {}", r1);
    
    // Mutable reference
    let mut y = 10;
    let r2 = &mut y;
    *r2 += 5;
    println!("y = {}", y);  // 15
}
```

### Question Mark Operator

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_file_content(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;  // Returns error if fails
    let mut content = String::new();
    file.read_to_string(&mut content)?;  // Returns error if fails
    Ok(content)
}

fn main() {
    match read_file_content("test.txt") {
        Ok(content) => println!("Content: {}", content),
        Err(e) => println!("Error: {}", e),
    }
}
```

---

## Overflow Handling

Rust handles arithmetic overflow differently in debug and release builds.

### Debug vs Release Modes

```rust
fn main() {
    let mut x: u8 = 255;
    
    // In debug mode: panics
    // In release mode: wraps to 0
    // x += 1;  // Dangerous!
    
    // Always safe: use checked operations
    match x.checked_add(1) {
        Some(result) => println!("Result: {}", result),
        None => println!("Overflow detected!"),
    }
}
```

### Safe Overflow Handling Patterns

```rust
fn main() {
    let a: u32 = 4_000_000_000;
    let b: u32 = 1_000_000_000;
    
    // 1. Checked arithmetic
    if let Some(sum) = a.checked_add(b) {
        println!("Sum: {}", sum);
    } else {
        println!("Overflow!");
    }
    
    // 2. Saturating arithmetic
    let saturated = a.saturating_add(b);
    println!("Saturated: {}", saturated);  // u32::MAX
    
    // 3. Wrapping arithmetic
    let wrapped = a.wrapping_add(b);
    println!("Wrapped: {}", wrapped);
    
    // 4. Overflowing arithmetic
    let (result, overflow) = a.overflowing_add(b);
    println!("Result: {}, Overflow: {}", result, overflow);
}
```

---

## Best Practices

### 1. Use Explicit Type Conversions

```rust
// ✅ Good: Explicit conversion
let x: i32 = 42;
let y: f64 = x as f64;

// ❌ Bad: Rust doesn't do implicit conversion
// let y: f64 = x;  // Error!
```

### 2. Check for Overflow

```rust
// ✅ Good: Use checked operations
let result = a.checked_add(b);

// ❌ Risky: Unchecked in release mode
let result = a + b;
```

### 3. Use Parentheses for Clarity

```rust
// ✅ Good: Clear intent
let result = (a + b) * c;

// ❌ Ambiguous: Relies on precedence knowledge
let result = a + b * c;
```

### 4. Prefer Ranges Over Manual Loops

```rust
// ✅ Good: Idiomatic
for i in 0..10 {
    println!("{}", i);
}

// ❌ Verbose
let mut i = 0;
while i < 10 {
    println!("{}", i);
    i += 1;
}
```

### 5. Use Compound Assignment

```rust
// ✅ Good: Concise
x += 5;

// ❌ Verbose
x = x + 5;
```

---

## Practical Examples

See the code examples in this directory:

1. **arithmetic_calculator** - Comprehensive arithmetic operations
2. **comparison_tester** - Comparison and ordering demonstrations
3. **logical_operations** - Boolean logic and decision trees
4. **bitwise_manipulator** - Bit manipulation and flags

---

## Exercises

### Exercise 1: Calculator
Build a calculator that supports:
- Basic arithmetic operations
- Operator precedence
- Error handling (division by zero, overflow)
- Expression parsing

### Exercise 2: Grade Evaluator
Create a program that:
- Takes numerical scores
- Uses comparison operators to assign grades
- Calculates class statistics
- Finds outliers

### Exercise 3: Access Control System
Implement a permissions system:
- Use bitwise operators for flags
- Support READ, WRITE, EXECUTE, DELETE
- Check and modify permissions
- Display permissions in binary

### Exercise 4: Number Analyzer
Write a program that:
- Performs various arithmetic operations
- Demonstrates overflow handling
- Converts between number types
- Shows binary representations

### Exercise 5: Logic Simulator
Build a digital logic simulator:
- Implement AND, OR, NOT, XOR gates
- Create truth tables
- Combine gates for complex circuits
- Demonstrate short-circuit evaluation

---

## Summary

Today you learned:

✅ **Arithmetic Operators**: `+`, `-`, `*`, `/`, `%`  
✅ **Comparison Operators**: `==`, `!=`, `<`, `>`, `<=`, `>=`  
✅ **Logical Operators**: `&&`, `||`, `!`  
✅ **Bitwise Operators**: `&`, `|`, `^`, `!`, `<<`, `>>`  
✅ **Assignment Operators**: `=`, `+=`, `-=`, etc.  
✅ **Range Operators**: `..`, `..=`  
✅ **Type Casting**: `as` operator  
✅ **Operator Precedence**: Order of operations  
✅ **Overflow Handling**: Safe arithmetic  
✅ **Operator Overloading**: Custom implementations  

### Key Takeaways

1. **Type safety** prevents implicit conversions
2. **Overflow behavior** differs in debug/release
3. **Use checked operations** for safety
4. **Short-circuit evaluation** optimizes logic
5. **Bitwise operators** are powerful for flags

### Next Steps

Tomorrow, we'll explore:
- **Control Flow**: if, match, loops
- **Functions**: defining and calling
- **Error Handling**: Result and panic

### Additional Resources

- [The Rust Book - Operators](https://doc.rust-lang.org/book/)
- [Rust by Example - Operators](https://doc.rust-lang.org/rust-by-example/)
- [std::ops Module](https://doc.rust-lang.org/std/ops/)

---

Happy coding! 🦀

*Last Updated: November 24, 2025*
