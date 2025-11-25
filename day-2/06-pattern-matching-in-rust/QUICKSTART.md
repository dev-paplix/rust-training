# Pattern Matching in Rust - Quick Reference

## Pattern Matching Fundamentals

### Basic Match Expression
```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression,
}
```

### Common Pattern Types

| Pattern Type | Syntax | Example |
|-------------|--------|---------|
| Literal | `value` | `match x { 1 => ..., 2 => ... }` |
| Variable | `name` | `match x { n => println!("{}", n) }` |
| Wildcard | `_` | `match x { _ => println!("default") }` |
| Range | `start..=end` | `match age { 0..=17 => "child" }` |
| Multiple | `val1 \| val2` | `match x { 1 \| 2 => "one or two" }` |
| Struct | `Struct { field }` | `match p { Point { x, y } => ... }` |
| Tuple | `(a, b, c)` | `match t { (1, 2, 3) => ... }` |
| Enum | `Variant(data)` | `match opt { Some(x) => ... }` |
| Reference | `&pattern` | `match &value { &x => ... }` |
| Guard | `pattern if condition` | `match x { n if n > 0 => ... }` |

## Match vs If Let vs While Let

### Match - Multiple Patterns
```rust
match option {
    Some(value) => println!("Got: {}", value),
    None => println!("Nothing"),
}
```

### If Let - Single Pattern
```rust
if let Some(value) = option {
    println!("Got: {}", value);
}
```

### While Let - Loop Until Pattern Fails
```rust
while let Some(value) = iterator.next() {
    println!("{}", value);
}
```

## Destructuring Patterns

### Tuple Destructuring
```rust
let (x, y, z) = (1, 2, 3);
let (first, .., last) = (1, 2, 3, 4, 5);
```

### Struct Destructuring
```rust
let Point { x, y } = point;
let Point { x: x_coord, .. } = point; // Rename and ignore rest
```

### Enum Destructuring
```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(error) => println!("Error: {}", error),
}
```

### Nested Destructuring
```rust
match message {
    Message::Move { x: 0, y } => println!("x is zero, y: {}", y),
    Message::Move { x, y } => println!("x: {}, y: {}", x, y),
}
```

## Pattern Matching Features

### Guards (Conditional Matching)
```rust
match number {
    n if n < 0 => println!("Negative"),
    n if n > 0 => println!("Positive"),
    _ => println!("Zero"),
}
```

### @ Bindings
```rust
match age {
    n @ 0..=17 => println!("Child of age {}", n),
    n @ 18..=64 => println!("Adult of age {}", n),
    n @ 65.. => println!("Senior of age {}", n),
}
```

### Multiple Patterns (OR)
```rust
match character {
    'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
    _ => println!("Consonant"),
}
```

### Range Patterns
```rust
match score {
    0..=59 => "F",
    60..=69 => "D",
    70..=79 => "C",
    80..=89 => "B",
    90..=100 => "A",
    _ => "Invalid",
}
```

## Refutability

### Irrefutable Patterns (Always Match)
```rust
let x = 5;                    // Always works
let (a, b) = (1, 2);         // Always works
fn foo(x: i32) { }           // Parameter always matches
```

### Refutable Patterns (May Not Match)
```rust
if let Some(x) = option { }  // May be None
match value { pattern => ... } // May not match pattern
```

## Common Patterns

### Option Matching
```rust
match option {
    Some(value) => process(value),
    None => default(),
}

// Or with if let
if let Some(value) = option {
    process(value);
}
```

### Result Matching
```rust
match result {
    Ok(value) => println!("Success: {}", value),
    Err(e) => eprintln!("Error: {}", e),
}
```

### Ignoring Values
```rust
let (x, _, z) = (1, 2, 3);   // Ignore middle value
match point {
    Point { x, .. } => x,     // Ignore y
}
```

### Reference Patterns
```rust
match &value {
    &Point { x, y } => println!("{}, {}", x, y),
}

// Or dereference in pattern
match value {
    Point { x, y } => println!("{}, {}", x, y),
}
```

## Best Practices

1. **Exhaustiveness**: Always handle all possible cases
   - Use `_` for default cases
   - Compiler enforces exhaustive matching

2. **Guard Conditions**: Use for complex logic
   ```rust
   match value {
       x if is_valid(x) => process(x),
       _ => handle_invalid(),
   }
   ```

3. **Prefer Match Over If-Else Chains**
   ```rust
   // Good
   match status {
       Status::Active => ...,
       Status::Inactive => ...,
       Status::Pending => ...,
   }
   
   // Avoid
   if status == Status::Active { }
   else if status == Status::Inactive { }
   else if status == Status::Pending { }
   ```

4. **Use If Let for Single Patterns**
   ```rust
   // Good for single pattern
   if let Some(value) = option {
       process(value);
   }
   
   // Use match for multiple patterns
   match option {
       Some(value) => process(value),
       None => handle_none(),
   }
   ```

## Pattern Matching with Collections

### Slice Patterns
```rust
match slice {
    [] => println!("Empty"),
    [x] => println!("One element: {}", x),
    [first, .., last] => println!("First: {}, Last: {}", first, last),
    _ => println!("Other"),
}
```

### Array Patterns
```rust
match array {
    [1, 2, 3] => println!("Specific array"),
    [a, b, c] => println!("Any three elements"),
    _ => println!("Other"),
}
```

## Advanced Patterns

### Matching References
```rust
let reference = &value;
match reference {
    &Point { x, y } => println!("{}, {}", x, y),
}
```

### Matching Multiple Values
```rust
match (x, y) {
    (0, 0) => println!("Origin"),
    (0, _) => println!("On Y-axis"),
    (_, 0) => println!("On X-axis"),
    (_, _) => println!("Somewhere else"),
}
```

### Struct Update with Patterns
```rust
let Point { x, .. } = point1;
let point2 = Point { x, y: 10 };
```

## Common Errors & Solutions

| Error | Cause | Solution |
|-------|-------|----------|
| Non-exhaustive patterns | Missing case | Add `_` or handle all cases |
| Refutable pattern in let | Pattern may fail | Use `if let` or `match` |
| Move in pattern | Value moved | Use `ref` or `&` |
| Type mismatch | Wrong pattern type | Match pattern to value type |

## Testing Patterns

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_pattern_matching() {
        let value = Some(5);
        match value {
            Some(x) if x > 0 => assert!(true),
            _ => assert!(false),
        }
    }
}
```

## Quick Tips

- Match expressions are exhaustive - compiler checks all cases
- Use `_` to ignore values you don't need
- Guards (`if`) add extra conditions to patterns
- `@` bindings let you capture and test values
- Patterns can be nested arbitrarily deep
- Match is an expression - it returns a value
- Order matters - first matching pattern wins
