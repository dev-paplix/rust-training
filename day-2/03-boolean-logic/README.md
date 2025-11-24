# Boolean Logic in Rust

## 📖 Introduction

Boolean logic is the foundation of decision-making in programming. In Rust, the `bool` type has two values: `true` and `false`. Understanding boolean operations is essential for control flow, conditionals, and logical reasoning in code.

### What You'll Learn

- **Boolean type** and values
- **Logical operators** (AND, OR, NOT, XOR)
- **Comparison operators** that return booleans
- **Short-circuit evaluation**
- **Truth tables** and logic laws
- **Practical applications** of boolean logic
- **Boolean algebra** and simplification

## 🔢 The Boolean Type

### Basic Boolean Values

```rust
fn main() {
    // Boolean type: bool
    let is_active: bool = true;
    let is_complete: bool = false;
    
    println!("Active: {}", is_active);      // Active: true
    println!("Complete: {}", is_complete);  // Complete: false
    
    // Boolean from expressions
    let is_positive = 5 > 0;
    let is_equal = 10 == 10;
    
    println!("Is positive: {}", is_positive);  // true
    println!("Is equal: {}", is_equal);        // true
}
```

**Key Points:**
- `bool` is a primitive type
- Only two values: `true` and `false`
- Takes 1 byte of memory
- Cannot be implicitly converted to/from integers

### Boolean in Memory

```rust
use std::mem::size_of;

fn main() {
    println!("Size of bool: {} byte", size_of::<bool>());
    
    // Comparison with other types
    println!("Size of i32: {} bytes", size_of::<i32>());
    println!("Size of char: {} bytes", size_of::<char>());
}
```

## ⚡ Logical Operators

### AND Operator (&&)

The AND operator returns `true` only if both operands are `true`:

```rust
fn main() {
    println!("true && true = {}", true && true);      // true
    println!("true && false = {}", true && false);    // false
    println!("false && true = {}", false && true);    // false
    println!("false && false = {}", false && false);  // false
    
    // Practical example
    let age = 25;
    let has_license = true;
    
    let can_drive = age >= 18 && has_license;
    println!("Can drive: {}", can_drive);  // true
    
    // Multiple conditions
    let is_weekend = true;
    let is_sunny = true;
    let has_time = true;
    
    let can_go_beach = is_weekend && is_sunny && has_time;
    println!("Can go to beach: {}", can_go_beach);  // true
}
```

**Use Cases:**
- All conditions must be satisfied
- Validation with multiple criteria
- Permission checking

### OR Operator (||)

The OR operator returns `true` if at least one operand is `true`:

```rust
fn main() {
    println!("true || true = {}", true || true);      // true
    println!("true || false = {}", true || false);    // true
    println!("false || true = {}", false || true);    // true
    println!("false || false = {}", false || false);  // false
    
    // Practical example
    let is_admin = false;
    let is_moderator = true;
    
    let has_privileges = is_admin || is_moderator;
    println!("Has privileges: {}", has_privileges);  // true
    
    // Alternative conditions
    let is_holiday = true;
    let is_vacation = false;
    
    let is_day_off = is_holiday || is_vacation;
    println!("Day off: {}", is_day_off);  // true
}
```

**Use Cases:**
- At least one condition must be satisfied
- Alternative requirements
- Fallback conditions

### NOT Operator (!)

The NOT operator inverts a boolean value:

```rust
fn main() {
    println!("!true = {}", !true);    // false
    println!("!false = {}", !false);  // true
    
    // Double negation
    println!("!!true = {}", !!true);  // true
    
    // Practical example
    let is_logged_in = true;
    let is_guest = !is_logged_in;
    println!("Is guest: {}", is_guest);  // false
    
    // Negating conditions
    let is_valid = true;
    if !is_valid {
        println!("Invalid!");
    } else {
        println!("Valid!");  // This prints
    }
}
```

**Use Cases:**
- Inverting conditions
- Checking for absence
- Negating boolean variables

### XOR (Exclusive OR)

Rust doesn't have a dedicated XOR operator for booleans, but you can use `!=`:

