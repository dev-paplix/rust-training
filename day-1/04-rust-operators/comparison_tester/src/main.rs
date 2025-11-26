// Comparison Tester
// Demonstrates comparison operators and ordering

use std::io::{self, Write};

fn main() {
    println!("=== Comparison Operator Tester ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Number Comparison");
        println!("2. String Comparison");
        println!("3. Range Testing");
        println!("4. Sorting Demo");
        println!("5. Grade Calculator");
        println!("6. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => number_comparison(),
            2 => string_comparison(),
            3 => range_testing(),
            4 => sorting_demo(),
            5 => grade_calculator(),
            6 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn number_comparison() {
    println!("\n--- Number Comparison ---");
    
    print!("Enter first number: ");
    io::stdout().flush().unwrap();
    let a = read_float();
    
    print!("Enter second number: ");
    io::stdout().flush().unwrap();
    let b = read_float();
    
    println!("\nComparison Results:");
    println!("  {} == {}: {}", a, b, a == b);
    println!("  {} != {}: {}", a, b, a != b);
    println!("  {} < {}: {}", a, b, a < b);
    println!("  {} > {}: {}", a, b, a > b);
    println!("  {} <= {}: {}", a, b, a <= b);
    println!("  {} >= {}: {}", a, b, a >= b);
    
    // Determine relationship
    if a == b {
        println!("\n✅ Numbers are equal");
    } else if a < b {
        println!("\n📊 {} is less than {}", a, b);
        println!("   Difference: {}", b - a);
    } else {
        println!("\n📊 {} is greater than {}", a, b);
        println!("   Difference: {}", a - b);
    }
    
    // Floating-point precision
    if (a - b).abs() < 1e-10 {
        println!("   (Within epsilon tolerance)");
    }
}

fn string_comparison() {
    println!("\n--- String Comparison ---");
    
    print!("Enter first string: ");
    io::stdout().flush().unwrap();
    let s1 = read_string();
    
    print!("Enter second string: ");
    io::stdout().flush().unwrap();
    let s2 = read_string();
    
    println!("\nComparison Results:");
    println!("  '{}' == '{}': {}", s1, s2, s1 == s2);
    println!("  '{}' != '{}': {}", s1, s2, s1 != s2);
    println!("  '{}' < '{}': {} (lexicographic)", s1, s2, s1 < s2);
    println!("  '{}' > '{}': {} (lexicographic)", s1, s2, s1 > s2);
    
    // Case-insensitive comparison
    let s1_lower = s1.to_lowercase();
    let s2_lower = s2.to_lowercase();
    println!("\nCase-insensitive comparison:");
    println!("  Equal: {}", s1_lower == s2_lower);
    
    // Length comparison
    println!("\nLength comparison:");
    println!("  '{}' length: {}", s1, s1.len());
    println!("  '{}' length: {}", s2, s2.len());
    
    if s1.len() == s2.len() {
        println!("  ✅ Same length");
    } else if s1.len() < s2.len() {
        println!("  📏 First string is shorter by {} characters", s2.len() - s1.len());
    } else {
        println!("  📏 First string is longer by {} characters", s1.len() - s2.len());
    }
}

fn range_testing() {
    println!("\n--- Range Testing ---");
    
    print!("Enter a value: ");
    io::stdout().flush().unwrap();
    let value = read_number();
    
    println!("\nRange Tests:");
    
    // Various range tests
    if (0..10).contains(&value) {
        println!("  ✅ {} is in range 0..10", value);
    } else {
        println!("  ❌ {} is NOT in range 0..10", value);
    }
    
    if (0..=10).contains(&value) {
        println!("  ✅ {} is in range 0..=10", value);
    } else {
        println!("  ❌ {} is NOT in range 0..=10", value);
    }
    
    if (50..=100).contains(&value) {
        println!("  ✅ {} is in range 50..=100", value);
    } else {
        println!("  ❌ {} is NOT in range 50..=100", value);
    }
    
    // Multiple range check
    println!("\nCategory:");
    match value {
        i32::MIN..=-1 => println!("  Negative number"),
        0 => println!("  Zero"),
        1..=10 => println!("  Single digit positive"),
        11..=99 => println!("  Double digit"),
        100..=999 => println!("  Triple digit"),
        _ => println!("  Large number"),
    }
}

fn sorting_demo() {
    println!("\n--- Sorting Demo ---");
    
    let mut numbers = vec![];
    
    println!("Enter 5 numbers:");
    for i in 1..=5 {
        print!("  Number {}: ", i);
        io::stdout().flush().unwrap();
        numbers.push(read_float());
    }
    
    println!("\n📋 Original: {:?}", numbers);
    
    // Sort ascending
    let mut sorted_asc = numbers.clone();
    sorted_asc.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("📈 Ascending: {:?}", sorted_asc);
    
    // Sort descending
    let mut sorted_desc = numbers.clone();
    sorted_desc.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("📉 Descending: {:?}", sorted_desc);
    
    // Statistics
    let min = sorted_asc.first().unwrap();
    let max = sorted_asc.last().unwrap();
    let sum: f64 = numbers.iter().sum();
    let avg = sum / numbers.len() as f64;
    
    println!("\n📊 Statistics:");
    println!("  Minimum: {}", min);
    println!("  Maximum: {}", max);
    println!("  Average: {:.2}", avg);
    println!("  Range: {}", max - min);
}

fn grade_calculator() {
    println!("\n--- Grade Calculator ---");
    
    print!("Enter score (0-100): ");
    io::stdout().flush().unwrap();
    let score = read_number();
    
    if score < 0 || score > 100 {
        println!("❌ Invalid score! Must be between 0 and 100.");
        return;
    }
    
    let grade = match score {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        0..=59 => 'F',
        _ => '?',
    };
    
    let description = match grade {
        'A' => "Excellent! 🌟",
        'B' => "Good work! 👍",
        'C' => "Satisfactory 👌",
        'D' => "Needs improvement 📚",
        'F' => "Failed ❌",
        _ => "Unknown",
    };
    
    println!("\n📝 Results:");
    println!("  Score: {}", score);
    println!("  Grade: {}", grade);
    println!("  Status: {}", description);
    
    // Performance feedback
    if score >= 90 {
        println!("  🎉 Top performer!");
    } else if score >= 70 {
        println!("  ✅ Passing grade");
    } else if score >= 60 {
        println!("  ⚠️  Barely passing");
    } else {
        println!("  ❌ Below passing grade");
    }
    
    // Points to next grade
    if score < 100 {
        let next_grade_threshold = match score {
            0..=59 => 60,
            60..=69 => 70,
            70..=79 => 80,
            80..=89 => 90,
            90..=99 => 100,
            _ => 100,
        };
        println!("  📈 {} points to next grade level", next_grade_threshold - score);
    }
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

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_comparisons() {
        assert!(5 > 3);
        assert!(3 < 5);
        assert!(5 >= 5);
        assert!(5 <= 5);
        assert_eq!(5, 5);
        assert_ne!(5, 3);
    }
    
    #[test]
    fn test_ranges() {
        let value = 5;
        assert!((0..10).contains(&value));
        assert!((0..=5).contains(&value));
        assert!(!(10..20).contains(&value));
    }
    
    #[test]
    fn test_string_comparison() {
        assert!("apple" < "banana");
        assert!("abc" == "abc");
        assert!("xyz" > "abc");
    }
}
