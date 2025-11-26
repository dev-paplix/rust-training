use std::io::{self, Write};

/// Demonstrates basic loop, while, and for loops in Rust
fn main() {
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => demonstrate_loop(),
            2 => demonstrate_while_loop(),
            3 => demonstrate_for_loop(),
            4 => demonstrate_loop_control(),
            5 => demonstrate_nested_loops(),
            6 => demonstrate_loop_labels(),
            7 => demonstrate_returning_values(),
            8 => demonstrate_range_loops(),
            0 => {
                println!("\nThank you for exploring Rust loops!");
                break;
            }
            _ => println!("\n❌ Invalid choice. Please try again."),
        }
        
        pause();
    }
}

fn display_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║       Basic Loops in Rust             ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Loop Keyword (Infinite Loop)");
    println!("2. While Loop");
    println!("3. For Loop");
    println!("4. Loop Control (break & continue)");
    println!("5. Nested Loops");
    println!("6. Loop Labels");
    println!("7. Returning Values from Loops");
    println!("8. Range Loops");
    println!("0. Exit");
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// 1. Loop Keyword Demonstration
fn demonstrate_loop() {
    println!("\n=== Loop Keyword (Infinite Loop) ===\n");
    
    println!("Example 1: Simple counting loop");
    let mut count = 0;
    loop {
        count += 1;
        println!("Count: {}", count);
        
        if count >= 5 {
            println!("Breaking out of loop!");
            break;
        }
    }
    
    println!("\nExample 2: Countdown");
    let mut countdown = 5;
    loop {
        println!("{}...", countdown);
        countdown -= 1;
        
        if countdown == 0 {
            println!("Liftoff! 🚀");
            break;
        }
    }
    
    println!("\nExample 3: Loop with multiple exit conditions");
    let mut attempts = 0;
    let max_attempts = 3;
    loop {
        attempts += 1;
        println!("Attempt {}/{}", attempts, max_attempts);
        
        if attempts >= max_attempts {
            println!("Maximum attempts reached!");
            break;
        }
        
        // Simulate random success
        if attempts == 2 {
            println!("Success on attempt {}!", attempts);
            break;
        }
    }
}

// 2. While Loop Demonstration
fn demonstrate_while_loop() {
    println!("\n=== While Loop ===\n");
    
    println!("Example 1: Count up to 5");
    let mut number = 1;
    while number <= 5 {
        println!("Number: {}", number);
        number += 1;
    }
    
    println!("\nExample 2: Countdown");
    let mut countdown = 5;
    while countdown > 0 {
        println!("{}!", countdown);
        countdown -= 1;
    }
    println!("LIFTOFF! 🚀");
    
    println!("\nExample 3: While with complex condition");
    let mut temperature = 20;
    let mut time = 0;
    
    println!("Heating water...");
    while temperature < 100 && time < 10 {
        temperature += 10;
        time += 1;
        println!("Time: {}min, Temperature: {}°C", time, temperature);
    }
    
    if temperature >= 100 {
        println!("Water is boiling! 💧");
    } else {
        println!("Time's up!");
    }
    
    println!("\nExample 4: Fibonacci sequence (first 10 numbers)");
    let mut a = 0;
    let mut b = 1;
    let mut count = 0;
    
    print!("Fibonacci: ");
    while count < 10 {
        print!("{} ", a);
        let temp = a;
        a = b;
        b = temp + b;
        count += 1;
    }
    println!();
}

// 3. For Loop Demonstration
fn demonstrate_for_loop() {
    println!("\n=== For Loop ===\n");
    
    println!("Example 1: Iterate through a range");
    print!("Numbers 1-5: ");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 2: Iterate through an array");
    let fruits = ["Apple", "Banana", "Orange", "Mango", "Grape"];
    println!("Fruits:");
    for fruit in fruits {
        println!("  - {}", fruit);
    }
    
    println!("\nExample 3: Iterate with index");
    println!("Indexed fruits:");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("  {}. {}", index + 1, fruit);
    }
    
    println!("\nExample 4: Iterate through a vector");
    let numbers = vec![10, 20, 30, 40, 50];
    let mut sum = 0;
    for &num in &numbers {
        sum += num;
    }
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    
    println!("\nExample 5: Modify vector elements");
    let mut values = vec![1, 2, 3, 4, 5];
    println!("Original: {:?}", values);
    for value in &mut values {
        *value *= 2;
    }
    println!("Doubled:  {:?}", values);
    
    println!("\nExample 6: Multiplication table");
    let n = 5;
    println!("Multiplication table for {}:", n);
    for i in 1..=10 {
        println!("{} × {} = {}", n, i, n * i);
    }
}

