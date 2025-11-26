use std::io;

// Exercise 4: Fibonacci Sequence Generator
// Generates the Fibonacci sequence up to n terms

fn main() {
    println!("=== Fibonacci Sequence Generator ===");
    println!("How many Fibonacci numbers do you want to generate?");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let n: u32 = match input.trim().parse() {
        Ok(num) if num > 0 => num,
        _ => {
            println!("Please enter a positive number!");
            return;
        }
    };
    
    println!("\nFirst {} Fibonacci numbers:", n);
    println!("─────────────────────────────");
    
    generate_fibonacci(n);
}

fn generate_fibonacci(n: u32) {
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    
    for i in 1..=n {
        print!("{}", a);
        
        if i < n {
            print!(", ");
        }
        
        // Calculate next Fibonacci number
        let next = a + b;
        a = b;
        b = next;
        
        // Add newline every 10 numbers for readability
        if i % 10 == 0 {
            println!();
        }
    }
    
    println!("\n─────────────────────────────");
}

// Alternative recursive implementation (less efficient)
#[allow(dead_code)]
fn fibonacci_recursive(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_fibonacci_recursive() {
        assert_eq!(fibonacci_recursive(0), 0);
        assert_eq!(fibonacci_recursive(1), 1);
        assert_eq!(fibonacci_recursive(2), 1);
        assert_eq!(fibonacci_recursive(3), 2);
        assert_eq!(fibonacci_recursive(4), 3);
        assert_eq!(fibonacci_recursive(5), 5);
        assert_eq!(fibonacci_recursive(10), 55);
    }
}