```rust
fn main() {
    // XOR using !=
    println!("true != true = {}", true != true);      // false
    println!("true != false = {}", true != false);    // true
    println!("false != true = {}", false != true);    // true
    println!("false != false = {}", false != false);  // false
    
    // Practical example
    let door1_open = true;
    let door2_open = false;
    
    // Exactly one door should be open
    let valid_state = door1_open != door2_open;
    println!("Valid state: {}", valid_state);  // true
    
    // Toggle pattern
    let light_on = true;
    let switch_pressed = true;
    
    let new_state = light_on != switch_pressed;
    println!("New light state: {}", new_state);  // false (turned off)
}
```

**Use Cases:**
- Exactly one condition must be true
- Toggle operations
- Parity checking

## 🔄 Comparison Operators

All comparison operators return `bool`:

```rust
fn main() {
    let a = 10;
    let b = 20;
    
    // Equality
    println!("{} == {} is {}", a, b, a == b);  // false
    println!("{} != {} is {}", a, b, a != b);  // true
    
    // Ordering
    println!("{} < {} is {}", a, b, a < b);    // true
    println!("{} > {} is {}", a, b, a > b);    // false
    println!("{} <= {} is {}", a, b, a <= b);  // true
    println!("{} >= {} is {}", a, b, a >= b);  // false
    
    // String comparison
    let name1 = "Alice";
    let name2 = "Bob";
    
    println!("{} == {} is {}", name1, name2, name1 == name2);  // false
    println!("{} < {} is {}", name1, name2, name1 < name2);    // true (alphabetical)
    
    // Boolean comparison
    println!("true == true is {}", true == true);    // true
    println!("true > false is {}", true > false);    // true (true is "greater")
}
```

### Chaining Comparisons

```rust
fn main() {
    let age = 25;
    
    // Multiple comparisons
    let is_adult = age >= 18 && age < 120;
    println!("Valid adult age: {}", is_adult);  // true
    
    // Range checking
    let score = 85;
    let is_passing = score >= 60 && score <= 100;
    println!("Passing score: {}", is_passing);  // true
    
    // Using ranges (cleaner)
    let in_range = (18..120).contains(&age);
    println!("In range: {}", in_range);  // true
}
```

## ⚡ Short-Circuit Evaluation

Rust uses short-circuit evaluation for `&&` and `||`:

```rust
fn expensive_check() -> bool {
    println!("Expensive check called!");
    true
}

fn main() {
    // AND short-circuit
    println!("\nAND with false first:");
    let result = false && expensive_check();
    println!("Result: {}", result);
    // expensive_check() is NOT called!
    
    println!("\nAND with true first:");
    let result = true && expensive_check();
    println!("Result: {}", result);
    // expensive_check() IS called
    
    // OR short-circuit
    println!("\nOR with true first:");
    let result = true || expensive_check();
    println!("Result: {}", result);
    // expensive_check() is NOT called!
    
    println!("\nOR with false first:");
    let result = false || expensive_check();
    println!("Result: {}", result);
    // expensive_check() IS called
}
```

**Practical Example:**

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    // Check b != 0 before dividing
    if b != 0 && a % b == 0 {
        Some(a / b)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", divide(10, 2));  // Some(5)
    println!("{:?}", divide(10, 0));  // None (no panic!)
}
```

## 📊 Truth Tables

### Complete Truth Tables

```rust
fn print_truth_table() {
    println!("A     | B     | A && B | A || B | !A    | A != B");
    println!("------|-------|--------|--------|-------|-------");
    
    for a in [false, true] {
        for b in [false, true] {
            println!(
                "{:<5} | {:<5} | {:<6} | {:<6} | {:<5} | {}",
                a, b, a && b, a || b, !a, a != b
            );
        }
    }
}

