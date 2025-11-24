# Control Flow Quick Reference

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
| **if_else_demo** | Conditional branching | if/else if/else, if expressions |
| **match_patterns** | Pattern matching | match, exhaustiveness, guards |
| **loop_examples** | Loop constructs | loop, while, for, labels |
| **control_flow_app** | Practical application | Combined flow control |

## 🔄 Control Flow Summary

### If Expressions

```rust
// If as expression
let number = if condition { 5 } else { 6 };

// Multiple conditions
if x < 0 {
    println!("negative");
} else if x > 0 {
    println!("positive");
} else {
    println!("zero");
}
```

### Match Expressions

```rust
match value {
    1 => println!("one"),
    2 | 3 => println!("two or three"),
    4..=10 => println!("four through ten"),
    _ => println!("anything else"),
}
```

### Loops

```rust
// Infinite loop
loop {
    if done { break; }
}

// While loop
while condition {
    // code
}

// For loop
for item in collection {
    // code
}

// Range iteration
for i in 0..10 {
    // code
}
```

## 💡 Common Patterns

### Early Return

```rust
fn process(value: i32) -> Result<i32, String> {
    if value < 0 {
        return Err("Negative value".to_string());
    }
    Ok(value * 2)
}
```

### Loop with Break Value

```rust
let result = loop {
    counter += 1;
    if counter == 10 {
        break counter * 2;
    }
};
```

### Labeled Loops

```rust
'outer: for x in 0..10 {
    for y in 0..10 {
        if x * y > 50 {
            break 'outer;
        }
    }
}
```

### Match Guards

```rust
match number {
    n if n < 0 => println!("negative"),
    0 => println!("zero"),
    n if n > 100 => println!("large"),
    _ => println!("normal"),
}
```

## ⚡ Performance Tips

| Pattern | Performance | Use When |
|---------|-------------|----------|
| `if/else` | Fast | 2-3 branches |
| `match` | Very fast | Many branches, patterns |
| `loop` + `break` | Fastest | Unknown iterations |
| `while` | Fast | Condition-based |
| `for` | Fast | Iterator-based |

## 🎯 Best Practices

### ✅ Do

```rust
// Use match for exhaustiveness
match option {
    Some(value) => process(value),
    None => handle_none(),
}

// Use ranges in for loops
for i in 0..10 {
    println!("{}", i);
}

// Use if let for single pattern
if let Some(x) = optional_value {
    println!("{}", x);
}
```

### ❌ Don't

```rust
// Don't use boolean match when if is clearer
match condition {
    true => do_something(),
    false => do_other(),
}
// Use: if condition { do_something() } else { do_other() }

// Don't forget break in loop
loop {
    // Infinite without break!
}

// Don't ignore exhaustiveness
match number {
    1 => {},
    2 => {},
    // Missing _ => {} can cause compile error
}
```

## 🔍 Troubleshooting

### Issue: "Pattern not covered"

```rust
// ❌ Not exhaustive
match x {
    0 => {},
    1 => {},
}

// ✅ Exhaustive
match x {
    0 => {},
    1 => {},
    _ => {},
}
```

### Issue: "If expression must have type"

```rust
// ❌ Type mismatch
let x = if condition { 5 } else { "text" };

// ✅ Same types
let x = if condition { 5 } else { 0 };
```

### Issue: "Unreachable pattern"

```rust
// ❌ Unreachable
match x {
    _ => {},
    1 => {},  // Never reached!
}

// ✅ Specific first
match x {
    1 => {},
    _ => {},
}
```

## 🎓 Learning Path

1. **Start with**: `if_else_demo` - Basic conditionals
2. **Then**: `match_patterns` - Pattern matching power
3. **Next**: `loop_examples` - Loop constructs
4. **Finally**: `control_flow_app` - Combined application

## 📚 Quick Reference

### If Expression

- Returns a value
- Both branches must return same type
- No parentheses around condition
- Braces always required

### Match Expression

- Must be exhaustive
- Evaluated top-to-bottom
- First matching arm executes
- Can match values, ranges, patterns

### Loop Types

- `loop`: Infinite, use `break` to exit
- `while`: Condition-based
- `for`: Iterator-based, most common

### Control Keywords

- `break`: Exit loop (optionally with value)
- `continue`: Skip to next iteration
- `return`: Exit function with value
- Labels: `'label:` for nested loops

## 💻 Running Examples

```bash
# If/else demonstration
cd if_else_demo
cargo run

# Pattern matching
cd match_patterns
cargo run

# Loop examples
cd loop_examples
cargo run

# Full application
cd control_flow_app
cargo run
```

## 🔗 Related Topics

- **Pattern Matching**: See day-2/06-pattern-matching-in-rust
- **Boolean Logic**: See day-2/03-boolean-logic
- **Loops**: See day-2/04-loops
- **Error Handling**: Result and Option matching
