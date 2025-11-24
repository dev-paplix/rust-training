# Boolean Logic Quick Reference

## 🎯 Quick Start

```bash
# Run any project
cd <project_name>
cargo run

# Run tests
cargo test

# Run with optimizations
cargo run --release
```

## 📋 Projects Overview

| Project | Focus | Key Concepts |
|---------|-------|--------------|
| **logic_operations** | Basic boolean operations | AND, OR, NOT, XOR |
| **truth_tables** | Logic truth tables | Visualization, combinations |
| **logic_gates** | Digital logic simulation | Gates, circuits |
| **boolean_calculator** | Practical application | Expression evaluation |

## 🔢 Boolean Operators

### Basic Operators

```rust
// AND - both must be true
let result = true && false;  // false

// OR - at least one must be true
let result = true || false;  // true

// NOT - inverts the value
let result = !true;  // false

// XOR - exactly one must be true (using !=)
let result = true != false;  // true
```

### Comparison Operators (Return bool)

```rust
let a = 5;
let b = 10;

a == b   // Equal: false
a != b   // Not equal: true
a < b    // Less than: true
a > b    // Greater than: false
a <= b   // Less or equal: true
a >= b   // Greater or equal: false
```

## 🎯 Operator Precedence

From highest to lowest:

1. **NOT** (`!`)
2. **Comparison** (`==`, `!=`, `<`, `>`, `<=`, `>=`)
3. **AND** (`&&`)
4. **OR** (`||`)

```rust
// Example
let result = !false && true || false;
// Evaluates as: ((!false) && true) || false
// = (true && true) || false
// = true || false
// = true
```

## ⚡ Short-Circuit Evaluation

```rust
// AND stops at first false
let result = false && expensive_function();  // expensive_function() NOT called

// OR stops at first true
let result = true || expensive_function();   // expensive_function() NOT called
```

## 💡 Common Patterns

### Guard Clauses

```rust
fn process(value: Option<i32>) {
    if value.is_none() || value.unwrap() < 0 {
        return;  // Early exit
    }
    // Process valid value
}
```

### Range Checking

```rust
let age = 25;
let is_adult = age >= 18 && age < 120;
let in_range = (1..=100).contains(&age);
```

### Multiple Conditions

```rust
let has_permission = is_logged_in && (is_admin || is_moderator);
let can_access = !is_banned && has_permission;
```

### Boolean Algebra

```rust
// De Morgan's Laws
!(a && b) == !a || !b
!(a || b) == !a && !b

// Double negation
!!a == a

// Absorption
a || (a && b) == a
a && (a || b) == a
```

## 📊 Truth Tables

### AND Truth Table

| A | B | A && B |
|---|---|--------|
| false | false | false |
| false | true | false |
| true | false | false |
| true | true | true |

### OR Truth Table

| A | B | A \|\| B |
|---|---|----------|
| false | false | false |
| false | true | true |
| true | false | true |
| true | true | true |

### XOR Truth Table (A != B)

| A | B | A != B |
|---|---|--------|
| false | false | false |
| false | true | true |
| true | false | true |
| true | true | false |

### NOT Truth Table

| A | !A |
|---|----|
| false | true |
| true | false |

## 🎓 Best Practices

### ✅ Do

```rust
// Use descriptive names
let is_valid = age >= 18 && has_license;

// Simplify complex conditions
let can_drive = is_adult() && has_valid_license();

// Use parentheses for clarity
let result = (a && b) || (c && d);

// Extract complex logic
fn is_eligible(user: &User) -> bool {
    user.age >= 18 && user.verified && !user.banned
}
```

### ❌ Don't

```rust
// Don't compare with true/false
if is_valid == true { }  // Bad
if is_valid { }          // Good

// Don't use ! with comparisons when unnecessary
if !(x > 5) { }          // Bad
if x <= 5 { }            // Good

// Don't create overly complex conditions
if a && b || c && d || e && f && !g { }  // Hard to read
```

## 🔍 Debugging Tips

### Print Boolean Values

```rust
let result = true && false;
println!("Result: {}", result);  // Result: false
```

### Use Debug Format

```rust
let conditions = (true, false, true);
println!("{:?}", conditions);  // (true, false, true)
```

### Test Individual Conditions

```rust
let a = true;
let b = false;
println!("a: {}, b: {}, a && b: {}", a, b, a && b);
```

## ⚠️ Common Pitfalls

### Assignment vs Comparison

```rust
// ❌ Wrong - assignment
if x = 5 { }  // Compile error

// ✅ Correct - comparison
if x == 5 { }
```

### Bitwise vs Logical

```rust
// Logical (short-circuit)
true && false   // Logical AND
true || false   // Logical OR

// Bitwise (always evaluate both)
true & false    // Bitwise AND (works on integers mainly)
true | false    // Bitwise OR
```

### NaN in Comparisons

```rust
let x = 0.0 / 0.0;  // NaN
println!("{}", x == x);  // false! NaN != NaN
println!("{}", x.is_nan());  // true
```

## 🧪 Testing Boolean Logic

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_and_operator() {
        assert_eq!(true && true, true);
        assert_eq!(true && false, false);
        assert_eq!(false && false, false);
    }
    
    #[test]
    fn test_or_operator() {
        assert!(true || false);
        assert!(!(false || false));
    }
    
    #[test]
    fn test_not_operator() {
        assert_eq!(!true, false);
        assert_eq!(!false, true);
    }
}
```

## 🎯 Quick Reference Card

| Operation | Symbol | Example | Result |
|-----------|--------|---------|--------|
| AND | `&&` | `true && false` | `false` |
| OR | `\|\|` | `true \|\| false` | `true` |
| NOT | `!` | `!true` | `false` |
| XOR | `!=` | `true != false` | `true` |
| Equal | `==` | `5 == 5` | `true` |
| Not Equal | `!=` | `5 != 3` | `true` |
| Less Than | `<` | `3 < 5` | `true` |
| Greater Than | `>` | `5 > 3` | `true` |
| Less or Equal | `<=` | `5 <= 5` | `true` |
| Greater or Equal | `>=` | `5 >= 3` | `true` |

## 💻 Running Examples

```bash
# Basic logic operations
cd logic_operations
cargo run

# Truth table generator
cd truth_tables
cargo run

# Logic gate simulator
cd logic_gates
cargo run

# Boolean expression calculator
cd boolean_calculator
cargo run
```

## 🔗 Related Topics

- **Control Flow**: If statements use boolean expressions
- **Pattern Matching**: Match guards with boolean conditions
- **Operators**: Comparison operators return booleans
- **Iterators**: filter(), any(), all() use boolean predicates
