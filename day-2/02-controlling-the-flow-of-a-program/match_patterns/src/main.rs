// Match Pattern Demonstration
// Shows pattern matching with match expressions

use std::io::{self, Write};

fn main() {
    println!("=== Match Pattern Demonstration ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Basic Match");
        println!("2. Match with Ranges");
        println!("3. Match Guards");
        println!("4. Destructuring");
        println!("5. Enum Matching");
        println!("6. Practical Examples");
        println!("7. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => basic_match(),
            2 => match_ranges(),
            3 => match_guards(),
            4 => destructuring(),
            5 => enum_matching(),
            6 => practical_examples(),
            7 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_match() {
    println!("\n=== Basic Match ===");
    
    print!("Enter a number (1-5): ");
    io::stdout().flush().unwrap();
    let number = read_number();
    
    // Basic match
    let description = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        _ => "other",
    };
    
    println!("✅ You entered: {}", description);
    
    // Match with multiple patterns
    let category = match number {
        1 | 2 => "low",
        3 => "medium",
        4 | 5 => "high",
        _ => "out of range",
    };
    
    println!("✅ Category: {}", category);
    
    // Match with expressions
    let result = match number {
        n if n < 1 => format!("Too small: {}", n),
        1..=3 => String::from("Small number"),
        4..=5 => String::from("Larger number"),
        _ => format!("Too large: {}", number),
    };
    
    println!("✅ Result: {}", result);
    
    println!("\n💡 Note: Match must be exhaustive (cover all cases)");
    println!("💡 Note: Use _ for catch-all pattern");
}

fn match_ranges() {
    println!("\n=== Match with Ranges ===");
    
    print!("Enter a test score (0-100): ");
    io::stdout().flush().unwrap();
    let score = read_number();
    
    // Match with ranges
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => {
            println!("⚠️  Invalid score!");
            '?'
        }
    };
    
    if grade != '?' {
        println!("✅ Grade: {}", grade);
    }
    
    // Character ranges
    print!("\nEnter a character: ");
    io::stdout().flush().unwrap();
    let ch = read_string().chars().next().unwrap_or(' ');
    
    let char_type = match ch {
        'a'..='z' => "lowercase letter",
        'A'..='Z' => "uppercase letter",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' => "whitespace",
        _ => "other character",
    };
    
    println!("✅ Character type: {}", char_type);
    
    // Age categories
    print!("\nEnter age: ");
    io::stdout().flush().unwrap();
    let age = read_number();
    
    let stage = match age {
        0..=2 => "Infant",
        3..=12 => "Child",
        13..=19 => "Teenager",
        20..=39 => "Young Adult",
        40..=59 => "Middle Age",
        60..=120 => "Senior",
        _ => "Invalid age",
    };
    
    println!("✅ Life stage: {}", stage);
}

fn match_guards() {
    println!("\n=== Match Guards ===");
    println!("Guards add conditions to patterns\n");
    
    print!("Enter a number: ");
    io::stdout().flush().unwrap();
    let number = read_number();
    
    print!("Is it special? (yes/no): ");
    io::stdout().flush().unwrap();
    let is_special = read_string().to_lowercase() == "yes";
    
    // Match with guards
    let description = match number {
        n if is_special => format!("{} is special! ⭐", n),
        n if n < 0 => format!("{} is negative", n),
        0 => String::from("zero"),
        n if n % 2 == 0 => format!("{} is even", n),
        n if n % 3 == 0 => format!("{} is divisible by 3", n),
        n if n % 5 == 0 => format!("{} is divisible by 5", n),
        n => format!("{} is odd", n),
    };
    
    println!("✅ {}", description);
    
    // Complex guards
    print!("\nEnter x coordinate: ");
    io::stdout().flush().unwrap();
    let x = read_number();
    
    print!("Enter y coordinate: ");
    io::stdout().flush().unwrap();
    let y = read_number();
    
    let quadrant = match (x, y) {
        (0, 0) => "Origin",
        (x, 0) if x > 0 => "Positive X-axis",
        (x, 0) if x < 0 => "Negative X-axis",
        (0, y) if y > 0 => "Positive Y-axis",
        (0, y) if y < 0 => "Negative Y-axis",
        (x, y) if x > 0 && y > 0 => "Quadrant I",
        (x, y) if x < 0 && y > 0 => "Quadrant II",
        (x, y) if x < 0 && y < 0 => "Quadrant III",
        (x, y) if x > 0 && y < 0 => "Quadrant IV",
        _ => "Unknown",
    };
    
    println!("✅ Point ({}, {}) is in: {}", x, y, quadrant);
}

fn destructuring() {
    println!("\n=== Destructuring ===");
    
    // Tuple destructuring
    print!("Enter x: ");
    io::stdout().flush().unwrap();
    let x = read_number();
    
    print!("Enter y: ");
    io::stdout().flush().unwrap();
    let y = read_number();
    
    let point = (x, y);
    
    match point {
        (0, 0) => println!("✅ Origin"),
        (0, y) => println!("✅ On Y-axis at y={}", y),
        (x, 0) => println!("✅ On X-axis at x={}", x),
        (x, y) if x == y => println!("✅ On diagonal at ({}, {})", x, y),
        (x, y) => println!("✅ General point at ({}, {})", x, y),
    }
    
    // Array destructuring
    print!("\nEnter 3 numbers (space-separated): ");
    io::stdout().flush().unwrap();
    let input = read_string();
    let numbers: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if numbers.len() >= 3 {
        match &numbers[..] {
            [first, .., last] if first == last => {
                println!("✅ First and last are equal: {}", first);
            }
            [first, middle, last] => {
                println!("✅ First: {}, Middle: {}, Last: {}", first, middle, last);
            }
            _ => println!("Pattern not matched"),
        }
    }
    
    // Struct-like destructuring
    print!("\nEnter RGB values (space-separated, 0-255): ");
    io::stdout().flush().unwrap();
    let input = read_string();
    let rgb: Vec<u8> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if rgb.len() >= 3 {
        match (rgb[0], rgb[1], rgb[2]) {
            (255, 0, 0) => println!("✅ Pure Red"),
            (0, 255, 0) => println!("✅ Pure Green"),
            (0, 0, 255) => println!("✅ Pure Blue"),
            (r, g, b) if r == g && g == b => println!("✅ Grayscale: {}", r),
            (r, g, b) => println!("✅ Color RGB({}, {}, {})", r, g, b),
        }
    }
}

