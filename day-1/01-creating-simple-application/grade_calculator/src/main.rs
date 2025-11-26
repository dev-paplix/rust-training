use std::io;

// Grade Calculator Application
// Converts numeric scores to letter grades

fn main() {
    println!("=== Grade Calculator ===");
    println!("Enter your score (0-100):");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let score: f64 = match input.trim().parse() {
        Ok(num) => {
            if num < 0.0 || num > 100.0 {
                println!("Error: Score must be between 0 and 100");
                return;
            }
            num
        }
        Err(_) => {
            println!("Error: Please enter a valid number");
            return;
        }
    };
    
    let grade = calculate_grade(score);
    let status = if score >= 60.0 { "PASS" } else { "FAIL" };
    
    println!("\n--- Results ---");
    println!("Score: {:.1}", score);
    println!("Letter Grade: {}", grade);
    println!("Status: {}", status);
    print_grade_description(grade);
}

/// Calculates letter grade from numeric score
fn calculate_grade(score: f64) -> char {
    if score >= 90.0 {
        'A'
    } else if score >= 80.0 {
        'B'
    } else if score >= 70.0 {
        'C'
    } else if score >= 60.0 {
        'D'
    } else {
        'F'
    }
}

/// Prints a description of the grade
fn print_grade_description(grade: char) {
    let description = match grade {
        'A' => "Excellent! Outstanding performance! 🌟",
        'B' => "Good work! Above average performance! 👍",
        'C' => "Satisfactory. Average performance. 📚",
        'D' => "Needs improvement. Below average. ⚠️",
        'F' => "Failing. Significant improvement needed. ❌",
        _ => "Invalid grade",
    };
    
    println!("Description: {}", description);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calculate_grade() {
        assert_eq!(calculate_grade(95.0), 'A');
        assert_eq!(calculate_grade(85.0), 'B');
        assert_eq!(calculate_grade(75.0), 'C');
        assert_eq!(calculate_grade(65.0), 'D');
        assert_eq!(calculate_grade(55.0), 'F');
        assert_eq!(calculate_grade(90.0), 'A');
        assert_eq!(calculate_grade(89.9), 'B');
    }
}
