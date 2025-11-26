use std::io::{self, Write};

/// Demonstrates basic match expressions in Rust
fn main() {
    println!("=== Match Expressions Demo ===\n");
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => number_classification_demo(),
            2 => character_type_demo(),
            3 => day_of_week_demo(),
            4 => traffic_light_demo(),
            5 => range_matching_demo(),
            6 => multiple_patterns_demo(),
            7 => calculator_demo(),
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
    println!("\n--- Match Expression Examples ---");
    println!("1. Number Classification");
    println!("2. Character Type Detection");
    println!("3. Day of Week");
    println!("4. Traffic Light Simulator");
    println!("5. Range Matching");
    println!("6. Multiple Patterns");
    println!("7. Simple Calculator");
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

fn number_classification_demo() {
    println!("\n--- Number Classification ---");
    
    let input = get_input("Enter a number: ");
    
    match input.parse::<i32>() {
        Ok(number) => {
            let classification = classify_number(number);
            println!("Classification: {}", classification);
            
            let description = describe_number(number);
            println!("Description: {}", description);
        }
        Err(_) => println!("Invalid number!"),
    }
}

fn classify_number(n: i32) -> &'static str {
    match n {
        0 => "zero",
        1..=10 => "small positive",
        11..=100 => "medium positive",
        101.. => "large positive",
        -10..=-1 => "small negative",
        -100..=-11 => "medium negative",
        _ => "large negative",
    }
}

fn describe_number(n: i32) -> String {
    let sign = match n {
        n if n > 0 => "positive",
        n if n < 0 => "negative",
        _ => "zero",
    };
    
    let parity = match n {
        0 => "neither even nor odd",
        n if n % 2 == 0 => "even",
        _ => "odd",
    };
    
    format!("{} and {}", sign, parity)
}

fn character_type_demo() {
    println!("\n--- Character Type Detection ---");
    
    let input = get_input("Enter a character: ");
    
    if let Some(ch) = input.chars().next() {
        let char_type = classify_character(ch);
        println!("'{}' is a {}", ch, char_type);
        
        let category = categorize_character(ch);
        println!("Category: {}", category);
    } else {
        println!("No character entered!");
    }
}

fn classify_character(ch: char) -> &'static str {
    match ch {
        'a' | 'e' | 'i' | 'o' | 'u' => "lowercase vowel",
        'A' | 'E' | 'I' | 'O' | 'U' => "uppercase vowel",
        'a'..='z' => "lowercase consonant",
        'A'..='Z' => "uppercase consonant",
        '0'..='9' => "digit",
        ' ' | '\t' | '\n' => "whitespace",
        '!' | '?' | '.' | ',' | ';' | ':' => "punctuation",
        '+' | '-' | '*' | '/' | '=' => "operator",
        _ => "other character",
    }
}

fn categorize_character(ch: char) -> &'static str {
    match ch {
        ch if ch.is_alphabetic() => "letter",
        ch if ch.is_numeric() => "number",
        ch if ch.is_whitespace() => "whitespace",
        ch if ch.is_ascii_punctuation() => "punctuation",
        _ => "special character",
    }
}

fn day_of_week_demo() {
    println!("\n--- Day of Week ---");
    
    let input = get_input("Enter day number (1-7): ");
    
    match input.parse::<u32>() {
        Ok(day) => {
            let day_name = get_day_name(day);
            let day_type = get_day_type(day);
            
            println!("Day {}: {}", day, day_name);
            println!("Type: {}", day_type);
        }
        Err(_) => println!("Invalid number!"),
    }
}

fn get_day_name(day: u32) -> &'static str {
    match day {
        1 => "Monday",
        2 => "Tuesday",
        3 => "Wednesday",
        4 => "Thursday",
        5 => "Friday",
        6 => "Saturday",
        7 => "Sunday",
        _ => "Invalid day",
    }
}

fn get_day_type(day: u32) -> &'static str {
    match day {
        1..=5 => "Weekday - Time to work!",
        6 | 7 => "Weekend - Time to relax!",
        _ => "Not a valid day",
    }
}

