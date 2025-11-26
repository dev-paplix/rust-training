use std::io;

// Simple Calculator Application
// Performs basic arithmetic operations: +, -, *, /

fn main() {
    println!("=== Simple Calculator ===");
    
    // Get first number
    println!("Enter the first number:");
    let num1 = read_number();
    
    // Get operation
    println!("Enter operation (+, -, *, /):");
    let mut operation = String::new();
    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read operation");
    let operation = operation.trim();
    
    // Get second number
    println!("Enter the second number:");
    let num2 = read_number();
    
    // Perform calculation
    let result = match operation {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            num1 / num2
        }
        _ => {
            println!("Error: Invalid operation! Use +, -, *, or /");
            return;
        }
    };
    
    println!("\nResult: {} {} {} = {}", num1, operation, num2, result);
}

/// Reads a number from user input
fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number! Using 0 as default.");
            0.0
        }
    }
}