// 4. Loop Control Demonstration
fn demonstrate_loop_control() {
    println!("\n=== Loop Control (break & continue) ===\n");
    
    println!("Example 1: Using 'break' to exit early");
    print!("Numbers until 5: ");
    for i in 1..=10 {
        if i > 5 {
            break;
        }
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 2: Using 'continue' to skip iterations");
    print!("Odd numbers 1-10: ");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 3: Skip multiples of 3");
    print!("Non-multiples of 3 (1-15): ");
    for i in 1..=15 {
        if i % 3 == 0 {
            continue;
        }
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 4: Find first even number");
    let numbers = vec![1, 3, 5, 7, 8, 9, 11, 12];
    let mut first_even = None;
    
    for &num in &numbers {
        if num % 2 == 0 {
            first_even = Some(num);
            break;
        }
    }
    
    match first_even {
        Some(n) => println!("First even number: {}", n),
        None => println!("No even number found"),
    }
    
    println!("\nExample 5: Process until condition");
    let mut total = 0;
    for i in 1..=100 {
        total += i;
        if total > 100 {
            println!("Sum exceeded 100 at i = {}", i);
            println!("Total: {}", total);
            break;
        }
    }
}

// 5. Nested Loops Demonstration
fn demonstrate_nested_loops() {
    println!("\n=== Nested Loops ===\n");
    
    println!("Example 1: Multiplication table");
    println!("     1   2   3   4   5");
    println!("   ---------------------");
    for i in 1..=5 {
        print!("{} | ", i);
        for j in 1..=5 {
            print!("{:3} ", i * j);
        }
        println!();
    }
    
    println!("\nExample 2: Right triangle pattern");
    for i in 1..=5 {
        for j in 1..=i {
            print!("* ");
        }
        println!();
    }
    
    println!("\nExample 3: Number pyramid");
    for i in 1..=5 {
        // Print spaces
        for _ in 0..(5 - i) {
            print!(" ");
        }
        // Print numbers
        for j in 1..=i {
            print!("{} ", j);
        }
        println!();
    }
    
    println!("\nExample 4: Matrix traversal");
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];
    
    println!("Matrix:");
    for row in &matrix {
        for &val in row {
            print!("{:3} ", val);
        }
        println!();
    }
    
    println!("\nExample 5: Finding pairs that sum to 10");
    let numbers = vec![2, 3, 5, 7, 8];
    println!("Numbers: {:?}", numbers);
    println!("Pairs that sum to 10:");
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            if numbers[i] + numbers[j] == 10 {
                println!("  {} + {} = 10", numbers[i], numbers[j]);
            }
        }
    }
}

// 6. Loop Labels Demonstration
fn demonstrate_loop_labels() {
    println!("\n=== Loop Labels ===\n");
    
    println!("Example 1: Breaking outer loop");
    'outer: for x in 0..5 {
        for y in 0..5 {
            println!("({}, {})", x, y);
            if x == 2 && y == 2 {
                println!("Breaking outer loop at ({}, {})", x, y);
                break 'outer;
            }
        }
    }
    
    println!("\nExample 2: Continuing outer loop");
    'outer: for x in 1..=3 {
        println!("\nOuter loop iteration: {}", x);
        for y in 1..=3 {
            print!("  Inner ({}, {}) ", x, y);
            if y == 2 {
                println!("- continuing outer");
                continue 'outer;
            }
            println!();
        }
    }
    
    println!("\nExample 3: Matrix search with early exit");
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    let target = 7;
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
    
    println!("\nExample 4: Multiple labeled loops");
    'first: loop {
        println!("First loop");
        'second: loop {
            println!("  Second loop");
            'third: loop {
                println!("    Third loop - breaking first");
                break 'first;
            }
        }
    }
    println!("Exited all loops");
}

