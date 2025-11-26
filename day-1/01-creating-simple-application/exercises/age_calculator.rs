use std::io;

// Exercise 1: Age Calculator
// Asks for birth year and calculates age in 2025

fn main() {
    println!("=== Age Calculator ===");
    println!("Enter your birth year:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let birth_year: u32 = match input.trim().parse() {
        Ok(year) if year <= 2025 && year >= 1900 => year,
        _ => {
            println!("Invalid year! Please enter a year between 1900 and 2025.");
            return;
        }
    };
    
    let age = 2025 - birth_year;
    
    println!("\nYou are {} years old in 2025!", age);
    
    // Additional information
    if age < 18 {
        println!("You are a minor.");
    } else if age < 65 {
        println!("You are an adult.");
    } else {
        println!("You are a senior citizen.");
    }
    
    // Fun facts
    println!("\nFun Facts:");
    println!("- You were born in the {}s", (birth_year / 10) * 10);
    println!("- You have lived through {} decades", age / 10);
    println!("- Approximate days lived: {}", age * 365);
}

// To run this file directly (not in a Cargo project):
// rustc age_calculator.rs
// ./age_calculator (Linux/Mac) or .\age_calculator.exe (Windows)
