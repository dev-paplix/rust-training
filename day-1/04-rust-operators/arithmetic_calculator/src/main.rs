// Arithmetic Calculator
// Demonstrates arithmetic operators and overflow handling

use std::io::{self, Write};

fn main() {
    println!("=== Arithmetic Calculator ===\n");
    
    loop {
        println!("\nChoose an operation:");
        println!("1. Basic Arithmetic");
        println!("2. Advanced Operations");
        println!("3. Overflow Handling Demo");
        println!("4. Type Conversion Demo");
        println!("5. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => basic_arithmetic(),
            2 => advanced_operations(),
            3 => overflow_handling_demo(),
            4 => type_conversion_demo(),
            5 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_arithmetic() {
    println!("\n--- Basic Arithmetic ---");
    
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let a = read_float();
    
    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let b = read_float();
    
    println!("\nResults:");
    println!("  {} + {} = {}", a, b, a + b);
    println!("  {} - {} = {}", a, b, a - b);
    println!("  {} * {} = {}", a, b, a * b);
    
    if b != 0.0 {
        println!("  {} / {} = {}", a, b, a / b);
        
        // Modulo only for integers
        let a_int = a as i32;
        let b_int = b as i32;
        if b_int != 0 {
            println!("  {} % {} = {}", a_int, b_int, a_int % b_int);
        }
    } else {
        println!("  Division by zero not allowed!");
    }
}

fn advanced_operations() {
    println!("\n--- Advanced Operations ---");
    
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let x = read_float();
    
    println!("\nResults:");
    println!("  Absolute value: {}", x.abs());
    println!("  Square: {}", x.powi(2));
    println!("  Square root: {}", x.sqrt());
    println!("  Cube: {}", x.powi(3));
    println!("  Cube root: {}", x.cbrt());
    
    if x > 0.0 {
        println!("  Natural log: {}", x.ln());
        println!("  Log base 10: {}", x.log10());
        println!("  Log base 2: {}", x.log2());
    }
    
    println!("  Floor: {}", x.floor());
    println!("  Ceiling: {}", x.ceil());
    println!("  Round: {}", x.round());
    println!("  Truncate: {}", x.trunc());
}

fn overflow_handling_demo() {
    println!("\n--- Overflow Handling Demo ---");
    
    println!("\nDemonstrating different overflow behaviors:");
    
    let a: u8 = 250;
    let b: u8 = 10;
    
    println!("\nStarting values: a = {}, b = {}", a, b);
    println!("Attempting a + b (would be 260, but u8 max is 255):\n");
    
    // 1. Checked arithmetic
    match a.checked_add(b) {
        Some(result) => println!("  ✅ Checked: {}", result),
        None => println!("  ❌ Checked: Overflow detected!"),
    }
    
    // 2. Saturating arithmetic
    let saturated = a.saturating_add(b);
    println!("  📊 Saturating: {} (clamped at u8::MAX)", saturated);
    
    // 3. Wrapping arithmetic
    let wrapped = a.wrapping_add(b);
    println!("  🔄 Wrapping: {} (wrapped around)", wrapped);
    
    // 4. Overflowing arithmetic
    let (result, overflow) = a.overflowing_add(b);
    println!("  ⚡ Overflowing: result = {}, overflow = {}", result, overflow);
    
    // Division by zero
    println!("\n--- Division by Zero Handling ---");
    let numerator = 10;
    let denominator = 0;
    
    match numerator.checked_div(denominator) {
        Some(result) => println!("  Result: {}", result),
        None => println!("  ❌ Cannot divide by zero!"),
    }
}

fn type_conversion_demo() {
    println!("\n--- Type Conversion Demo ---");
    
    print!("Enter an integer: ");
    io::stdout().flush().unwrap();
    let int_val = read_number();
    
    println!("\nConversions:");
    println!("  As i8: {}", int_val as i8);
    println!("  As i16: {}", int_val as i16);
    println!("  As i32: {}", int_val);
    println!("  As i64: {}", int_val as i64);
    println!("  As f32: {}", int_val as f32);
    println!("  As f64: {}", int_val as f64);
    
    // Float to int (truncation)
    let float_val = 3.14159;
    println!("\nFloat {} converted to int: {}", float_val, float_val as i32);
    
    // Character conversions
    let ch = 'A';
    println!("\nCharacter '{}' as u8: {}", ch, ch as u8);
    
    let code = 65u8;
    println!("Byte {} as char: '{}'", code, code as char);
    
    // Demonstrating precision loss
    println!("\n--- Precision Loss Warning ---");
    let large: i64 = 1234567890123456789;
    let as_float = large as f64;
    let back_to_int = as_float as i64;
    println!("  Original:      {}", large);
    println!("  As f64:        {}", as_float);
    println!("  Back to i64:   {}", back_to_int);
    println!("  Loss:          {}", large - back_to_int);
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_float() -> f64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(5 + 3, 8);
        assert_eq!(5 - 3, 2);
        assert_eq!(5 * 3, 15);
        assert_eq!(6 / 3, 2);
        assert_eq!(5 % 3, 2);
    }
    
    #[test]
    fn test_overflow_handling() {
        let a: u8 = 250;
        let b: u8 = 10;
        
        assert_eq!(a.checked_add(b), None);
        assert_eq!(a.saturating_add(b), 255);
        assert_eq!(a.wrapping_add(b), 4);
    }
    
    #[test]
    fn test_type_conversion() {
        let x: i32 = 42;
        assert_eq!(x as f64, 42.0);
        
        let y: f64 = 3.14;
        assert_eq!(y as i32, 3);
    }
}