fn traffic_light_demo() {
    println!("\n--- Traffic Light Simulator ---");
    
    println!("Colors: red, yellow, green");
    let input = get_input("Enter traffic light color: ");
    
    let color = input.to_lowercase();
    
    let action = match color.as_str() {
        "red" => "STOP! Wait for green.",
        "yellow" => "CAUTION! Prepare to stop.",
        "green" => "GO! Proceed safely.",
        _ => "Invalid color!",
    };
    
    println!("{}", action);
    
    let duration = match color.as_str() {
        "red" => 60,
        "yellow" => 5,
        "green" => 45,
        _ => 0,
    };
    
    if duration > 0 {
        println!("Typical duration: {} seconds", duration);
    }
}

fn range_matching_demo() {
    println!("\n--- Range Matching ---");
    
    let input = get_input("Enter your age: ");
    
    match input.parse::<u32>() {
        Ok(age) => {
            let category = match age {
                0..=2 => "Infant",
                3..=12 => "Child",
                13..=19 => "Teenager",
                20..=35 => "Young Adult",
                36..=59 => "Middle-Aged Adult",
                60..=79 => "Senior",
                80.. => "Elderly",
            };
            
            println!("Age category: {}", category);
            
            let stage = match age {
                0..=17 => "Minor - Cannot vote",
                18..=20 => "Young adult - Can vote, cannot drink (in US)",
                21..=64 => "Adult - Full privileges",
                65.. => "Senior - Eligible for senior benefits",
            };
            
            println!("{}", stage);
        }
        Err(_) => println!("Invalid age!"),
    }
}

fn multiple_patterns_demo() {
    println!("\n--- Multiple Patterns (OR) ---");
    
    let input = get_input("Enter a month number (1-12): ");
    
    match input.parse::<u32>() {
        Ok(month) => {
            let season = match month {
                12 | 1 | 2 => "Winter",
                3 | 4 | 5 => "Spring",
                6 | 7 | 8 => "Summer",
                9 | 10 | 11 => "Fall",
                _ => "Invalid month",
            };
            
            let days = match month {
                1 | 3 | 5 | 7 | 8 | 10 | 12 => "31 days",
                4 | 6 | 9 | 11 => "30 days",
                2 => "28 or 29 days",
                _ => "Invalid",
            };
            
            println!("Month {}: {} - {}", month, season, days);
        }
        Err(_) => println!("Invalid number!"),
    }
}

fn calculator_demo() {
    println!("\n--- Simple Calculator ---");
    
    let num1 = get_input("Enter first number: ");
    let operator = get_input("Enter operator (+, -, *, /): ");
    let num2 = get_input("Enter second number: ");
    
    let n1 = match num1.parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid first number!");
            return;
        }
    };
    
    let n2 = match num2.parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid second number!");
            return;
        }
    };
    
    let result = match operator.as_str() {
        "+" => n1 + n2,
        "-" => n1 - n2,
        "*" => n1 * n2,
        "/" => {
            if n2 == 0.0 {
                println!("Error: Division by zero!");
                return;
            }
            n1 / n2
        }
        "%" => n1 % n2,
        "^" => n1.powf(n2),
        _ => {
            println!("Unknown operator!");
            return;
        }
    };
    
    println!("\n{} {} {} = {}", n1, operator, n2, result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_classify_number() {
        assert_eq!(classify_number(0), "zero");
        assert_eq!(classify_number(5), "small positive");
        assert_eq!(classify_number(50), "medium positive");
        assert_eq!(classify_number(500), "large positive");
        assert_eq!(classify_number(-5), "small negative");
    }
    
    #[test]
    fn test_classify_character() {
        assert_eq!(classify_character('a'), "lowercase vowel");
        assert_eq!(classify_character('B'), "uppercase consonant");
        assert_eq!(classify_character('5'), "digit");
        assert_eq!(classify_character('+'), "operator");
    }
    
    #[test]
    fn test_get_day_name() {
        assert_eq!(get_day_name(1), "Monday");
        assert_eq!(get_day_name(7), "Sunday");
        assert_eq!(get_day_name(8), "Invalid day");
    }
    
    #[test]
    fn test_get_day_type() {
        assert_eq!(get_day_type(1), "Weekday - Time to work!");
        assert_eq!(get_day_type(6), "Weekend - Time to relax!");
    }
}
