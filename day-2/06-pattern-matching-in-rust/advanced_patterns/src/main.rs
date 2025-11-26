use std::io::{self, Write};

/// Demonstrates advanced pattern matching features
fn main() {
    println!("=== Advanced Pattern Matching Demo ===\n");
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => match_guards_demo(),
            2 => at_bindings_demo(),
            3 => reference_patterns_demo(),
            4 => multiple_patterns_demo(),
            5 => if_let_demo(),
            6 => while_let_demo(),
            7 => complex_matching_demo(),
            8 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
        
        println!();
    }
}

fn display_menu() {
    println!("\n--- Advanced Pattern Matching ---");
    println!("1. Match Guards");
    println!("2. @ Bindings");
    println!("3. Reference Patterns");
    println!("4. Multiple Patterns");
    println!("5. If Let Patterns");
    println!("6. While Let Patterns");
    println!("7. Complex Matching");
    println!("8. Exit");
}

fn get_user_choice() -> u32 {
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Match Guards Demo
fn match_guards_demo() {
    println!("\n--- Match Guards ---");
    
    let input = get_input("Enter a number: ");
    
    match input.parse::<i32>() {
        Ok(num) => {
            // Guard with condition
            match num {
                n if n < 0 => println!("{} is negative", n),
                n if n == 0 => println!("{} is zero", n),
                n if n % 2 == 0 && n > 0 => println!("{} is positive and even", n),
                n if n % 2 != 0 && n > 0 => println!("{} is positive and odd", n),
                _ => println!("Unexpected value"),
            }
            
            // Guard with multiple patterns
            match num {
                n if n % 3 == 0 && n % 5 == 0 => println!("FizzBuzz (divisible by 3 and 5)"),
                n if n % 3 == 0 => println!("Fizz (divisible by 3)"),
                n if n % 5 == 0 => println!("Buzz (divisible by 5)"),
                _ => println!("Number: {}", num),
            }
            
            // Guard with range
            match num {
                n @ 0..=100 if n % 10 == 0 => {
                    println!("{} is between 0-100 and divisible by 10", n)
                }
                n @ 0..=100 => println!("{} is between 0-100", n),
                n => println!("{} is outside 0-100 range", n),
            }
        }
        Err(_) => println!("Invalid number!"),
    }
}

// @ Bindings Demo
fn at_bindings_demo() {
    println!("\n--- @ Bindings ---");
    
    let input = get_input("Enter your age: ");
    
    match input.parse::<u32>() {
        Ok(age) => {
            // @ binding captures value while testing
            match age {
                n @ 0..=2 => println!("Infant ({} years old)", n),
                n @ 3..=12 => {
                    println!("Child ({} years old)", n);
                    println!("School grade: approximately {}", n - 5);
                }
                n @ 13..=19 => {
                    println!("Teenager ({} years old)", n);
                    println!("High school years");
                }
                n @ 20..=35 => {
                    println!("Young adult ({} years old)", n);
                    println!("Prime working years");
                }
                n @ 36..=59 => {
                    println!("Middle-aged adult ({} years old)", n);
                    println!("Experienced professional");
                }
                n @ 60..=79 => {
                    println!("Senior ({} years old)", n);
                    println!("Retirement age in many countries");
                }
                n @ 80.. => {
                    println!("Elderly ({} years old)", n);
                    println!("Congratulations on longevity!");
                }
            }
            
            // @ with guards
            match age {
                n @ _ if n < 18 => {
                    println!("\nMinor: {} years until adulthood", 18 - n)
                }
                n @ 18..=20 if n == 18 => {
                    println!("\nJust became an adult!")
                }
                n @ _ if n >= 18 => {
                    println!("\nAdult for {} years", n - 18)
                }
                _ => {}
            }
        }
        Err(_) => println!("Invalid age!"),
    }
}

// Reference Patterns Demo
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn reference_patterns_demo() {
    println!("\n--- Reference Patterns ---");
    
    let x = get_input("Enter x: ").parse().unwrap_or(0);
    let y = get_input("Enter y: ").parse().unwrap_or(0);
    
    let point = Point { x, y };
    
    // Matching a reference
    println!("\nMatching &point:");
    match &point {
        Point { x: 0, y: 0 } => println!("  At origin"),
        Point { x, y } => println!("  At ({}, {})", x, y),
    }
    
    // Point is still valid!
    println!("Point is still accessible: {:?}", point);
    
    // Using ref in pattern
    let numbers = vec![1, 2, 3, 4, 5];
    match numbers.first() {
        Some(first) => println!("\nFirst number (by reference): {}", first),
        None => println!("\nNo first number"),
    }
    
    // Mutable reference
    let mut value = 10;
    println!("\nOriginal value: {}", value);
    
    match &mut value {
        v => {
            *v += 5;
            println!("Modified value: {}", v);
        }
    }
    
    println!("Final value: {}", value);
}

// Multiple Patterns Demo
fn multiple_patterns_demo() {
    println!("\n--- Multiple Patterns (OR) ---");
    
    let input = get_input("Enter a character: ");
    
    if let Some(ch) = input.chars().next() {
        match ch {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                println!("'{}' is a lowercase vowel", ch)
            }
            'A' | 'E' | 'I' | 'O' | 'U' => {
                println!("'{}' is an uppercase vowel", ch)
            }
            'y' | 'Y' => {
                println!("'{}' is sometimes a vowel", ch)
            }
            'a'..='z' | 'A'..='Z' => {
                println!("'{}' is a consonant", ch)
            }
            '0'..='9' => {
                println!("'{}' is a digit", ch)
            }
            '+' | '-' | '*' | '/' | '%' => {
                println!("'{}' is a math operator", ch)
            }
            _ => {
                println!("'{}' is some other character", ch)
            }
        }
    }
    
    // Multiple patterns with numbers
    println!("\nEnter a month number (1-12):");
    let input = get_input("Month: ");
    
    if let Ok(month) = input.parse::<u32>() {
        match month {
            12 | 1 | 2 => println!("Winter month"),
            3 | 4 | 5 => println!("Spring month"),
            6 | 7 | 8 => println!("Summer month"),
            9 | 10 | 11 => println!("Fall/Autumn month"),
            _ => println!("Invalid month"),
        }
    }
}

