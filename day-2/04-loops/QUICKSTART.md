# Loops in Rust - Quick Reference

## Loop Types Overview

| Loop Type | Use Case | Breaking | Continuing |
|-----------|----------|----------|------------|
| `loop` | Infinite loops | `break` | `continue` |
| `while` | Conditional loops | `break` | `continue` |
| `for` | Iteration loops | `break` | `continue` |

## Basic Syntax

### Infinite Loop
```rust
loop {
    // Runs forever until break
    break;
}
```

### While Loop
```rust
while condition {
    // Runs while condition is true
}
```

### For Loop
```rust
for item in collection {
    // Iterates over collection
}
```

## Common Loop Patterns

### Range Iteration
```rust
// Exclusive range
for i in 0..10 {
    println!("{}", i); // 0 to 9
}

// Inclusive range
for i in 0..=10 {
    println!("{}", i); // 0 to 10
}

// Reverse range
for i in (0..10).rev() {
    println!("{}", i); // 9 to 0
}
```

### Array/Vector Iteration
```rust
let items = vec![1, 2, 3, 4, 5];

// By value
for item in items {
    println!("{}", item);
}

// By reference
for item in &items {
    println!("{}", item);
}

// By mutable reference
for item in &mut items {
    *item *= 2;
}

// With index
for (index, item) in items.iter().enumerate() {
    println!("{}: {}", index, item);
}
```

### Loop with Return Value
```rust
let result = loop {
    let value = get_value();
    if value > 10 {
        break value * 2;
    }
};
```

## Loop Control

### Break
```rust
for i in 0..10 {
    if i == 5 {
        break; // Exit loop
    }
}
```

### Continue
```rust
for i in 0..10 {
    if i % 2 == 0 {
        continue; // Skip to next iteration
    }
    println!("{}", i);
}
```

### Nested Loops with Labels
```rust
'outer: for x in 0..5 {
    'inner: for y in 0..5 {
        if x * y > 10 {
            break 'outer;
        }
        if y == 2 {
            continue 'inner;
        }
    }
}
```

## Iterator Methods

### Common Iterator Adapters
```rust
let numbers = vec![1, 2, 3, 4, 5];

// map - transform elements
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

// filter - select elements
let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// fold - accumulate values
let sum = numbers.iter().fold(0, |acc, x| acc + x);

// take - limit elements
let first_three: Vec<_> = numbers.iter().take(3).collect();

// skip - skip elements
let after_two: Vec<_> = numbers.iter().skip(2).collect();

// chain - combine iterators
let combined: Vec<_> = numbers.iter().chain(&[6, 7, 8]).collect();
```

### Consuming vs Non-Consuming
```rust
let numbers = vec![1, 2, 3, 4, 5];

// Non-consuming (returns iterator)
for num in &numbers {
    println!("{}", num);
}
// numbers still available

// Consuming (takes ownership)
for num in numbers {
    println!("{}", num);
}
// numbers no longer available
```

## Performance Tips

1. **Use iterators instead of indexing**
   ```rust
   // Slower
   for i in 0..vec.len() {
       println!("{}", vec[i]);
   }
   
   // Faster
   for item in &vec {
       println!("{}", item);
   }
   ```

2. **Prefer `for` over `while` for known iterations**
   ```rust
   // Less efficient
   let mut i = 0;
   while i < 10 {
       println!("{}", i);
       i += 1;
   }
   
   // More efficient
   for i in 0..10 {
       println!("{}", i);
   }
   ```

3. **Use `loop` for infinite loops**
   ```rust
   // Less clear
   while true {
       // ...
   }
   
   // Better
   loop {
       // ...
   }
   ```

## Common Patterns

### Process Until Condition
```rust
let mut count = 0;
loop {
    count += 1;
    if count > 10 {
        break;
    }
}
```

