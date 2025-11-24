# Rust Operators - Quick Start

## Running the Examples

### 1. Arithmetic Calculator

```powershell
cd arithmetic_calculator
cargo run
```

**Demonstrates:**
- Arithmetic operators (+, -, *, /, %)
- Order of operations
- Integer vs floating-point arithmetic
- Overflow handling
- Type conversion

**Features:**
- Basic calculations
- Scientific operations
- Expression evaluation
- Precision handling

### 2. Comparison Tester

```powershell
cd comparison_tester
cargo run
```

**Demonstrates:**
- Comparison operators (==, !=, <, >, <=, >=)
- Equality vs identity
- Ordering with Ord trait
- Floating-point comparisons
- String comparisons

**Features:**
- Value comparison
- Range checking
- Sorting demonstrations
- Type comparison examples

### 3. Logical Operations

```powershell
cd logical_operations
cargo run
```

**Demonstrates:**
- Logical operators (&&, ||, !)
- Short-circuit evaluation
- Boolean expressions
- Truth tables
- Pattern matching with boolean logic

**Features:**
- Boolean algebra
- Conditional logic
- Access control simulation
- Complex decision trees

### 4. Bitwise Manipulator

```powershell
cd bitwise_manipulator
cargo run
```

**Demonstrates:**
- Bitwise operators (&, |, ^, !, <<, >>)
- Bit manipulation techniques
- Flags and masks
- Binary representation
- Performance optimization

**Features:**
- Bit setting/clearing
- Flag management
- Permissions system
- Binary visualization

## Running Tests

```powershell
# Arithmetic Calculator tests
cd arithmetic_calculator
cargo test

# All projects support testing
cargo test --all
```

## Key Operators Covered

### Operator Summary

| Operator Type | Operators | Example Project |
|--------------|-----------|-----------------|
| Arithmetic | `+`, `-`, `*`, `/`, `%` | Arithmetic Calculator |
| Comparison | `==`, `!=`, `<`, `>`, `<=`, `>=` | Comparison Tester |
| Logical | `&&`, `\|\|`, `!` | Logical Operations |
| Bitwise | `&`, `\|`, `^`, `!`, `<<`, `>>` | Bitwise Manipulator |
| Assignment | `=`, `+=`, `-=`, `*=`, `/=`, etc. | All projects |
| Range | `..`, `..=` | Comparison Tester |
| Type Cast | `as` | Arithmetic Calculator |

### Operator Precedence

```rust
// From highest to lowest precedence:
// 1. Unary: !, -
// 2. Multiplicative: *, /, %
// 3. Additive: +, -
// 4. Shift: <<, >>
// 5. Bitwise AND: &
// 6. Bitwise XOR: ^
// 7. Bitwise OR: |
// 8. Comparison: ==, !=, <, >, <=, >=
// 9. Logical AND: &&
// 10. Logical OR: ||
// 11. Range: .., ..=
// 12. Assignment: =, +=, -=, etc.

let result = 2 + 3 * 4;  // 14, not 20
let result = (2 + 3) * 4;  // 20, explicit grouping
```

## Common Patterns Demonstrated

### 1. Safe Arithmetic
```rust
// Checked operations
let result = a.checked_add(b);

// Saturating operations
let result = a.saturating_sub(b);

// Wrapping operations
let result = a.wrapping_mul(b);

// Overflowing operations
let (result, overflow) = a.overflowing_add(b);
```

### 2. Comparison Chains
```rust
// Range checking
if 0 <= value && value <= 100 {
    println!("Valid percentage");
}

// Using ranges
if (0..=100).contains(&value) {
    println!("Valid percentage");
}
```

### 3. Short-Circuit Evaluation
```rust
// && stops at first false
if is_valid() && expensive_check() {
    // expensive_check() not called if is_valid() is false
}

// || stops at first true
if has_cache() || load_from_database() {
    // load_from_database() not called if has_cache() is true
}
```

### 4. Bitwise Flags
```rust
const READ: u8 = 0b0001;
const WRITE: u8 = 0b0010;
const EXECUTE: u8 = 0b0100;

let permissions = READ | WRITE;  // Combine flags
let can_write = (permissions & WRITE) != 0;  // Check flag
let without_write = permissions & !WRITE;  // Clear flag
```

## Troubleshooting

### Integer Overflow in Debug Mode
```rust
// ❌ Panics in debug mode
let x: u8 = 255;
let y = x + 1;  // Overflow!

// ✅ Use checked operations
let y = x.checked_add(1);
match y {
    Some(val) => println!("Result: {}", val),
    None => println!("Overflow!"),
}
```

### Floating-Point Comparison
```rust
// ❌ Bad: Direct comparison
let a = 0.1 + 0.2;
let b = 0.3;
if a == b {  // May be false!
    println!("Equal");
}

// ✅ Good: Use epsilon comparison
const EPSILON: f64 = 1e-10;
if (a - b).abs() < EPSILON {
    println!("Equal");
}
```

### Division by Zero
```rust
// ❌ Panics
let result = 10 / 0;

// ✅ Check before dividing
if divisor != 0 {
    let result = dividend / divisor;
} else {
    println!("Cannot divide by zero");
}

// ✅ Use checked division
match dividend.checked_div(divisor) {
    Some(result) => println!("Result: {}", result),
    None => println!("Division error"),
}
```

### Bitwise vs Logical Operators
```rust
// Logical operators: short-circuit, work with bool
let result = true && false;  // false

// Bitwise operators: no short-circuit, work with integers
let result = 0b1010 & 0b1100;  // 0b1000

// ❌ Don't confuse them!
let x = 5;
let y = 10;
// if x & y { }  // Error: expected bool, found integer
if x > 0 && y > 0 { }  // Correct
```

## Next Steps

1. ✅ Run each example and experiment
2. ✅ Modify the code to add features
3. ✅ Complete the exercises in README.md
4. ✅ Try combining operators in expressions
5. ✅ Build your own calculator application

## Additional Examples

### Compound Assignment
```rust
let mut x = 10;
x += 5;   // x = x + 5
x -= 3;   // x = x - 3
x *= 2;   // x = x * 2
x /= 4;   // x = x / 4
x %= 3;   // x = x % 3
```

### Bit Shifting
```rust
let x = 5;  // 0b0101

let left = x << 1;   // 0b1010 = 10 (multiply by 2)
let right = x >> 1;  // 0b0010 = 2 (divide by 2)

// Fast multiplication/division by powers of 2
let times_8 = x << 3;  // x * 8
let div_4 = x >> 2;    // x / 4
```

### Chained Comparisons (Using match)
```rust
let score = 85;

let grade = match score {
    90..=100 => 'A',
    80..=89 => 'B',
    70..=79 => 'C',
    60..=69 => 'D',
    _ => 'F',
};
```

### Type Casting
```rust
let integer = 42;
let float = integer as f64;  // 42.0

let large_int: i64 = 1000;
let small_int = large_int as i32;  // Be careful with range!

// Character to integer
let ch = 'A';
let code = ch as u32;  // 65
```

Happy coding! 🦀