fn main() {
    print_truth_table();
}
```

**Output:**
```
A     | B     | A && B | A || B | !A    | A != B
------|-------|--------|--------|-------|-------
false | false | false  | false  | true  | false
false | true  | false  | true   | true  | true
true  | false | false  | true   | false | true
true  | true  | true   | true   | false | false
```

## 🎯 Boolean Algebra Laws

### Identity Laws

```rust
fn main() {
    let a = true;
    
    // AND with true
    assert_eq!(a && true, a);
    
    // OR with false
    assert_eq!(a || false, a);
    
    println!("✅ Identity laws verified");
}
```

### Domination Laws

```rust
fn main() {
    let a = true;
    
    // AND with false
    assert_eq!(a && false, false);
    
    // OR with true
    assert_eq!(a || true, true);
    
    println!("✅ Domination laws verified");
}
```

### Idempotent Laws

```rust
fn main() {
    let a = true;
    
    // AND with self
    assert_eq!(a && a, a);
    
    // OR with self
    assert_eq!(a || a, a);
    
    println!("✅ Idempotent laws verified");
}
```

### Complement Laws

```rust
fn main() {
    let a = true;
    
    // AND with complement
    assert_eq!(a && !a, false);
    
    // OR with complement
    assert_eq!(a || !a, true);
    
    println!("✅ Complement laws verified");
}
```

### De Morgan's Laws

```rust
fn main() {
    let a = true;
    let b = false;
    
    // !(A && B) = !A || !B
    assert_eq!(!(a && b), !a || !b);
    
    // !(A || B) = !A && !B
    assert_eq!(!(a || b), !a && !b);
    
    println!("✅ De Morgan's laws verified");
}
```

### Distributive Laws

```rust
fn main() {
    let a = true;
    let b = false;
    let c = true;
    
    // AND over OR: A && (B || C) = (A && B) || (A && C)
    assert_eq!(a && (b || c), (a && b) || (a && c));
    
    // OR over AND: A || (B && C) = (A || B) && (A || C)
    assert_eq!(a || (b && c), (a || b) && (a || c));
    
    println!("✅ Distributive laws verified");
}
```

## 🎯 Practical Applications

### Access Control

```rust
struct User {
    is_logged_in: bool,
    is_admin: bool,
    is_verified: bool,
    is_banned: bool,
}

impl User {
    fn can_post(&self) -> bool {
        self.is_logged_in && !self.is_banned && self.is_verified
    }
    
    fn can_delete(&self) -> bool {
        self.is_admin && !self.is_banned
    }
    
    fn can_moderate(&self) -> bool {
        (self.is_admin || self.has_mod_role()) && !self.is_banned
    }
    
    fn has_mod_role(&self) -> bool {
        // Placeholder
        false
    }
}

fn main() {
    let user = User {
        is_logged_in: true,
        is_admin: false,
        is_verified: true,
        is_banned: false,
    };
    
    println!("Can post: {}", user.can_post());      // true
    println!("Can delete: {}", user.can_delete());  // false
}
```

### Validation

```rust
fn validate_password(password: &str) -> (bool, Vec<String>) {
    let mut errors = Vec::new();
    
    let has_min_length = password.len() >= 8;
    let has_uppercase = password.chars().any(|c| c.is_uppercase());
    let has_lowercase = password.chars().any(|c| c.is_lowercase());
    let has_digit = password.chars().any(|c| c.is_numeric());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    
    if !has_min_length {
        errors.push("Must be at least 8 characters".to_string());
    }
    if !has_uppercase {
        errors.push("Must contain uppercase letter".to_string());
    }
    if !has_lowercase {
        errors.push("Must contain lowercase letter".to_string());
    }
    if !has_digit {
        errors.push("Must contain digit".to_string());
    }
    if !has_special {
        errors.push("Must contain special character".to_string());
    }
    
    let is_valid = has_min_length && has_uppercase && has_lowercase && has_digit && has_special;
    
    (is_valid, errors)
}

fn main() {
    let (valid, errors) = validate_password("Pass123!");
    println!("Valid: {}", valid);
    for error in errors {
        println!("  - {}", error);
    }
}
```

### State Machines

```rust
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn can_go(&self) -> bool {
        *self == TrafficLight::Green
    }
    
    fn should_slow(&self) -> bool {
        *self == TrafficLight::Yellow
    }
    
    fn must_stop(&self) -> bool {
        *self == TrafficLight::Red || *self == TrafficLight::Yellow
    }
}

