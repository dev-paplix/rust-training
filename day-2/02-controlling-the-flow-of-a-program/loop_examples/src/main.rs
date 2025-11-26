// Loop Examples
// Demonstrates loop, while, for, and control flow

use std::io::{self, Write};

fn main() {
    println!("=== Loop Examples ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Loop (Infinite with Break)");
        println!("2. While Loop");
        println!("3. For Loop");
        println!("4. Loop with Return Value");
        println!("5. Labeled Loops");
        println!("6. Break and Continue");
        println!("7. Practical Examples");
        println!("8. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => loop_demo(),
            2 => while_demo(),
            3 => for_demo(),
            4 => loop_return_value(),
            5 => labeled_loops(),
            6 => break_continue(),
            7 => practical_examples(),
            8 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn loop_demo() {
    println!("\n=== Loop Demo ===");
    println!("Infinite loop with break\n");
    
    print!("Enter max count: ");
    io::stdout().flush().unwrap();
    let max = read_number();
    
    let mut counter = 0;
    
    loop {
        counter += 1;
        println!("Count: {}", counter);
        
        if counter >= max {
            println!("✅ Reached maximum!");
            break;
        }
    }
    
    println!("\n💡 Note: loop {} requires explicit break");
    println!("💡 Note: Useful when condition is complex");
}

fn while_demo() {
    println!("\n=== While Loop Demo ===");
    
    print!("Enter starting number: ");
    io::stdout().flush().unwrap();
    let mut number = read_number();
    
    println!("\nCountdown:");
    while number > 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF! 🚀");
    
    // While with complex condition
    print!("\nEnter a number to find first divisor: ");
    io::stdout().flush().unwrap();
    let target = read_number().abs();
    
    let mut divisor = 2;
    let mut found = false;
    
    while divisor <= target / 2 {
        if target % divisor == 0 {
            println!("✅ First divisor: {}", divisor);
            found = true;
            break;
        }
        divisor += 1;
    }
    
    if !found && target > 1 {
        println!("✅ {} is prime!", target);
    }
}

fn for_demo() {
    println!("\n=== For Loop Demo ===");
    
    // Range
    println!("\n1. Range (0..5):");
    for i in 0..5 {
        print!("{} ", i);
    }
    println!();
    
    // Inclusive range
    println!("\n2. Inclusive range (1..=5):");
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();
    
    // Reverse
    println!("\n3. Reverse (5 down to 1):");
    for i in (1..=5).rev() {
        print!("{} ", i);
    }
    println!();
    
    // Array iteration
    println!("\n4. Array iteration:");
    let fruits = ["apple", "banana", "cherry", "date"];
    for fruit in fruits {
        println!("   🍎 {}", fruit);
    }
    
    // With index
    println!("\n5. With enumerate (index + value):");
    for (index, fruit) in fruits.iter().enumerate() {
        println!("   [{}] {}", index, fruit);
    }
    
    // Step by
    println!("\n6. Step by 2:");
    for i in (0..10).step_by(2) {
        print!("{} ", i);
    }
    println!();
    
    // Custom range
    print!("\n7. Enter start: ");
    io::stdout().flush().unwrap();
    let start = read_number();
    
    print!("Enter end: ");
    io::stdout().flush().unwrap();
    let end = read_number();
    
    println!("\nNumbers from {} to {}:", start, end);
    for i in start..=end {
        print!("{} ", i);
    }
    println!();
}

fn loop_return_value() {
    println!("\n=== Loop with Return Value ===");
    
    print!("Enter target number: ");
    io::stdout().flush().unwrap();
    let target = read_number();
    
    let mut counter = 0;
    
    // Loop can return a value with break
    let result = loop {
        counter += 1;
        
        if counter * counter >= target {
            break counter;
        }
        
        if counter > 1000 {
            break -1; // Error value
        }
    };
    
    if result > 0 {
        println!("✅ First number whose square >= {}: {}", target, result);
        println!("   {}² = {}", result, result * result);
    } else {
        println!("❌ Could not find result");
    }
    
    // Another example: finding sum until threshold
    print!("\nEnter sum threshold: ");
    io::stdout().flush().unwrap();
    let threshold = read_number();
    
    let mut sum = 0;
    let mut n = 1;
    
    let final_n = loop {
        sum += n;
        
        if sum >= threshold {
            break n;
        }
        
        n += 1;
    };
    
    println!("✅ Sum reached {} at n={}", sum, final_n);
    println!("   1 + 2 + ... + {} = {}", final_n, sum);
}

fn labeled_loops() {
    println!("\n=== Labeled Loops ===");
    println!("Use labels to control nested loops\n");
    
    print!("Enter grid size: ");
    io::stdout().flush().unwrap();
    let size = read_number().min(10).max(1);
    
    print!("Enter target product: ");
    io::stdout().flush().unwrap();
    let target = read_number();
    
    println!("\nSearching {} × {} grid for product >= {}:", size, size, target);
    
    let mut found = false;
    
    'outer: for x in 1..=size {
        for y in 1..=size {
            let product = x * y;
            
            if product >= target {
                println!("✅ Found: {} × {} = {}", x, y, product);
                found = true;
                break 'outer; // Break outer loop
            }
        }
    }
    
    if !found {
        println!("❌ Not found in grid");
    }
    
    // Multiple labeled loops
    println!("\n3D search example:");
    print!("Enter target sum (x+y+z): ");
    io::stdout().flush().unwrap();
    let target_sum = read_number();
    
    'x_loop: for x in 1..=5 {
        'y_loop: for y in 1..=5 {
            for z in 1..=5 {
                if x + y + z == target_sum {
                    println!("✅ Found: {} + {} + {} = {}", x, y, z, target_sum);
                    break 'x_loop;
                }
            }
        }
    }
}