### Retry Logic
```rust
let mut attempts = 0;
let max_attempts = 5;

loop {
    attempts += 1;
    
    if try_operation() {
        break;
    }
    
    if attempts >= max_attempts {
        println!("Max attempts reached");
        break;
    }
}
```

### Menu Loop
```rust
loop {
    display_menu();
    let choice = get_user_input();
    
    match choice {
        1 => handle_option_1(),
        2 => handle_option_2(),
        0 => break,
        _ => println!("Invalid choice"),
    }
}
```

### Accumulation
```rust
let mut sum = 0;
for i in 1..=100 {
    sum += i;
}
println!("Sum: {}", sum);
```

### Filtering and Collecting
```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
let evens: Vec<i32> = numbers
    .iter()
    .filter(|&x| x % 2 == 0)
    .copied()
    .collect();
```

## Best Practices

1. **Use the right loop type**
   - `loop` for infinite loops or when you need to return a value
   - `while` for conditional repetition
   - `for` for iterating over collections or ranges

2. **Avoid infinite loops without break conditions**
   ```rust
   // Bad
   loop {
       process();
   }
   
   // Good
   loop {
       if should_stop() {
           break;
       }
       process();
   }
   ```

3. **Use iterator methods for cleaner code**
   ```rust
   // Verbose
   let mut result = Vec::new();
   for i in 0..10 {
       if i % 2 == 0 {
           result.push(i * 2);
       }
   }
   
   // Cleaner
   let result: Vec<_> = (0..10)
       .filter(|x| x % 2 == 0)
       .map(|x| x * 2)
       .collect();
   ```

4. **Use labels for nested loops**
   ```rust
   'outer: for x in 0..10 {
       for y in 0..10 {
           if x * y > 50 {
               break 'outer;
           }
       }
   }
   ```

5. **Avoid modifying loop variables in for loops**
   ```rust
   // Bad
   for mut i in 0..10 {
       i += 1; // This doesn't affect the loop
   }
   
   // Good
   for i in 0..10 {
       let modified = i + 1;
   }
   ```

## Troubleshooting

### Issue: Borrow checker errors in loops
**Solution**: Use references appropriately
```rust
let mut vec = vec![1, 2, 3];

// Error: can't borrow as mutable more than once
// for item in &vec {
//     vec.push(4);
// }

// Solution: collect indices first
let indices: Vec<_> = (0..vec.len()).collect();
for i in indices {
    vec.push(vec[i]);
}
```

### Issue: Infinite loop
**Solution**: Add break condition
```rust
let mut count = 0;
loop {
    count += 1;
    if count > 1000 {
        break; // Safety break
    }
}
```

### Issue: Off-by-one errors
**Solution**: Use inclusive ranges or adjust bounds
```rust
// Exclusive: 0 to 9
for i in 0..10 {
    println!("{}", i);
}

// Inclusive: 0 to 10
for i in 0..=10 {
    println!("{}", i);
}
```

## Quick Examples

### Example 1: Sum of Numbers
```rust
let sum: i32 = (1..=100).sum();
println!("Sum: {}", sum);
```

### Example 2: Factorial
```rust
let mut factorial = 1;
for i in 1..=5 {
    factorial *= i;
}
println!("5! = {}", factorial);
```

### Example 3: Find First Match
```rust
let numbers = vec![1, 3, 5, 8, 9, 11];
let mut first_even = None;

for &num in &numbers {
    if num % 2 == 0 {
        first_even = Some(num);
        break;
    }
}
```

### Example 4: Nested Loop Pattern
```rust
for i in 1..=5 {
    for j in 1..=i {
        print!("* ");
    }
    println!();
}
```

### Example 5: While Loop with Counter
```rust
let mut count = 0;
while count < 10 {
    println!("Count: {}", count);
    count += 1;
}
```

## Additional Resources

- [The Rust Book - Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust by Example - Loops](https://doc.rust-lang.org/rust-by-example/flow_control/loop.html)
- [Iterator Documentation](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
