use std::io;

// Exercise 2: Area Calculator
// Calculates area for different shapes

fn main() {
    println!("=== Area Calculator ===");
    println!("Select a shape:");
    println!("1. Circle");
    println!("2. Rectangle");
    println!("3. Triangle");
    println!("Enter your choice (1-3):");
    
    let choice = read_number() as u32;
    
    match choice {
        1 => calculate_circle_area(),
        2 => calculate_rectangle_area(),
        3 => calculate_triangle_area(),
        _ => println!("Invalid choice!"),
    }
}

fn calculate_circle_area() {
    println!("\n--- Circle ---");
    println!("Enter the radius:");
    let radius = read_number();
    
    let area = std::f64::consts::PI * radius * radius;
    println!("Area of circle with radius {}: {:.2}", radius, area);
}

fn calculate_rectangle_area() {
    println!("\n--- Rectangle ---");
    println!("Enter the length:");
    let length = read_number();
    
    println!("Enter the width:");
    let width = read_number();
    
    let area = length * width;
    let perimeter = 2.0 * (length + width);
    
    println!("Area: {:.2}", area);
    println!("Perimeter: {:.2}", perimeter);
}

fn calculate_triangle_area() {
    println!("\n--- Triangle ---");
    println!("Enter the base:");
    let base = read_number();
    
    println!("Enter the height:");
    let height = read_number();
    
    let area = 0.5 * base * height;
    println!("Area: {:.2}", area);
}

fn read_number() -> f64 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    input.trim().parse().unwrap_or(0.0)
}
