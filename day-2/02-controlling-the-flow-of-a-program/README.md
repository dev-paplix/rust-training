# Controlling the Flow of a Program

## 📖 Introduction

Control flow is fundamental to programming, allowing programs to make decisions and repeat actions. Rust provides powerful control flow constructs that are both expressive and safe.

### What You'll Learn

- **If expressions** for conditional execution
- **Match expressions** for pattern-based control
- **Loop constructs** (loop, while, for)
- **Break and continue** statements
- **Labeled loops** for nested control
- **Control flow best practices**

## 🎯 If Expressions

### Basic If/Else

In Rust, `if` is an expression that returns a value:

```rust
fn main() {
    let number = 6;
    
    // Simple if
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // If expression (returns value)
    let result = if number % 2 == 0 {
        "even"
    } else {
        "odd"
    };
    
    println!("Number is {}", result);
}
```

**Key Points:**
- No parentheses around condition
- Braces are always required
- Condition must be a `bool`
- Can return values

### Multiple Conditions

```rust
fn categorize_number(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n == 0 {
        "zero"
    } else if n < 10 {
        "small positive"
    } else if n < 100 {
        "medium positive"
    } else {
        "large positive"
    }
}

fn main() {
    println!("-5 is {}", categorize_number(-5));
    println!("0 is {}", categorize_number(0));
    println!("7 is {}", categorize_number(7));
    println!("150 is {}", categorize_number(150));
}
```

### If Let Pattern

For single pattern matching:

```rust
fn main() {
    let favorite_color: Option<&str> = Some("blue");
    
    // Using match
    match favorite_color {
        Some(color) => println!("Favorite color: {}", color),
        None => {},
    }
    
    // Using if let (cleaner for single pattern)
    if let Some(color) = favorite_color {
        println!("Favorite color: {}", color);
    }
    
    // With else
    let age: Result<u8, _> = "34".parse();
    
    if let Ok(age) = age {
        println!("Age: {}", age);
    } else {
        println!("Could not parse age");
    }
}
```

## 🎲 Match Expressions

### Basic Match

Match is exhaustive - all possibilities must be covered:

```rust
fn describe_number(n: i32) -> String {
    match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 | 3 | 5 | 7 => String::from("small prime"),
        n if n < 0 => format!("negative: {}", n),
        n if n % 2 == 0 => format!("even: {}", n),
        _ => format!("odd: {}", n),
    }
}

fn main() {
    for n in [-1, 0, 1, 2, 4, 7, 9] {
        println!("{} -> {}", n, describe_number(n));
    }
}
```

### Pattern Matching

```rust
enum Color {
    Red,
    Green,
    Blue,
    RGB(u8, u8, u8),
}

fn describe_color(color: Color) {
    match color {
        Color::Red => println!("It's red!"),
        Color::Green => println!("It's green!"),
        Color::Blue => println!("It's blue!"),
        Color::RGB(r, g, b) => {
            println!("RGB({}, {}, {})", r, g, b);
        }
    }
}

fn main() {
    describe_color(Color::Red);
    describe_color(Color::RGB(100, 200, 50));
}
```

### Match Guards

Add conditions to patterns:

```rust
fn categorize(number: i32, is_special: bool) {
    match number {
        n if is_special => println!("{} is special!", n),
        n if n < 0 => println!("Negative: {}", n),
        0 => println!("Zero"),
        n if n % 2 == 0 => println!("Even: {}", n),
        n => println!("Odd: {}", n),
    }
}

fn main() {
    categorize(5, true);   // 5 is special!
    categorize(5, false);  // Odd: 5
    categorize(4, false);  // Even: 4
}
```

### Matching Ranges

```rust
fn grade_letter(score: u8) -> char {
    match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => panic!("Invalid score"),
    }
}

fn main() {
    println!("95 -> {}", grade_letter(95));
    println!("82 -> {}", grade_letter(82));
    println!("55 -> {}", grade_letter(55));
}
```

### Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn describe_point(point: Point) {
    match point {
        Point { x: 0, y: 0 } => println!("Origin"),
        Point { x: 0, y } => println!("On Y-axis at y={}", y),
        Point { x, y: 0 } => println!("On X-axis at x={}", x),
        Point { x, y } if x == y => println!("On diagonal at ({}, {})", x, y),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}