// If Let Demo
fn if_let_demo() {
    println!("\n--- If Let Patterns ---");
    
    let input = get_input("Enter a number (or leave empty): ");
    
    let option = if input.is_empty() {
        None
    } else {
        input.parse::<i32>().ok()
    };
    
    // Simple if let
    if let Some(num) = option {
        println!("\nYou entered: {}", num);
        
        if let n @ 0..=10 = num {
            println!("Small number: {}", n);
        } else if let n @ 11..=100 = num {
            println!("Medium number: {}", n);
        } else {
            println!("Large number!");
        }
    } else {
        println!("\nNo number entered");
    }
    
    // If let with else
    let result: Result<i32, String> = if let Some(n) = option {
        if n != 0 {
            Ok(100 / n)
        } else {
            Err("Cannot divide by zero".to_string())
        }
    } else {
        Err("No number provided".to_string())
    };
    
    if let Ok(value) = result {
        println!("Calculation result: {}", value);
    } else if let Err(e) = result {
        println!("Error: {}", e);
    }
}

// While Let Demo
fn while_let_demo() {
    println!("\n--- While Let Patterns ---");
    
    // Stack example
    let mut stack = vec![1, 2, 3, 4, 5];
    println!("Initial stack: {:?}", stack);
    
    println!("\nPopping from stack:");
    while let Some(top) = stack.pop() {
        println!("  Popped: {}", top);
        
        if top == 3 {
            println!("  Stopping at 3");
            break;
        }
    }
    
    println!("Remaining: {:?}", stack);
    
    // Iterator example
    let numbers = vec![10, 20, 30, 40, 50];
    let mut iter = numbers.iter();
    
    println!("\nIterating with while let:");
    let mut count = 0;
    while let Some(&num) = iter.next() {
        count += 1;
        println!("  Item {}: {}", count, num);
        
        if num == 30 {
            println!("  Found 30, stopping");
            break;
        }
    }
}

