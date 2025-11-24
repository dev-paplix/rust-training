# Executing Code Repeatedly (Loops) in Rust

## Table of Contents
- [Introduction](#introduction)
- [Loop Types](#loop-types)
  - [The `loop` Keyword](#the-loop-keyword)
  - [While Loops](#while-loops)
  - [For Loops](#for-loops)
- [Loop Control](#loop-control)
  - [Breaking Loops](#breaking-loops)
  - [Continuing Loops](#continuing-loops)
  - [Loop Labels](#loop-labels)
- [Returning Values from Loops](#returning-values-from-loops)
- [Iterators](#iterators)
  - [Iterator Basics](#iterator-basics)
  - [Iterator Adapters](#iterator-adapters)
  - [Consuming Adapters](#consuming-adapters)
- [Common Loop Patterns](#common-loop-patterns)
- [Performance Considerations](#performance-considerations)
- [Best Practices](#best-practices)
- [Examples](#examples)
- [Exercises](#exercises)

## Introduction

Loops are fundamental control flow constructs that allow you to execute code repeatedly. Rust provides three main types of loops:

1. **`loop`** - Infinite loops with explicit break conditions
2. **`while`** - Conditional loops that run while a condition is true
3. **`for`** - Iteration loops for traversing collections and ranges

Each loop type has specific use cases and advantages. Understanding when to use each type is crucial for writing efficient and idiomatic Rust code.

## Loop Types

### The `loop` Keyword

The `loop` keyword creates an infinite loop that runs until explicitly broken.

#### Basic Syntax
```rust
loop {
    // Code that runs forever
    // Use break to exit
}
```

#### Simple Example
```rust
fn main() {
    let mut count = 0;
    
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count >= 5 {
            break;
        }
    }
    
    println!("Loop finished!");
}
```

**Output:**
```
Count: 1
Count: 2
Count: 3
Count: 4
Count: 5
Loop finished!
```

#### When to Use `loop`
- When you need an infinite loop
- When you want to return a value from the loop
- When the termination condition is complex
- For retry logic or event loops

### While Loops

While loops execute code as long as a condition remains true.

#### Basic Syntax
```rust
while condition {
    // Code that runs while condition is true
}
```

#### Simple Example
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

**Output:**
```
3!
2!
1!
LIFTOFF!!!
```

#### While Loop with Complex Condition
```rust
fn main() {
    let mut temperature = 20;
    let mut time = 0;
    
    while temperature < 100 && time < 30 {
        temperature += 5;
        time += 1;
        println!("Time: {}min, Temperature: {}°C", time, temperature);
    }
    
    if temperature >= 100 {
        println!("Water is boiling!");
    } else {
        println!("Time's up!");
    }
}
```

#### When to Use `while`
- When you have a clear termination condition
- When the condition is checked before each iteration
- For countdown or countdown-based operations
- When the number of iterations is unknown

### For Loops

For loops iterate over collections, ranges, or any type that implements the `Iterator` trait.

#### Basic Syntax
```rust
for item in collection {
    // Code that runs for each item
}
```

#### Range Iteration
```rust
fn main() {
    // Exclusive range (0 to 4)
    println!("Exclusive range (0..5):");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    
    // Inclusive range (0 to 5)
    println!("\nInclusive range (0..=5):");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();
    
    // Reverse range
    println!("\nReverse range:");
    for i in (0..5).rev() {
        print!("{} ", i);
    }
    println!();
}
```

**Output:**
```
Exclusive range (0..5):
0 1 2 3 4 

Inclusive range (0..=5):
0 1 2 3 4 5 

Reverse range:
4 3 2 1 0 
```

#### Array and Vector Iteration
```rust
fn main() {
    let array = [10, 20, 30, 40, 50];
    
    // Iterate by value (array is Copy)
    println!("By value:");
    for element in array {
        println!("{}", element);
    }
    
    let vec = vec![1, 2, 3, 4, 5];
    
    // Iterate by reference
    println!("\nBy reference:");
    for element in &vec {
        println!("{}", element);
    }
    
    // Iterate by mutable reference
    let mut mut_vec = vec![1, 2, 3, 4, 5];
    println!("\nModifying elements:");
    for element in &mut mut_vec {
        *element *= 2;
    }
    println!("{:?}", mut_vec);
    
    // Iterate with index
    println!("\nWith index:");
    for (index, value) in vec.iter().enumerate() {
        println!("Index {}: {}", index, value);
    }
}
```

#### When to Use `for`
- When iterating over collections (arrays, vectors, etc.)
- When working with ranges
- When you know the number of iterations
- For cleaner, more readable code

## Loop Control

### Breaking Loops

The `break` keyword exits a loop immediately.

#### Simple Break
```rust
fn main() {
    for i in 0..10 {
        if i == 5 {
            break;
        }
        println!("{}", i);
    }
    println!("Loop broken at 5");
}
```

**Output:**
```
0
1
2
3
4
Loop broken at 5
```

#### Break with Condition
```rust
fn main() {
    let numbers = vec![1, 3, 5, 7, 8, 10, 12];
    let mut first_even = None;
    
    for &num in &numbers {
        if num % 2 == 0 {
            first_even = Some(num);
            break;
        }
    }
    
    match first_even {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even numbers found"),
    }
}
```

### Continuing Loops

The `continue` keyword skips the rest of the current iteration and moves to the next one.

#### Simple Continue
```rust
fn main() {
    for i in 0..10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        println!("{}", i);
    }
}
```

**Output:**
```
1
3
5
7
9
```

#### Continue with Multiple Conditions
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    for &num in &numbers {
        // Skip numbers divisible by 2 or 3
        if num % 2 == 0 || num % 3 == 0 {
            continue;
        }
        println!("{}", num);
    }
}
```

**Output:**
```
1
5
7
```

### Loop Labels

Loop labels allow you to break or continue outer loops from within nested loops.

#### Syntax
```rust
'label_name: loop {
    // Outer loop
    loop {
        // Inner loop
        break 'label_name; // Breaks outer loop
    }
}
```

#### Nested Loop Example
```rust
fn main() {
    'outer: for x in 0..5 {
        'inner: for y in 0..5 {
            println!("x: {}, y: {}", x, y);
            
            if x == 2 && y == 2 {
                println!("Breaking outer loop");
                break 'outer;
            }
            
            if y == 3 {
                println!("Continuing outer loop");
                continue 'outer;
            }
        }
    }
    
    println!("Done!");
}
```

#### Practical Example: Matrix Search
```rust
fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    let target = 5;
    let mut found = false;
    
    'search: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                println!("Found {} at position ({}, {})", target, i, j);
                found = true;
                break 'search;
            }
        }
    }
    
    if !found {
        println!("Value {} not found", target);
    }
}
```

## Returning Values from Loops

The `loop` construct can return values using `break`.

#### Basic Syntax
```rust
let result = loop {
    // ...
    break value;
};
```

#### Simple Example
```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {}", result);
}
```

**Output:**
```
The result is 20
```

#### Practical Example: Finding Value
```rust
fn find_first_divisible(numbers: &[i32], divisor: i32) -> Option<i32> {
    let mut index = 0;
    
    let result = loop {
        if index >= numbers.len() {
            break None;
        }
        
        if numbers[index] % divisor == 0 {
            break Some(numbers[index]);
        }
        
        index += 1;
    };
    
    result
}

fn main() {
    let numbers = vec![1, 3, 5, 7, 9, 12, 15, 18];
    
    match find_first_divisible(&numbers, 4) {
        Some(n) => println!("First number divisible by 4: {}", n),
        None => println!("No number divisible by 4 found"),
    }
}
```

#### Returning from Nested Loops
```rust
fn main() {
    let mut count = 0;
    
    let result = 'outer: loop {
        count += 1;
        let mut inner_count = 0;
        
        loop {
            inner_count += 1;
            
            if count * inner_count > 50 {
                break 'outer count * inner_count;
            }
            
            if inner_count >= 10 {
                break;
            }
        }
    };
    
    println!("Result: {}", result);
}
```

## Iterators

### Iterator Basics

An iterator is a pattern that allows you to perform operations on sequences of elements.

#### Creating Iterators
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // iter() - borrows each element
    for val in v.iter() {
        println!("{}", val);
    }
    // v is still usable here
    
    // into_iter() - takes ownership
    // for val in v.into_iter() {
    //     println!("{}", val);
    // }
    // v is no longer usable here
    
    let mut v2 = vec![1, 2, 3, 4, 5];
    
    // iter_mut() - borrows each element mutably
    for val in v2.iter_mut() {
        *val *= 2;
    }
    println!("{:?}", v2);
}
```

#### Iterator Methods
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    
    // next() - gets next element
    let mut iter = v.iter();
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    
    // count() - counts elements
    let count = v.iter().count();
    println!("Count: {}", count);
    
    // last() - gets last element
    let last = v.iter().last();
    println!("Last: {:?}", last);
    
    // nth() - gets nth element
    let third = v.iter().nth(2);
    println!("Third: {:?}", third);
}
```

### Iterator Adapters

Iterator adapters transform iterators into new iterators.

#### map - Transform Elements
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect();
    
    println!("{:?}", doubled); // [2, 4, 6, 8, 10]
    
    // Chaining multiple maps
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .map(|x| x + 1)
        .collect();
    
    println!("{:?}", result); // [3, 5, 7, 9, 11]
}
```

#### filter - Select Elements
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0)
        .copied()
        .collect();
    
    println!("Evens: {:?}", evens); // [2, 4, 6, 8, 10]
    
    // Filter with multiple conditions
    let result: Vec<i32> = numbers
        .iter()
        .filter(|&x| x % 2 == 0 && x > 5)
        .copied()
        .collect();
    
    println!("Evens > 5: {:?}", result); // [6, 8, 10]
}
```

#### take and skip
```rust
fn main() {
    let numbers: Vec<i32> = (1..=10).collect();
    
    // take - first n elements
    let first_five: Vec<i32> = numbers
        .iter()
        .take(5)
        .copied()
        .collect();
    
    println!("First 5: {:?}", first_five); // [1, 2, 3, 4, 5]
    
    // skip - skip first n elements
    let after_five: Vec<i32> = numbers
        .iter()
        .skip(5)
        .copied()
        .collect();
    
    println!("After 5: {:?}", after_five); // [6, 7, 8, 9, 10]
    
    // Combine take and skip
    let middle: Vec<i32> = numbers
        .iter()
        .skip(3)
        .take(4)
        .copied()
        .collect();
    
    println!("Middle: {:?}", middle); // [4, 5, 6, 7]
}
```

#### enumerate - Add Indices
```rust
fn main() {
    let words = vec!["hello", "world", "rust"];
    
    for (index, word) in words.iter().enumerate() {
        println!("{}: {}", index, word);
    }
    
    // Filter with indices
    let result: Vec<_> = words
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, w)| w)
        .collect();
    
    println!("{:?}", result); // ["hello", "rust"]
}
```

#### zip - Combine Iterators
```rust
fn main() {
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
    
    // Collect into vector of tuples
    let people: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("{:?}", people);
}
```

#### chain - Concatenate Iterators
```rust
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    
    let combined: Vec<i32> = v1
        .iter()
        .chain(v2.iter())
        .copied()
        .collect();
    
    println!("{:?}", combined); // [1, 2, 3, 4, 5, 6]
}
```

#### flat_map - Flatten and Map
```rust
fn main() {
    let words = vec!["hello", "world"];
    
    let chars: Vec<char> = words
        .iter()
        .flat_map(|s| s.chars())
        .collect();
    
    println!("{:?}", chars); // ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
}
```

### Consuming Adapters

Consuming adapters process the entire iterator and return a value.

#### collect - Gather into Collection
```rust
fn main() {
    let v: Vec<i32> = (1..=5).collect();
    println!("{:?}", v);
    
    // Collect into different types
    use std::collections::HashSet;
    let set: HashSet<i32> = (1..=5).collect();
    println!("{:?}", set);
    
    // Collect string
    let s: String = vec!['h', 'e', 'l', 'l', 'o']
        .into_iter()
        .collect();
    println!("{}", s);
}
```

#### sum and product
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum); // 15
    
    let product: i32 = numbers.iter().product();
    println!("Product: {}", product); // 120
}
```

#### fold - Accumulate Values
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Sum using fold
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum: {}", sum); // 15
    
    // Concatenate strings
    let words = vec!["Hello", " ", "World", "!"];
    let sentence = words.iter().fold(String::new(), |acc, s| acc + s);
    println!("{}", sentence); // "Hello World!"
    
    // Find maximum
    let max = numbers.iter().fold(i32::MIN, |acc, &x| acc.max(x));
    println!("Max: {}", max); // 5
}
```

#### reduce - Similar to Fold
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum = numbers.iter().reduce(|acc, x| acc + x);
    println!("Sum: {:?}", sum); // Some(15)
    
    let empty: Vec<i32> = vec![];
    let sum = empty.iter().reduce(|acc, x| acc + x);
    println!("Empty sum: {:?}", sum); // None
}
```

#### any and all
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    let has_even = numbers.iter().any(|&x| x % 2 == 0);
    println!("Has even: {}", has_even); // true
    
    let all_positive = numbers.iter().all(|&x| x > 0);
    println!("All positive: {}", all_positive); // true
    
    let all_even = numbers.iter().all(|&x| x % 2 == 0);
    println!("All even: {}", all_even); // false
}
```

#### find - First Match
```rust
fn main() {
    let numbers = vec![1, 3, 5, 8, 9, 11];
    
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even); // Some(8)
    
    let first_negative = numbers.iter().find(|&&x| x < 0);
    println!("First negative: {:?}", first_negative); // None
}
```

#### position - Index of First Match
```rust
fn main() {
    let numbers = vec![1, 3, 5, 8, 9, 11];
    
    let pos = numbers.iter().position(|&x| x % 2 == 0);
    println!("Position of first even: {:?}", pos); // Some(3)
}
```

## Common Loop Patterns

### Pattern 1: Counting and Accumulation
```rust
fn main() {
    // Count occurrences
    let text = "hello world";
    let mut count = 0;
    
    for ch in text.chars() {
        if ch == 'l' {
            count += 1;
        }
    }
    println!("'l' appears {} times", count);
    
    // Accumulate values
    let numbers = vec![1, 2, 3, 4, 5];
    let mut sum = 0;
    
    for &num in &numbers {
        sum += num;
    }
    println!("Sum: {}", sum);
}
```

### Pattern 2: Searching
```rust
fn main() {
    let numbers = vec![1, 3, 5, 7, 9, 12, 15];
    
    // Linear search
    let target = 12;
    let mut found_index = None;
    
    for (i, &num) in numbers.iter().enumerate() {
        if num == target {
            found_index = Some(i);
            break;
        }
    }
    
    match found_index {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
}
```

### Pattern 3: Filtering and Transformation
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Manual filtering
    let mut evens = Vec::new();
    for &num in &numbers {
        if num % 2 == 0 {
            evens.push(num);
        }
    }
    println!("Evens: {:?}", evens);
    
    // Using iterator (idiomatic)
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    println!("Evens: {:?}", evens);
}
```

### Pattern 4: Nested Iteration
```rust
fn main() {
    // Multiplication table
    for i in 1..=5 {
        for j in 1..=5 {
            print!("{:3} ", i * j);
        }
        println!();
    }
}
```

### Pattern 5: Menu Loop
```rust
use std::io::{self, Write};

fn display_menu() {
    println!("\n=== Menu ===");
    println!("1. Option 1");
    println!("2. Option 2");
    println!("3. Option 3");
    println!("0. Exit");
    print!("Choose: ");
    io::stdout().flush().unwrap();
}

fn get_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn main() {
    loop {
        display_menu();
        let choice = get_choice();
        
        match choice {
            1 => println!("You chose option 1"),
            2 => println!("You chose option 2"),
            3 => println!("You chose option 3"),
            0 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice"),
        }
    }
}
```

### Pattern 6: Retry Logic
```rust
fn main() {
    let max_attempts = 3;
    let mut attempts = 0;
    let mut success = false;
    
    while attempts < max_attempts && !success {
        attempts += 1;
        println!("Attempt {}", attempts);
        
        // Simulate operation
        success = attempts == 2;
        
        if success {
            println!("Success!");
        } else if attempts < max_attempts {
            println!("Retrying...");
        }
    }
    
    if !success {
        println!("Failed after {} attempts", max_attempts);
    }
}
```

### Pattern 7: Sliding Window
```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let window_size = 3;
    
    for i in 0..=numbers.len() - window_size {
        let window = &numbers[i..i + window_size];
        let sum: i32 = window.iter().sum();
        println!("Window {:?}: sum = {}", window, sum);
    }
}
```

### Pattern 8: Two Pointers
```rust
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut left = 0;
    let mut right = numbers.len() - 1;
    
    // Reverse array in place
    while left < right {
        numbers.swap(left, right);
        left += 1;
        right -= 1;
    }
    
    println!("{:?}", numbers);
}
```

## Performance Considerations

### 1. Iterator vs Index-Based Loops

```rust
use std::time::Instant;

fn main() {
    let numbers: Vec<i32> = (0..1_000_000).collect();
    
    // Index-based (slower)
    let start = Instant::now();
    let mut sum = 0;
    for i in 0..numbers.len() {
        sum += numbers[i];
    }
    println!("Index-based: {:?}", start.elapsed());
    
    // Iterator (faster)
    let start = Instant::now();
    let sum: i32 = numbers.iter().sum();
    println!("Iterator: {:?}", start.elapsed());
}
```

### 2. Lazy Evaluation

Iterators in Rust are lazy - they don't do work until consumed.

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // This doesn't execute yet
    let iter = numbers.iter().map(|x| {
        println!("Mapping {}", x);
        x * 2
    });
    
    println!("Created iterator");
    
    // Now it executes
    let result: Vec<_> = iter.collect();
    println!("{:?}", result);
}
```

### 3. Avoid Unnecessary Allocations

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Inefficient - creates intermediate vectors
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .collect::<Vec<i32>>() // Unnecessary
        .iter()
        .filter(|&x| x > 5)
        .copied()
        .collect();
    
    // Efficient - no intermediate allocations
    let result: Vec<i32> = numbers
        .iter()
        .map(|x| x * 2)
        .filter(|&x| x > 5)
        .collect();
    
    println!("{:?}", result);
}
```