fn enum_matching() {
    println!("\n=== Enum Matching ===");
    
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(u8, u8, u8),
    }
    
    println!("\nSelect message type:");
    println!("1. Quit");
    println!("2. Move");
    println!("3. Write");
    println!("4. ChangeColor");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    let message = match choice {
        1 => Message::Quit,
        2 => {
            print!("Enter x: ");
            io::stdout().flush().unwrap();
            let x = read_number();
            print!("Enter y: ");
            io::stdout().flush().unwrap();
            let y = read_number();
            Message::Move { x, y }
        }
        3 => {
            print!("Enter text: ");
            io::stdout().flush().unwrap();
            let text = read_string();
            Message::Write(text)
        }
        4 => {
            print!("Enter R G B (space-separated): ");
            io::stdout().flush().unwrap();
            let input = read_string();
            let rgb: Vec<u8> = input
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            Message::ChangeColor(
                *rgb.get(0).unwrap_or(&0),
                *rgb.get(1).unwrap_or(&0),
                *rgb.get(2).unwrap_or(&0),
            )
        }
        _ => Message::Quit,
    };
    
    // Match on enum
    match message {
        Message::Quit => {
            println!("✅ Quit message received");
        }
        Message::Move { x, y } => {
            println!("✅ Move to position ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("✅ Write message: '{}'", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("✅ Change color to RGB({}, {}, {})", r, g, b);
        }
    }
    
    // Option matching
    print!("\nEnter a number (or 'none'): ");
    io::stdout().flush().unwrap();
    let input = read_string();
    
    let option: Option<i32> = if input == "none" {
        None
    } else {
        input.parse().ok()
    };
    
    match option {
        Some(n) if n < 0 => println!("✅ Negative number: {}", n),
        Some(n) if n > 100 => println!("✅ Large number: {}", n),
        Some(n) => println!("✅ Number: {}", n),
        None => println!("✅ No value"),
    }
}

fn practical_examples() {
    println!("\n=== Practical Examples ===");
    
    println!("\n1. HTTP Status Code Handler");
    print!("Enter HTTP status code: ");
    io::stdout().flush().unwrap();
    let status = read_number();
    
    match status {
        200 => println!("   ✅ OK"),
        201 => println!("   ✅ Created"),
        204 => println!("   ✅ No Content"),
        301 | 302 => println!("   ↪️  Redirect"),
        400 => println!("   ❌ Bad Request"),
        401 => println!("   ❌ Unauthorized"),
        403 => println!("   ❌ Forbidden"),
        404 => println!("   ❌ Not Found"),
        500 => println!("   ❌ Internal Server Error"),
        502 => println!("   ❌ Bad Gateway"),
        503 => println!("   ❌ Service Unavailable"),
        200..=299 => println!("   ✅ Success"),
        300..=399 => println!("   ↪️  Redirection"),
        400..=499 => println!("   ❌ Client Error"),
        500..=599 => println!("   ❌ Server Error"),
        _ => println!("   ❓ Unknown Status Code"),
    }
    
    println!("\n2. Day of Week");
    print!("Enter day number (1-7): ");
    io::stdout().flush().unwrap();
    let day = read_number();
    
    let (day_name, is_weekend) = match day {
        1 => ("Monday", false),
        2 => ("Tuesday", false),
        3 => ("Wednesday", false),
        4 => ("Thursday", false),
        5 => ("Friday", false),
        6 => ("Saturday", true),
        7 => ("Sunday", true),
        _ => ("Invalid", false),
    };
    
    println!("   {}", day_name);
    if is_weekend {
        println!("   🎉 It's the weekend!");
    }
    
    println!("\n3. Calculator");
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let a = read_number();
    
    print!("Enter operator (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let op = read_string();
    
    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let b = read_number();
    
    match op.as_str() {
        "+" => println!("   {} + {} = {}", a, b, a + b),
        "-" => println!("   {} - {} = {}", a, b, a - b),
        "*" => println!("   {} × {} = {}", a, b, a * b),
        "/" if b != 0 => println!("   {} ÷ {} = {}", a, b, a / b),
        "/" => println!("   ❌ Division by zero!"),
        _ => println!("   ❌ Unknown operator"),
    }
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
    fn test_basic_match() {
        let result = match 3 {
            1 => "one",
            2 => "two",
            3 => "three",
            _ => "other",
        };
        assert_eq!(result, "three");
    }
    
    #[test]
    fn test_range_match() {
        let grade = match 85 {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            _ => 'F',
        };
        assert_eq!(grade, 'B');
    }
    
    #[test]
    fn test_guard_match() {
        let x = 10;
        let result = match x {
            n if n < 0 => "negative",
            0 => "zero",
            n if n % 2 == 0 => "even",
            _ => "odd",
        };
        assert_eq!(result, "even");
    }
}