// Complex Matching Demo
#[derive(Debug)]
enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn complex_matching_demo() {
    println!("\n--- Complex Matching ---");
    
    println!("1. Quit");
    println!("2. Move");
    println!("3. Write");
    println!("4. Change to RGB");
    println!("5. Change to HSV");
    
    let choice = get_input("\nSelect message type: ");
    
    let message = match choice.as_str() {
        "1" => Message::Quit,
        "2" => {
            let x = get_input("X: ").parse().unwrap_or(0);
            let y = get_input("Y: ").parse().unwrap_or(0);
            Message::Move { x, y }
        }
        "3" => {
            let text = get_input("Text: ");
            Message::Write(text)
        }
        "4" => {
            let r = get_input("Red (0-255): ").parse().unwrap_or(0);
            let g = get_input("Green (0-255): ").parse().unwrap_or(0);
            let b = get_input("Blue (0-255): ").parse().unwrap_or(0);
            Message::ChangeColor(Color::Rgb(r, g, b))
        }
        "5" => {
            let h = get_input("Hue (0-360): ").parse().unwrap_or(0);
            let s = get_input("Saturation (0-100): ").parse().unwrap_or(0);
            let v = get_input("Value (0-100): ").parse().unwrap_or(0);
            Message::ChangeColor(Color::Hsv(h, s, v))
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    };
    
    println!("\n--- Processing Message ---");
    
    // Complex nested pattern matching
    match message {
        Message::Quit => {
            println!("Quit command received");
        }
        Message::Move { x, y } if x == 0 && y == 0 => {
            println!("Moving to origin (0, 0)");
        }
        Message::Move { x, y } if x == 0 => {
            println!("Moving vertically to y={}", y);
        }
        Message::Move { x, y } if y == 0 => {
            println!("Moving horizontally to x={}", x);
        }
        Message::Move { x, y } => {
            println!("Moving to ({}, {})", x, y);
            let distance = ((x * x + y * y) as f64).sqrt();
            println!("Distance from origin: {:.2}", distance);
        }
        Message::Write(ref text) if text.is_empty() => {
            println!("Empty message");
        }
        Message::Write(ref text) if text.len() > 20 => {
            println!("Long message ({} chars): {}...", text.len(), &text[..20]);
        }
        Message::Write(text) => {
            println!("Message: {}", text);
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) if r == g && g == b => {
            println!("Grayscale color: RGB({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Rgb(255, 0, 0)) => {
            println!("Pure red!");
        }
        Message::ChangeColor(Color::Rgb(0, 255, 0)) => {
            println!("Pure green!");
        }
        Message::ChangeColor(Color::Rgb(0, 0, 255)) => {
            println!("Pure blue!");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("RGB color: ({}, {}, {})", r, g, b);
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("HSV color: ({}°, {}%, {}%)", h, s, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_point_matching() {
        let point = Point { x: 0, y: 0 };
        
        let result = match &point {
            Point { x: 0, y: 0 } => "origin",
            Point { x: 0, .. } => "y-axis",
            Point { y: 0, .. } => "x-axis",
            _ => "other",
        };
        
        assert_eq!(result, "origin");
    }
    
    #[test]
    fn test_at_binding() {
        let age = 25;
        
        let category = match age {
            n @ 0..=17 => format!("minor: {}", n),
            n @ 18..=64 => format!("adult: {}", n),
            n @ 65.. => format!("senior: {}", n),
        };
        
        assert_eq!(category, "adult: 25");
    }
}