fn main() {
    let light = TrafficLight::Yellow;
    
    println!("Can go: {}", light.can_go());          // false
    println!("Should slow: {}", light.should_slow()); // true
    println!("Must stop: {}", light.must_stop());    // true
}
```

## 🔍 Advanced Patterns

### Lazy Evaluation

```rust
fn expensive_operation() -> bool {
    println!("Expensive operation executed!");
    true
}

fn main() {
    let cache_hit = true;
    
    // Short-circuit prevents expensive call
    let result = cache_hit || expensive_operation();
    println!("Result: {}", result);
    // "Expensive operation executed!" is NOT printed
}
```

### Boolean Flags with Bitwise Operations

```rust
const READ: u8 = 0b0001;
const WRITE: u8 = 0b0010;
const EXECUTE: u8 = 0b0100;

fn main() {
    let permissions = READ | WRITE;  // 0b0011
    
    let can_read = (permissions & READ) != 0;
    let can_write = (permissions & WRITE) != 0;
    let can_execute = (permissions & EXECUTE) != 0;
    
    println!("Can read: {}", can_read);      // true
    println!("Can write: {}", can_write);    // true
    println!("Can execute: {}", can_execute); // false
}
```

## 🧪 Testing Boolean Logic

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_and_operator() {
        assert!(true && true);
        assert!(!(true && false));
        assert!(!(false && true));
        assert!(!(false && false));
    }
    
    #[test]
    fn test_or_operator() {
        assert!(true || true);
        assert!(true || false);
        assert!(false || true);
        assert!(!(false || false));
    }
    
    #[test]
    fn test_not_operator() {
        assert_eq!(!true, false);
        assert_eq!(!false, true);
        assert_eq!(!!true, true);
    }
    
    #[test]
    fn test_de_morgans() {
        let a = true;
        let b = false;
        
        assert_eq!(!(a && b), !a || !b);
        assert_eq!(!(a || b), !a && !b);
    }
}
```

## 💡 Best Practices

### 1. Use Descriptive Names

```rust
// ✅ Good
let is_valid_email = email.contains('@');
let has_permission = user.is_admin || user.is_moderator;

// ❌ Bad
let x = email.contains('@');
let flag = user.is_admin || user.is_moderator;
```

### 2. Avoid Comparing with true/false

```rust
// ✅ Good
if is_active {
    // ...
}

// ❌ Bad
if is_active == true {
    // ...
}
```

### 3. Simplify Double Negatives

```rust
// ✅ Good
if user.is_active {
    // ...
}

// ❌ Bad
if !user.is_inactive {
    // ...
}
```

### 4. Extract Complex Conditions

```rust
// ✅ Good
fn can_access_resource(user: &User) -> bool {
    user.is_authenticated() && 
    !user.is_banned() && 
    (user.is_owner() || user.has_permission())
}

if can_access_resource(&user) {
    // ...
}

// ❌ Bad
if user.is_authenticated() && !user.is_banned() && (user.is_owner() || user.has_permission()) {
    // ...
}
```

## 📚 Summary

Boolean logic in Rust:

- **Type**: `bool` with values `true` and `false`
- **Operators**: `&&` (AND), `||` (OR), `!` (NOT), `!=` (XOR)
- **Comparisons**: All return `bool` values
- **Short-circuit**: `&&` and `||` evaluate lazily
- **Laws**: Identity, domination, De Morgan's, etc.
- **Applications**: Validation, access control, state machines

### Key Takeaways

1. Booleans are fundamental to control flow
2. Short-circuit evaluation optimizes performance
3. Truth tables help understand complex logic
4. Boolean algebra laws enable simplification
5. Descriptive names improve code clarity
6. Extract complex conditions into functions

## 🔗 Next Steps

- Practice with truth tables
- Implement logic gates
- Build boolean calculators
- Study digital circuit design
- Explore bitwise operations for flags
