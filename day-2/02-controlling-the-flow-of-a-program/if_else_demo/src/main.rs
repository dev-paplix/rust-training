// If/Else Demonstration
// Shows conditional branching and if expressions

use std::io::{self, Write};

fn main() {
    println!("=== If/Else Demonstration ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Basic If/Else");
        println!("2. If Expressions (Return Values)");
        println!("3. Multiple Conditions");
        println!("4. If Let Pattern");
        println!("5. Nested Conditions");
        println!("6. Practical Examples");
        println!("7. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => basic_if_else(),
            2 => if_expressions(),
            3 => multiple_conditions(),
            4 => if_let_pattern(),
            5 => nested_conditions(),
            6 => practical_examples(),
            7 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_if_else() {
    println!("\n=== Basic If/Else ===");
    
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let number = read_number();
    
    // Simple if
    if number > 0 {
        println!("✅ The number is positive");
    }
    
    // If/else
    if number % 2 == 0 {
        println!("✅ The number is even");
    } else {
        println!("✅ The number is odd");
    }
    
    // No parentheses around condition in Rust
    println!("\n💡 Note: Rust doesn't require () around conditions");
    println!("💡 Note: Braces {{}} are always required");
}

fn if_expressions() {
    println!("\n=== If Expressions ===");
    println!("In Rust, if is an expression that returns a value\n");
    
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let number = read_number();
    
    // If expression returns a value
    let description = if number < 0 {
        "negative"
    } else if number > 0 {
        "positive"
    } else {
        "zero"
    };
    
    println!("✅ The number is {}", description);
    
    // More complex expression
    let category = if number.abs() > 100 {
        "large"
    } else if number.abs() > 10 {
        "medium"
    } else {
        "small"
    };
    
    println!("✅ Magnitude: {}", category);
    
    // Inline usage
    let doubled = if number > 0 { number * 2 } else { 0 };
    println!("✅ Doubled (if positive): {}", doubled);
    
    println!("\n💡 Note: Both branches must return the same type");
}

fn multiple_conditions() {
    println!("\n=== Multiple Conditions ===");
    
    print!("Enter a number (0-100): ");
    io::stdout().flush().unwrap();
    let score = read_number();
    
    // Grade calculator
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };
    
    println!("✅ Grade: {}", grade);
    
    // Categorize
    let category = if score < 0 {
        "Invalid (negative)"
    } else if score == 0 {
        "Zero"
    } else if score < 50 {
        "Failing"
    } else if score < 70 {
        "Passing"
    } else if score < 90 {
        "Good"
    } else if score <= 100 {
        "Excellent"
    } else {
        "Invalid (too high)"
    };
    
    println!("✅ Category: {}", category);
    
    // Combined conditions
    print!("\nEnter age: ");
    io::stdout().flush().unwrap();
    let age = read_number();
    
    if age >= 18 && score >= 60 {
        println!("✅ Eligible for program");
    } else if age < 18 {
        println!("❌ Too young");
    } else {
        println!("❌ Score too low");
    }
}

fn if_let_pattern() {
    println!("\n=== If Let Pattern ===");
    println!("Use if let for single pattern matching\n");
    
    // Working with Option
    print!("Enter a number (or 'none' for None): ");
    io::stdout().flush().unwrap();
    let input = read_string();
    
    let optional: Option<i32> = if input == "none" {
        None
    } else {
        input.parse().ok()
    };
    
    // Using if let
    if let Some(value) = optional {
        println!("✅ Got value: {}", value);
        println!("   Doubled: {}", value * 2);
    } else {
        println!("❌ No value provided");
    }
    
    // Working with Result
    print!("\nEnter a number to parse: ");
    io::stdout().flush().unwrap();
    let input = read_string();
    let result: Result<i32, _> = input.parse();
    
    if let Ok(number) = result {
        println!("✅ Successfully parsed: {}", number);
    } else {
        println!("❌ Failed to parse");
    }
    
    // Multiple patterns
    print!("\nEnter a word: ");
    io::stdout().flush().unwrap();
    let word = read_string();
    
    if let "hello" | "hi" | "hey" = word.as_str() {
        println!("✅ Greeting detected!");
    } else {
        println!("ℹ️  Not a greeting");
    }
}

fn nested_conditions() {
    println!("\n=== Nested Conditions ===");
    
    print!("Enter your age: ");
    io::stdout().flush().unwrap();
    let age = read_number();
    
    print!("Do you have a license? (yes/no): ");
    io::stdout().flush().unwrap();
    let has_license = read_string().to_lowercase() == "yes";
    
    print!("Enter years of experience: ");
    io::stdout().flush().unwrap();
    let experience = read_number();
    
    // Nested conditions
    if age >= 18 {
        if has_license {
            if experience >= 2 {
                println!("✅ Can rent a car");
            } else {
                println!("⚠️  Can rent with additional fee (< 2 years experience)");
            }
        } else {
            println!("❌ Need a license");
        }
    } else {
        println!("❌ Must be 18 or older");
    }
    
    println!("\n💡 Better: Use early returns or match to avoid deep nesting");
    
    // Better version with early return logic (shown conceptually)
    let can_rent = age >= 18 && has_license;
    let message = if !can_rent {
        "❌ Cannot rent (age or license)"
    } else if experience >= 2 {
        "✅ Can rent a car"
    } else {
        "⚠️  Can rent with additional fee"
    };
    
    println!("Flattened: {}", message);
}

fn practical_examples() {
    println!("\n=== Practical Examples ===");
    
    println!("\n1. Temperature Checker");
    print!("Enter temperature (°C): ");
    io::stdout().flush().unwrap();
    let temp = read_number();
    
    let status = if temp < 0 {
        "Freezing ❄️"
    } else if temp < 10 {
        "Cold 🧊"
    } else if temp < 20 {
        "Cool 🌤️"
    } else if temp < 30 {
        "Warm ☀️"
    } else {
        "Hot 🔥"
    };
    
    println!("   {}", status);
    
    println!("\n2. Password Strength");
    print!("Enter password: ");
    io::stdout().flush().unwrap();
    let password = read_string();
    
    let length = password.len();
    let has_number = password.chars().any(|c| c.is_numeric());
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_lower = password.chars().any(|c| c.is_lowercase());
    
    let strength = if length < 6 {
        "❌ Too short"
    } else if !has_number {
        "⚠️  Weak (add numbers)"
    } else if !has_upper || !has_lower {
        "⚠️  Medium (add mixed case)"
    } else if length < 10 {
        "✅ Good"
    } else {
        "✅ Strong"
    };
    
    println!("   Strength: {}", strength);
    
    println!("\n3. Discount Calculator");
    print!("Enter purchase amount: $");
    io::stdout().flush().unwrap();
    let amount = read_number();
    
    print!("Are you a member? (yes/no): ");
    io::stdout().flush().unwrap();
    let is_member = read_string().to_lowercase() == "yes";
    
    let discount = if amount >= 100 {
        if is_member { 20 } else { 15 }
    } else if amount >= 50 {
        if is_member { 10 } else { 5 }
    } else {
        if is_member { 5 } else { 0 }
    };
    
    let final_amount = amount - (amount * discount / 100);
    
    println!("   Discount: {}%", discount);
    println!("   Final price: ${}", final_amount);
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
    fn test_if_expression() {
        let num = 5;
        let result = if num > 0 { "positive" } else { "non-positive" };
        assert_eq!(result, "positive");
    }
    
    #[test]
    fn test_grade_calculation() {
        let grade = if 95 >= 90 { 'A' } else { 'B' };
        assert_eq!(grade, 'A');
        
        let grade = if 75 >= 90 { 'A' } else if 75 >= 70 { 'C' } else { 'F' };
        assert_eq!(grade, 'C');
    }
    
    #[test]
    fn test_if_let() {
        let opt = Some(42);
        let mut found = false;
        
        if let Some(x) = opt {
            assert_eq!(x, 42);
            found = true;
        }
        
        assert!(found);
    }
}