### 4. Use the Right Loop Type

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Inefficient - using while for known iterations
    let mut i = 0;
    while i < numbers.len() {
        println!("{}", numbers[i]);
        i += 1;
    }
    
    // Efficient - using for
    for &num in &numbers {
        println!("{}", num);
    }
}
```

## Best Practices

### 1. Choose the Right Loop Type

```rust
// Use loop for infinite loops
loop {
    if should_exit() {
        break;
    }
}

// Use while for conditional loops
while !done {
    process();
}

// Use for for iteration
for item in collection {
    handle(item);
}
```

### 2. Prefer Iterators

```rust
// Instead of this
let mut result = Vec::new();
for i in 0..10 {
    if i % 2 == 0 {
        result.push(i * 2);
    }
}

// Do this
let result: Vec<_> = (0..10)
    .filter(|x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

### 3. Use Descriptive Variable Names

```rust
// Bad
for i in data {
    // What is i?
}

// Good
for customer in customers {
    // Clear what we're iterating over
}
```

### 4. Keep Loop Bodies Small

```rust
// Bad - too much in loop body
for item in items {
    // 50 lines of code...
}

// Good - extract to function
for item in items {
    process_item(item);
}

fn process_item(item: Item) {
    // 50 lines of code...
}
```

### 5. Use Labels for Nested Loops

```rust
'outer: for x in 0..10 {
    for y in 0..10 {
        if x * y > 50 {
            break 'outer;
        }
    }
}
```

### 6. Avoid Modifying Loop Variables

```rust
// Bad
for mut i in 0..10 {
    i += 1; // Doesn't affect loop
    println!("{}", i);
}

// Good
for i in 0..10 {
    let modified = i + 1;
    println!("{}", modified);
}
```

## Examples

### Example 1: Fibonacci Sequence
```rust
fn fibonacci(n: usize) -> Vec<u64> {
    let mut sequence = vec![0, 1];
    
    for i in 2..n {
        let next = sequence[i - 1] + sequence[i - 2];
        sequence.push(next);
    }
    
    sequence
}

fn main() {
    let fib = fibonacci(10);
    println!("{:?}", fib);
}
```

### Example 2: Prime Numbers
```rust
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    
    true
}

fn primes_up_to(limit: u32) -> Vec<u32> {
    (2..=limit)
        .filter(|&n| is_prime(n))
        .collect()
}

fn main() {
    let primes = primes_up_to(50);
    println!("Primes up to 50: {:?}", primes);
}
```

### Example 3: String Reversal
```rust
fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

fn reverse_words(s: &str) -> String {
    s.split_whitespace()
        .rev()
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let text = "Hello World";
    println!("Reversed: {}", reverse_string(text));
    println!("Words reversed: {}", reverse_words(text));
}
```

### Example 4: Matrix Operations
```rust
fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if matrix.is_empty() {
        return vec![];
    }
    
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut result = vec![vec![0; rows]; cols];
    
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = matrix[i][j];
        }
    }
    
    result
}

fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    
    let transposed = transpose(matrix);
    
    for row in transposed {
        println!("{:?}", row);
    }
}
```

### Example 5: Word Frequency Counter
```rust
use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *freq.entry(word).or_insert(0) += 1;
    }
    
    freq
}

fn main() {
    let text = "hello world hello rust world";
    let freq = word_frequency(text);
    
    for (word, count) in freq {
        println!("{}: {}", word, count);
    }
}
```

## Exercises

### Exercise 1: Sum of Multiples
Write a function that finds the sum of all multiples of 3 or 5 below a given number.

```rust
fn sum_of_multiples(limit: u32) -> u32 {
    // Your code here
    0
}

fn main() {
    assert_eq!(sum_of_multiples(10), 23); // 3 + 5 + 6 + 9 = 23
    assert_eq!(sum_of_multiples(1000), 233168);
}
```

### Exercise 2: Palindrome Checker
Write a function to check if a number is a palindrome.

```rust
fn is_palindrome(n: u32) -> bool {
    // Your code here
    false
}

fn main() {
    assert_eq!(is_palindrome(121), true);
    assert_eq!(is_palindrome(123), false);
}
```

### Exercise 3: Largest Prime Factor
Find the largest prime factor of a given number.

```rust
fn largest_prime_factor(n: u64) -> u64 {
    // Your code here
    0
}

fn main() {
    assert_eq!(largest_prime_factor(13195), 29);
    assert_eq!(largest_prime_factor(600851475143), 6857);
}
```

### Exercise 4: Collatz Sequence
Generate the Collatz sequence for a given starting number.

```rust
fn collatz_sequence(mut n: u64) -> Vec<u64> {
    // Your code here
    vec![]
}

fn main() {
    let seq = collatz_sequence(13);
    println!("{:?}", seq);
    // Should be [13, 40, 20, 10, 5, 16, 8, 4, 2, 1]
}
```

### Exercise 5: Spiral Matrix
Create a spiral matrix of size n x n.

```rust
fn spiral_matrix(n: usize) -> Vec<Vec<i32>> {
    // Your code here
    vec![]
}

fn main() {
    let matrix = spiral_matrix(3);
    // Should produce:
    // [[1, 2, 3],
    //  [8, 9, 4],
    //  [7, 6, 5]]
    
    for row in matrix {
        println!("{:?}", row);
    }
}
```

## Summary

Rust provides three powerful loop constructs:

1. **`loop`** - For infinite loops and returning values
2. **`while`** - For conditional iteration
3. **`for`** - For iterating over collections and ranges

Key takeaways:

- Use `break` to exit loops and `continue` to skip iterations
- Loop labels allow control of nested loops
- Iterators provide a powerful and efficient way to work with sequences
- Iterator adapters (map, filter, etc.) enable functional-style programming
- Choose the appropriate loop type for better performance and readability
- Prefer iterators over manual indexing for cleaner, safer code

Understanding loops and iterators is fundamental to writing efficient and idiomatic Rust code. Practice with the provided examples and exercises to master these concepts.