fn main() {
    describe_point(Point { x: 0, y: 0 });
    describe_point(Point { x: 0, y: 5 });
    describe_point(Point { x: 3, y: 3 });
}
```

## 🔄 Loop Constructs

### Loop - Infinite Loop

Use `loop` for infinite loops:

```rust
fn main() {
    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("Count: {}", counter);
        
        if counter >= 5 {
            break;
        }
    }
    
    println!("Final count: {}", counter);
}
```

### Loop with Return Value

```rust
fn find_first_even(numbers: &[i32]) -> Option<i32> {
    let mut i = 0;
    
    let result = loop {
        if i >= numbers.len() {
            break None;
        }
        
        if numbers[i] % 2 == 0 {
            break Some(numbers[i]);
        }
        
        i += 1;
    };
    
    result
}

fn main() {
    let nums = [1, 3, 7, 8, 9];
    match find_first_even(&nums) {
        Some(n) => println!("First even: {}", n),
        None => println!("No even numbers"),
    }
}
```

### While Loop

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!");
}
```

### For Loop - Iterator

The most common loop in Rust:

```rust
fn main() {
    // Iterate over array
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("Value: {}", element);
    }
    
    // Range
    for number in 1..=5 {
        println!("{}", number);
    }
    
    // Reverse
    for number in (1..=5).rev() {
        println!("{}", number);
    }
    
    // With index
    for (index, value) in a.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
}
```

### Labeled Loops

For nested loops:

```rust
fn main() {
    let mut count = 0;
    
    'outer: for x in 0..10 {
        for y in 0..10 {
            if x * y > 50 {
                println!("Breaking at x={}, y={}", x, y);
                break 'outer;
            }
            count += 1;
        }
    }
    
    println!("Iterations: {}", count);
}
```

### Continue Statement

```rust
fn main() {
    for number in 1..=10 {
        if number % 2 == 0 {
            continue;
        }
        println!("{} is odd", number);
    }
}
```

## 🎯 Practical Examples

### Menu System

```rust
use std::io::{self, Write};

fn main() {
    loop {
        println!("\n=== Main Menu ===");
        println!("1. Option A");
        println!("2. Option B");
        println!("3. Exit");
        print!("Choice: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => println!("Selected Option A"),
            "2" => println!("Selected Option B"),
            "3" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
```

### State Machine

```rust
#[derive(Debug, PartialEq)]
enum State {
    Idle,
    Running,
    Paused,
    Stopped,
}

fn next_state(current: State, input: &str) -> State {
    match (current, input) {
        (State::Idle, "start") => State::Running,
        (State::Running, "pause") => State::Paused,
        (State::Paused, "resume") => State::Running,
        (State::Running, "stop") | (State::Paused, "stop") => State::Stopped,
        (State::Stopped, "reset") => State::Idle,
        (state, _) => state,
    }
}

fn main() {
    let mut state = State::Idle;
    
    let commands = ["start", "pause", "resume", "stop", "reset"];
    
    for cmd in commands {
        state = next_state(state, cmd);
        println!("{} -> {:?}", cmd, state);
    }
}
```

### Validation

```rust
fn validate_age(age: i32) -> Result<u8, String> {
    if age < 0 {
        return Err("Age cannot be negative".to_string());
    }
    
    if age > 150 {
        return Err("Age is unrealistic".to_string());
    }
    
    Ok(age as u8)
}

fn main() {
    let test_ages = [-5, 25, 200];
    
    for age in test_ages {
        match validate_age(age) {
            Ok(valid_age) => println!("{} is valid", valid_age),
            Err(e) => println!("Error for {}: {}", age, e),
        }
    }
}
```

## 📊 Control Flow Comparison

| Construct | Use When | Returns Value | Exhaustive |
|-----------|----------|---------------|------------|
| `if/else` | Simple conditions | Yes | No |
| `if let` | Single pattern | No | No |
| `match` | Multiple patterns | Yes | Yes |
| `loop` | Unknown iterations | Yes (with break) | N/A |
| `while` | Condition-based | No | N/A |
| `for` | Known iterations | No | N/A |

## ⚡ Performance Considerations

### Branch Prediction