// 7. Returning Values from Loops
fn demonstrate_returning_values() {
    println!("\n=== Returning Values from Loops ===\n");
    
    println!("Example 1: Return counter value");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Counter reached 10, result: {}", result);
    
    println!("\nExample 2: Find first number divisible by 7");
    let numbers = vec![1, 3, 5, 8, 12, 14, 19, 21];
    let result = loop {
        let mut index = 0;
        let found = loop {
            if index >= numbers.len() {
                break None;
            }
            if numbers[index] % 7 == 0 {
                break Some(numbers[index]);
            }
            index += 1;
        };
        break found;
    };
    
    match result {
        Some(n) => println!("First number divisible by 7: {}", n),
        None => println!("No number divisible by 7 found"),
    }
    
    println!("\nExample 3: Compute factorial with loop");
    let n = 5;
    let factorial = {
        let mut result = 1;
        let mut i = 1;
        loop {
            if i > n {
                break result;
            }
            result *= i;
            i += 1;
        }
    };
    println!("{}! = {}", n, factorial);
    
    println!("\nExample 4: Find sum exceeding threshold");
    let threshold = 50;
    let sum = loop {
        let mut total = 0;
        let mut i = 1;
        loop {
            total += i;
            if total > threshold {
                break total;
            }
            i += 1;
        }
        break total;
    };
    println!("First sum exceeding {}: {}", threshold, sum);
}

// 8. Range Loops Demonstration
fn demonstrate_range_loops() {
    println!("\n=== Range Loops ===\n");
    
    println!("Example 1: Exclusive range (0..5)");
    print!("Values: ");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 2: Inclusive range (0..=5)");
    print!("Values: ");
    for i in 0..=5 {
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 3: Reverse range");
    print!("Countdown: ");
    for i in (1..=10).rev() {
        print!("{} ", i);
    }
    println!("Blastoff! 🚀");
    
    println!("\nExample 4: Step by (using step_by)");
    print!("Even numbers 0-20: ");
    for i in (0..=20).step_by(2) {
        print!("{} ", i);
    }
    println!();
    
    println!("\nExample 5: Range with char");
    print!("Lowercase letters a-j: ");
    for ch in 'a'..='j' {
        print!("{} ", ch);
    }
    println!();
    
    println!("\nExample 6: Creating collections from ranges");
    let vec1: Vec<i32> = (1..=10).collect();
    println!("Vector from range: {:?}", vec1);
    
    let vec2: Vec<i32> = (0..20).step_by(3).collect();
    println!("Every 3rd number: {:?}", vec2);
    
    println!("\nExample 7: Sum using range");
    let sum: i32 = (1..=100).sum();
    println!("Sum of 1 to 100: {}", sum);
    
    println!("\nExample 8: Product using range");
    let product: i32 = (1..=5).product();
    println!("Product of 1 to 5 (5!): {}", product);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_basics() {
        let mut count = 0;
        loop {
            count += 1;
            if count >= 5 {
                break;
            }
        }
        assert_eq!(count, 5);
    }
    
    #[test]
    fn test_while_loop() {
        let mut number = 0;
        while number < 10 {
            number += 1;
        }
        assert_eq!(number, 10);
    }
    
    #[test]
    fn test_for_loop() {
        let mut sum = 0;
        for i in 1..=10 {
            sum += i;
        }
        assert_eq!(sum, 55);
    }
    
    #[test]
    fn test_break_continue() {
        let mut evens = Vec::new();
        for i in 1..=10 {
            if i % 2 != 0 {
                continue;
            }
            evens.push(i);
            if evens.len() >= 3 {
                break;
            }
        }
        assert_eq!(evens, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_return_from_loop() {
        let result = loop {
            break 42;
        };
        assert_eq!(result, 42);
    }
    
    #[test]
    fn test_range_operations() {
        let sum: i32 = (1..=10).sum();
        assert_eq!(sum, 55);
        
        let product: i32 = (1..=5).product();
        assert_eq!(product, 120);
    }
}
