// Modular Calculator
// Demonstrates organizing functionality into modules

mod math;

use std::io;
use math::{basic, advanced, statistics};

fn main() {
    loop {
        println!("\n=== Modular Calculator ===");
        println!("1. Basic Operations");
        println!("2. Advanced Operations");
        println!("3. Statistics");
        println!("4. Exit");
        println!("Choose an option:");
        
        let choice = read_input();
        
        match choice.trim() {
            "1" => basic_operations_menu(),
            "2" => advanced_operations_menu(),
            "3" => statistics_menu(),
            "4" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_operations_menu() {
    println!("\n--- Basic Operations ---");
    println!("Enter first number:");
    let a: f64 = read_input().trim().parse().unwrap_or(0.0);
    
    println!("Enter second number:");
    let b: f64 = read_input().trim().parse().unwrap_or(0.0);
    
    println!("\n1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Divide");
    println!("Choose operation:");
    
    let op = read_input();
    
    let result = match op.trim() {
        "1" => basic::add(a, b),
        "2" => basic::subtract(a, b),
        "3" => basic::multiply(a, b),
        "4" => basic::divide(a, b).unwrap_or_else(|| {
            println!("Error: Division by zero!");
            0.0
        }),
        _ => {
            println!("Invalid operation!");
            return;
        }
    };
    
    println!("Result: {}", result);
}

fn advanced_operations_menu() {
    println!("\n--- Advanced Operations ---");
    println!("1. Power");
    println!("2. Square Root");
    println!("3. Factorial");
    println!("Choose operation:");
    
    let op = read_input();
    
    match op.trim() {
        "1" => {
            println!("Enter base:");
            let base: f64 = read_input().trim().parse().unwrap_or(0.0);
            println!("Enter exponent:");
            let exp: f64 = read_input().trim().parse().unwrap_or(0.0);
            println!("Result: {}", advanced::power(base, exp));
        }
        "2" => {
            println!("Enter number:");
            let num: f64 = read_input().trim().parse().unwrap_or(0.0);
            println!("Result: {}", advanced::sqrt(num));
        }
        "3" => {
            println!("Enter number:");
            let num: u32 = read_input().trim().parse().unwrap_or(0);
            println!("Result: {}", advanced::factorial(num));
        }
        _ => println!("Invalid operation!"),
    }
}

fn statistics_menu() {
    println!("\n--- Statistics ---");
    println!("Enter numbers separated by spaces:");
    
    let input = read_input();
    let numbers: Vec<f64> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if numbers.is_empty() {
        println!("No valid numbers entered!");
        return;
    }
    
    println!("\nResults:");
    println!("Mean: {:.2}", statistics::mean(&numbers));
    println!("Median: {:.2}", statistics::median(&numbers));
    println!("Sum: {:.2}", statistics::sum(&numbers));
    println!("Max: {:.2}", statistics::max(&numbers).unwrap_or(0.0));
    println!("Min: {:.2}", statistics::min(&numbers).unwrap_or(0.0));
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input
}