```rust
// Modern CPUs predict branches
// Consistent patterns are faster

// ✅ Predictable
for i in 0..1000 {
    if i % 2 == 0 {
        // Even branch
    } else {
        // Odd branch
    }
}

// ❌ Less predictable (random data)
for random_value in random_data {
    if random_value > threshold {
        // Unpredictable
    }
}
```

### Match vs If Chain

```rust
// Match is often faster for many branches
// Compiler can optimize to jump table

// ✅ Match for many branches
match value {
    1 => action1(),
    2 => action2(),
    3 => action3(),
    // ... many cases
    _ => default(),
}

// Use if/else for 2-3 conditions
if condition1 {
    action1()
} else if condition2 {
    action2()
} else {
    action3()
}
```

## 🎓 Best Practices

### 1. Prefer Match for Exhaustiveness

```rust
// ✅ Compiler ensures all cases handled
enum Status {
    Success,
    Error,
    Pending,
}

fn handle(status: Status) {
    match status {
        Status::Success => println!("OK"),
        Status::Error => println!("Failed"),
        Status::Pending => println!("Waiting"),
        // Compiler error if case missing
    }
}
```

### 2. Use Early Returns

```rust
// ✅ Clear and reduces nesting
fn process(value: Option<i32>) -> i32 {
    let Some(v) = value else {
        return 0;
    };
    
    if v < 0 {
        return -1;
    }
    
    v * 2
}

// ❌ Deep nesting
fn process_nested(value: Option<i32>) -> i32 {
    if let Some(v) = value {
        if v >= 0 {
            v * 2
        } else {
            -1
        }
    } else {
        0
    }
}
```

### 3. Choose the Right Loop

```rust
// ✅ For with iterator
for item in collection {
    process(item);
}

// ❌ Manual indexing (unless needed)
let mut i = 0;
while i < collection.len() {
    process(collection[i]);
    i += 1;
}
```

### 4. Use If Let for Single Patterns

```rust
// ✅ Concise
if let Some(x) = optional {
    println!("{}", x);
}

// ❌ Verbose
match optional {
    Some(x) => println!("{}", x),
    None => {},
}
```

## 🔍 Common Pitfalls

### 1. If Expression Type Mismatch

```rust
// ❌ Compile error
let x = if condition {
    5
} else {
    "five"  // Different type!
};

// ✅ Same type
let x = if condition {
    "5"
} else {
    "five"
};
```

### 2. Missing Match Arms

```rust
// ❌ Non-exhaustive
match option {
    Some(x) => println!("{}", x),
    // Missing None case!
}

// ✅ Exhaustive
match option {
    Some(x) => println!("{}", x),
    None => println!("No value"),
}
```

### 3. Unreachable Patterns

```rust
// ❌ Second pattern never reached
match x {
    _ => println!("anything"),
    5 => println!("five"),  // Unreachable!
}

// ✅ Specific patterns first
match x {
    5 => println!("five"),
    _ => println!("anything"),
}
```

### 4. Infinite Loops

```rust
// ❌ Missing break
loop {
    println!("Forever!");
}

// ✅ With break condition
loop {
    if should_stop {
        break;
    }
}
```

## 🧪 Testing Control Flow

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_categorize() {
        assert_eq!(categorize_number(-5), "negative");
        assert_eq!(categorize_number(0), "zero");
        assert_eq!(categorize_number(5), "small positive");
    }
    
    #[test]
    fn test_grade() {
        assert_eq!(grade_letter(95), 'A');
        assert_eq!(grade_letter(85), 'B');
        assert_eq!(grade_letter(55), 'F');
    }
    
    #[test]
    #[should_panic]
    fn test_invalid_grade() {
        grade_letter(101);
    }
}
```

## 📚 Summary

Control flow in Rust is:

- **Expressive**: If and match are expressions
- **Safe**: Match ensures exhaustiveness
- **Powerful**: Pattern matching and guards
- **Efficient**: Compiler optimizations

### Key Takeaways

1. Use `if` for simple conditions
2. Use `match` for pattern matching and exhaustiveness
3. Use `for` loops for iteration
4. Use `loop` for infinite loops with break
5. Leverage pattern matching power
6. Prefer early returns to reduce nesting
7. Let the compiler help with exhaustiveness

## 🔗 Next Steps

- Explore **pattern matching** in depth
- Learn about **iterators** and functional programming
- Study **error handling** with Result and Option
- Practice with the example projects!