fn break_continue() {
    println!("\n=== Break and Continue ===");
    
    // Continue - skip to next iteration
    println!("\n1. Continue - Print only odd numbers (1-10):");
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // Skip even numbers
        }
        print!("{} ", i);
    }
    println!();
    
    // Break - exit loop early
    println!("\n2. Break - Stop at first number divisible by 7:");
    for i in 1..=50 {
        if i % 7 == 0 {
            println!("   Found: {}", i);
            break;
        }
    }
    
    // Complex example
    print!("\n3. Enter a sentence: ");
    io::stdout().flush().unwrap();
    let sentence = read_string();
    
    println!("\nWords until first 4-letter word:");
    for word in sentence.split_whitespace() {
        if word.len() == 4 {
            println!("   Stopped at: '{}'", word);
            break;
        }
        println!("   {}", word);
    }
    
    // Continue with condition
    print!("\n4. Enter max value: ");
    io::stdout().flush().unwrap();
    let max = read_number();
    
    println!("\nPerfect squares up to {} (skip numbers ending in 5):", max);
    for i in 1..=max {
        let square = i * i;
        
        if square > max {
            break;
        }
        
        if square % 10 == 5 {
            continue;
        }
        
        println!("   {}² = {}", i, square);
    }
}

fn practical_examples() {
    println!("\n=== Practical Examples ===");
    
    println!("\n1. Number Guessing Game");
    let secret = 42; // In real game, use random number
    let mut attempts = 0;
    let max_attempts = 5;
    
    loop {
        attempts += 1;
        
        if attempts > max_attempts {
            println!("   ❌ Out of attempts! The number was {}", secret);
            break;
        }
        
        print!("   Attempt {}/{}, enter guess: ", attempts, max_attempts);
        io::stdout().flush().unwrap();
        let guess = read_number();
        
        if guess == secret {
            println!("   ✅ Correct! You won in {} attempts!", attempts);
            break;
        } else if guess < secret {
            println!("   📈 Too low!");
        } else {
            println!("   📉 Too high!");
        }
    }
    
    println!("\n2. Sum Calculator");
    let mut sum = 0;
    let mut count = 0;
    
    println!("Enter numbers (0 to stop):");
    loop {
        print!("   Number: ");
        io::stdout().flush().unwrap();
        let num = read_number();
        
        if num == 0 {
            break;
        }
        
        sum += num;
        count += 1;
    }
    
    if count > 0 {
        println!("   Sum: {}", sum);
        println!("   Average: {}", sum as f64 / count as f64);
    }
    
    println!("\n3. Fibonacci Generator");
    print!("How many Fibonacci numbers? ");
    io::stdout().flush().unwrap();
    let n = read_number();
    
    let mut a = 0;
    let mut b = 1;
    
    println!("\nFirst {} Fibonacci numbers:", n);
    for i in 0..n {
        println!("   F({}) = {}", i, a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    
    println!("\n4. Prime Numbers");
    print!("Find primes up to: ");
    io::stdout().flush().unwrap();
    let limit = read_number();
    
    println!("\nPrime numbers up to {}:", limit);
    
    'number: for num in 2..=limit {
        for divisor in 2..=(num as f64).sqrt() as i32 {
            if num % divisor == 0 {
                continue 'number;
            }
        }
        print!("{} ", num);
    }
    println!();
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_loop_with_break() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
    }
    
    #[test]
    fn test_for_range() {
        let mut sum = 0;
        for i in 1..=5 {
            sum += i;
        }
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_while_loop() {
        let mut count = 5;
        let mut result = 0;
        
        while count > 0 {
            result += count;
            count -= 1;
        }
        
        assert_eq!(result, 15);
    }
}
