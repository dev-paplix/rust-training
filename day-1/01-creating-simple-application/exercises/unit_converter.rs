use std::io;

// Exercise 5: Unit Converter
// Converts between different units of measurement

fn main() {
    loop {
        println!("\n=== Unit Converter ===");
        println!("Select conversion type:");
        println!("1. Kilometers ↔ Miles");
        println!("2. Kilograms ↔ Pounds");
        println!("3. Celsius ↔ Fahrenheit");
        println!("4. Exit");
        println!("Enter your choice (1-4):");
        
        let choice = read_number() as u32;
        
        match choice {
            1 => distance_converter(),
            2 => weight_converter(),
            3 => temperature_converter(),
            4 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn distance_converter() {
    println!("\n--- Distance Converter ---");
    println!("1. Kilometers to Miles");
    println!("2. Miles to Kilometers");
    println!("Enter your choice:");
    
    let choice = read_number() as u32;
    println!("Enter the value:");
    let value = read_number();
    
    match choice {
        1 => {
            let miles = value * 0.621371;
            println!("{} km = {:.2} miles", value, miles);
        }
        2 => {
            let km = value * 1.60934;
            println!("{} miles = {:.2} km", value, km);
        }
        _ => println!("Invalid choice!"),
    }
}

fn weight_converter() {
    println!("\n--- Weight Converter ---");
    println!("1. Kilograms to Pounds");
    println!("2. Pounds to Kilograms");
    println!("Enter your choice:");
    
    let choice = read_number() as u32;
    println!("Enter the value:");
    let value = read_number();
    
    match choice {
        1 => {
            let pounds = value * 2.20462;
            println!("{} kg = {:.2} lbs", value, pounds);
        }
        2 => {
            let kg = value * 0.453592;
            println!("{} lbs = {:.2} kg", value, kg);
        }
        _ => println!("Invalid choice!"),
    }
}

fn temperature_converter() {
    println!("\n--- Temperature Converter ---");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    println!("Enter your choice:");
    
    let choice = read_number() as u32;
    println!("Enter the temperature:");
    let temp = read_number();
    
    match choice {
        1 => {
            let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
            println!("{:.1}°C = {:.1}°F", temp, fahrenheit);
        }
        2 => {
            let celsius = (temp - 32.0) * 5.0 / 9.0;
            println!("{:.1}°F = {:.1}°C", temp, celsius);
        }
        _ => println!("Invalid choice!"),
    }
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().parse().unwrap_or(0.0)
}
