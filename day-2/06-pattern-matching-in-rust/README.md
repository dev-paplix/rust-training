# Pattern Matching in Rust

Pattern matching is one of Rust's most powerful features, allowing you to compare values against patterns and execute code based on which pattern matches. This module covers all aspects of pattern matching in Rust.

## Table of Contents

1. [Introduction to Pattern Matching](#introduction-to-pattern-matching)
2. [Basic Match Expression](#basic-match-expression)
3. [Pattern Types](#pattern-types)
4. [Destructuring](#destructuring)
5. [Match Guards](#match-guards)
6. [If Let and While Let](#if-let-and-while-let)
7. [Pattern Matching with Enums](#pattern-matching-with-enums)
8. [Advanced Patterns](#advanced-patterns)
9. [Refutability](#refutability)
10. [Best Practices](#best-practices)

---

## Introduction to Pattern Matching

Pattern matching allows you to:
- Compare a value against a series of patterns
- Execute different code based on which pattern matches
- Extract values from complex data structures
- Write more expressive and safer code

### Why Pattern Matching?

**Traditional Approach (Without Pattern Matching):**
```rust
fn describe_number(n: i32) -> &'static str {
    if n == 0 {
        "zero"
    } else if n > 0 && n < 10 {
        "small positive"
    } else if n < 0 && n > -10 {
        "small negative"
    } else {
        "large"
    }
}
```

**Pattern Matching Approach:**
```rust
fn describe_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=9 => "small positive",
        -9..=-1 => "small negative",
        _ => "large",
    }
}
```

---

## Basic Match Expression

### Syntax

```rust
match value {
    pattern1 => expression1,
    pattern2 => expression2,
    pattern3 => expression3,
    _ => default_expression,
}
```

### Simple Example

```rust
fn main() {
    let number = 7;
    
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "something else",
    };
    
    println!("{} is {}", number, description);
}
```

### Key Points

1. **Exhaustive**: Must handle all possible values
2. **Expression**: Match returns a value
3. **Order Matters**: First matching pattern wins
4. **No Fallthrough**: Unlike switch in other languages

### Match with Multiple Lines

```rust
match number {
    1 => {
        println!("It's one!");
        println!("The loneliest number");
    }
    2 => {
        println!("It's two!");
        println!("Takes two to tango");
    }
    _ => println!("Some other number"),
}
```

---

## Pattern Types

### 1. Literal Patterns

Match specific values:

```rust
match character {
    'a' => println!("The letter a"),
    'b' => println!("The letter b"),
    '1' => println!("The number one"),
    _ => println!("Something else"),
}
```

### 2. Variable Patterns

Bind matched value to a variable:

```rust
match value {
    x => println!("Matched and bound to x: {}", x),
}
```

### 3. Wildcard Pattern (`_`)

Ignore the value:

```rust
match value {
    0 => println!("Zero"),
    _ => println!("Not zero"),
}
```

### 4. Range Patterns

Match a range of values:

```rust
match age {
    0..=12 => println!("Child"),
    13..=19 => println!("Teenager"),
    20..=64 => println!("Adult"),
    65.. => println!("Senior"),
}
```

**Note**: Ranges are inclusive on both ends with `..=`

### 5. Multiple Patterns (OR)

Match multiple patterns with `|`:

```rust
match character {
    'a' | 'e' | 'i' | 'o' | 'u' => println!("Vowel"),
    'y' => println!("Sometimes a vowel"),
    _ => println!("Consonant"),
}
```

### 6. Named Variables

```rust
let x = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    n => println!("The number is {}", n),
}
```

---

## Destructuring

### Tuple Destructuring

```rust
fn main() {
    let tuple = (1, "hello", 3.14);
    
    match tuple {
        (1, text, _) => println!("First is 1, second is {}", text),
        (_, _, pi) => println!("Third value is {}", pi),
    }
    
    // Destructuring in let
    let (x, y, z) = tuple;
    println!("x: {}, y: {}, z: {}", x, y, z);
}
```

### Ignoring Parts of Tuples

```rust
let tuple = (1, 2, 3, 4, 5);

// Ignore specific positions
let (first, _, third, _, fifth) = tuple;

// Ignore rest with ..
let (first, .., last) = tuple;

match tuple {
    (1, .., 5) => println!("Starts with 1, ends with 5"),
    _ => println!("Other pattern"),
}
```

### Struct Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    
    // Basic destructuring
    match point {
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }
    
    // Rename fields
    match point {
        Point { x: horizontal, y: vertical } => {
            println!("h: {}, v: {}", horizontal, vertical)
        }
    }
    
    // Ignore some fields
    match point {
        Point { x, .. } => println!("x: {}", x),
    }
    
    // Match specific values
    match point {
        Point { x: 0, y } => println!("On y-axis at {}", y),
        Point { x, y: 0 } => println!("On x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```

### Enum Destructuring

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Changing color to RGB({}, {}, {})", r, g, b)
        }
    }
}
```

### Nested Destructuring

```rust
enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Rgb(255, 0, 0));
    
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB: {}, {}, {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV: {}, {}, {}", h, s, v)
        }
        Message::Move { x, y } => println!("Move to {}, {}", x, y),
        Message::Quit => println!("Quit"),
    }
}
```

---

## Match Guards

Guards add additional conditions to patterns using `if`:

### Basic Guards

```rust
fn main() {
    let number = 4;
    
    match number {
        n if n < 0 => println!("Negative: {}", n),
        n if n % 2 == 0 => println!("Even: {}", n),
        n if n % 2 != 0 => println!("Odd: {}", n),
        _ => println!("Zero"),
    }
}
```

### Guards with Multiple Patterns

```rust
match value {
    4 | 5 | 6 if value % 2 == 0 => println!("Even number 4, 5, or 6"),
    4 | 5 | 6 => println!("Odd number 4, 5, or 6"),
    _ => println!("Other"),
}
```

### Guards with Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 10, y: 20 };
    
    match point {
        Point { x, y } if x == y => println!("On diagonal"),
        Point { x, y } if x > y => println!("x is greater"),
        Point { x, y } if x < y => println!("y is greater"),
        _ => println!("Other"),
    }
}
```

### Complex Guards

```rust
fn categorize_age(age: u32, is_student: bool) -> &'static str {
    match age {
        a if a < 18 && is_student => "Student (minor)",
        a if a < 18 => "Minor",
        a if a >= 18 && a < 65 && is_student => "Student (adult)",
        a if a >= 18 && a < 65 => "Adult",
        _ => "Senior",
    }
}
```

---

## If Let and While Let

### If Let

Use when you only care about one pattern:

```rust
// Instead of this:
match some_option {
    Some(value) => println!("Got: {}", value),
    None => (),
}

// Use this:
if let Some(value) = some_option {
    println!("Got: {}", value);
}
```

### If Let with Else

```rust
let some_value = Some(5);

if let Some(x) = some_value {
    println!("Got value: {}", x);
} else {
    println!("No value");
}
```

### If Let Else If Let

```rust
fn main() {
    let color = Color::Rgb(128, 128, 128);
    
    if let Color::Rgb(r, g, b) = color {
        println!("RGB: {}, {}, {}", r, g, b);
    } else if let Color::Hsv(h, s, v) = color {
        println!("HSV: {}, {}, {}", h, s, v);
    } else {
        println!("Unknown color");
    }
}
```

### While Let

Loop while pattern matches:

```rust
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];
    
    while let Some(value) = stack.pop() {
        println!("Popped: {}", value);
    }
    
    println!("Stack is now empty");
}
```

### While Let with Iterators

```rust
fn main() {
    let mut iter = vec![1, 2, 3].into_iter();
    
    while let Some(value) = iter.next() {
        println!("Value: {}", value);
    }
}
```

---

## Pattern Matching with Enums

### Option Enum

```rust
fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn main() {
    let result = divide(10.0, 2.0);
    
    match result {
        Some(value) => println!("Result: {}", value),
        None => println!("Cannot divide by zero"),
    }
}
```

### Result Enum

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(format!("'{}' is not a valid number", s)),
    }
}

fn main() {
    let input = "42";
    
    match parse_number(input) {
        Ok(num) => println!("Parsed number: {}", num),
        Err(e) => println!("Error: {}", e),
    }
}
```

### Custom Enums

```rust
enum Status {
    Active,
    Inactive,
    Pending,
    Cancelled { reason: String },
}

fn handle_status(status: Status) {
    match status {
        Status::Active => println!("Currently active"),
        Status::Inactive => println!("Not active"),
        Status::Pending => println!("Waiting..."),
        Status::Cancelled { reason } => {
            println!("Cancelled because: {}", reason)
        }
    }
}
```

---

## Advanced Patterns

### @ Bindings

Bind value to variable while testing:

```rust
fn main() {
    let age = 15;
    
    match age {
        n @ 0..=12 => println!("Child of age {}", n),
        n @ 13..=19 => println!("Teen of age {}", n),
        n @ 20..=64 => println!("Adult of age {}", n),
        n @ 65.. => println!("Senior of age {}", n),
    }
}
```

### @ with Guards

```rust
match point {
    p @ Point { x, y } if x == y => {
        println!("On diagonal at {:?}", p)
    }
    Point { x, y } => println!("Not on diagonal: ({}, {})", x, y),
}
```

### Reference Patterns

```rust
fn main() {
    let value = 5;
    let reference = &value;
    
    match reference {
        &val => println!("Got value via reference: {}", val),
    }
    
    // Or
    match value {
        ref r => println!("Got reference to value: {}", r),
    }
}
```

### Mutable References

```rust
fn main() {
    let mut value = 5;
    
    match value {
        ref mut r => {
            *r += 10;
            println!("Modified to: {}", r);
        }
    }
    
    println!("Final value: {}", value);
}
```

### Slice Patterns

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5];
    
    match numbers {
        [] => println!("Empty"),
        [x] => println!("One element: {}", x),
        [first, second] => println!("Two: {} and {}", first, second),
        [first, .., last] => println!("First: {}, Last: {}", first, last),
    }
}
```

### Rest Patterns

```rust
fn main() {
    let tuple = (1, 2, 3, 4, 5);
    
    match tuple {
        (first, .., last) => {
            println!("First: {}, Last: {}", first, last);
        }
    }
    
    let (a, .., z) = tuple;
    println!("a: {}, z: {}", a, z);
}
```

---

## Refutability

### Irrefutable Patterns

Patterns that always match:

```rust
// Let bindings - must be irrefutable
let x = 5;
let (a, b) = (1, 2);

// Function parameters - must be irrefutable
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}
```

### Refutable Patterns

Patterns that might not match:

```rust
// This won't compile - let requires irrefutable pattern
// let Some(x) = some_option; // ERROR!

// Use if let for refutable patterns
if let Some(x) = some_option {
    println!("Got: {}", x);
}

// Or match
match some_option {
    Some(x) => println!("Got: {}", x),
    None => println!("Nothing"),
}
```

### When to Use Each

```rust
// Irrefutable - always use let
let tuple = (1, 2, 3);
let (x, y, z) = tuple;

// Refutable - use if let or match
let option = Some(5);

// Good
if let Some(value) = option {
    println!("{}", value);
}

// Also good
match option {
    Some(value) => println!("{}", value),
    None => println!("No value"),
}

// Bad - won't compile
// let Some(value) = option;
```

---

## Best Practices

### 1. Prefer Match Over If-Else Chains

**Bad:**
```rust
if status == Status::Active {
    println!("Active");
} else if status == Status::Inactive {
    println!("Inactive");
} else if status == Status::Pending {
    println!("Pending");
}
```

**Good:**
```rust
match status {
    Status::Active => println!("Active"),
    Status::Inactive => println!("Inactive"),
    Status::Pending => println!("Pending"),
    Status::Cancelled { reason } => println!("Cancelled: {}", reason),
}
```

### 2. Use If Let for Single Patterns

**Bad:**
```rust
match some_value {
    Some(x) => println!("{}", x),
    None => (),
}
```

**Good:**
```rust
if let Some(x) = some_value {
    println!("{}", x);
}
```

### 3. Leverage Exhaustiveness

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// Compiler ensures all cases handled
fn time_to_wait(light: TrafficLight) -> u32 {
    match light {
        TrafficLight::Red => 60,
        TrafficLight::Yellow => 5,
        TrafficLight::Green => 45,
        // If you add a new variant, compiler will warn
    }
}
```

### 4. Use Guards for Complex Logic

```rust
match (x, y) {
    (a, b) if a > b => println!("a is greater"),
    (a, b) if a < b => println!("b is greater"),
    _ => println!("Equal"),
}
```

### 5. Destructure in Function Parameters

```rust
// Instead of this:
fn process_point(point: Point) {
    let x = point.x;
    let y = point.y;
    println!("({}, {})", x, y);
}

// Do this:
fn process_point(Point { x, y }: Point) {
    println!("({}, {})", x, y);
}
```

### 6. Use @ Bindings When Needed

```rust
match message {
    m @ Message::Write(_) => {
        log_message(&m);
        process_write(m);
    }
    other => process_other(other),
}
```

### 7. Order Patterns Correctly

```rust
// Good - specific patterns first
match value {
    0 => println!("Zero"),
    1..=10 => println!("Small"),
    _ => println!("Large"),
}

// Bad - unreachable patterns
match value {
    _ => println!("Anything"),      // This catches everything
    0 => println!("Zero"),          // Never reached!
}
```

---

## Common Patterns and Idioms

### Processing Collections

```rust
fn process_vector(vec: Vec<i32>) {
    match vec.as_slice() {
        [] => println!("Empty"),
        [x] => println!("Single element: {}", x),
        [first, .., last] => {
            println!("First: {}, Last: {}", first, last);
        }
    }
}
```

### State Machines

```rust
enum State {
    Idle,
    Running { progress: u32 },
    Paused { progress: u32 },
    Completed,
    Failed { error: String },
}

fn transition(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::Start) => State::Running { progress: 0 },
        (State::Running { progress }, Event::Pause) => {
            State::Paused { progress }
        }
        (State::Paused { progress }, Event::Resume) => {
            State::Running { progress }
        }
        (State::Running { .. }, Event::Complete) => State::Completed,
        (_, Event::Error(e)) => State::Failed { error: e },
        (s, _) => s, // Invalid transition, keep current state
    }
}
```

### Option Chaining

```rust
fn get_user_email(user_id: u32) -> Option<String> {
    let user = find_user(user_id)?;
    let profile = user.profile?;
    Some(profile.email)
}

// With pattern matching
fn get_user_email_match(user_id: u32) -> Option<String> {
    match find_user(user_id) {
        Some(user) => match user.profile {
            Some(profile) => Some(profile.email),
            None => None,
        },
        None => None,
    }
}
```

### Validation Patterns

```rust
fn validate_age(age: i32) -> Result<u32, String> {
    match age {
        a if a < 0 => Err("Age cannot be negative".to_string()),
        a if a > 150 => Err("Age too high".to_string()),
        a => Ok(a as u32),
    }
}
```

---

## Examples from the Projects

### Project 1: Match Expressions Demo
Demonstrates basic match expressions with:
- Number classification
- Character type detection
- Day of week handling
- Traffic light simulation

### Project 2: Enum Pattern Matching
Shows pattern matching with enums:
- Custom message types
- Option and Result handling
- Nested enum matching
- State machine implementation

### Project 3: Destructuring Patterns
Covers destructuring:
- Tuple destructuring
- Struct field extraction
- Array and slice patterns
- Nested destructuring

### Project 4: Advanced Pattern Matching
Advanced features:
- Match guards
- @ bindings
- Reference patterns
- Pattern matching in function parameters

---

## Testing Pattern Matching

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_number_classification() {
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(5), "small positive");
        assert_eq!(classify_number(-5), "small negative");
        assert_eq!(classify_number(100), "large");
    }
    
    #[test]
    fn test_option_matching() {
        let some = Some(5);
        let none: Option<i32> = None;
        
        assert_eq!(process_option(some), 5);
        assert_eq!(process_option(none), 0);
    }
    
    #[test]
    fn test_destructuring() {
        let point = Point { x: 10, y: 20 };
        let Point { x, y } = point;
        
        assert_eq!(x, 10);
        assert_eq!(y, 20);
    }
}
```

---

## Common Mistakes and Solutions

### Mistake 1: Non-Exhaustive Patterns

```rust
// Wrong - missing case
match number {
    1 => println!("one"),
    2 => println!("two"),
    // Error: non-exhaustive patterns
}

// Right - handle all cases
match number {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("other"),
}
```

### Mistake 2: Using Refutable Pattern in Let

```rust
// Wrong
let Some(x) = option_value; // Error!

// Right
if let Some(x) = option_value {
    println!("{}", x);
}
```

### Mistake 3: Moving Values Unintentionally

```rust
// Wrong - moves value
match some_string {
    s => {
        println!("{}", s);
        // some_string is now moved
    }
}

// Right - use reference
match &some_string {
    s => println!("{}", s),
    // some_string still valid
}
```

---

## Summary

Pattern matching in Rust provides:
- **Type Safety**: Compiler enforces exhaustive matching
- **Clarity**: More readable than if-else chains
- **Power**: Destructure complex types easily
- **Flexibility**: Guards, bindings, and multiple patterns

Key takeaways:
1. Use `match` for multiple patterns
2. Use `if let` for single patterns
3. Leverage exhaustiveness checking
4. Destructure to extract data
5. Use guards for complex conditions
6. Prefer patterns over manual extraction

Pattern matching is fundamental to writing idiomatic Rust code!
