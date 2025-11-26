// Control Flow Application
// Comprehensive control flow demonstration with practical scenarios

use std::io::{self, Write};

fn main() {
    println!("=== Control Flow Application ===");
    println!("Practical scenarios combining if, match, and loops\n");
    
    loop {
        println!("\n📋 Main Menu:");
        println!("1. Task Manager");
        println!("2. Grade Calculator");
        println!("3. Menu System Builder");
        println!("4. State Machine");
        println!("5. Data Validator");
        println!("6. Game Logic");
        println!("7. Exit");
        print!("\nSelect: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => task_manager(),
            2 => grade_calculator(),
            3 => menu_system(),
            4 => state_machine(),
            5 => data_validator(),
            6 => game_logic(),
            7 => {
                println!("\n👋 Goodbye!");
                break;
            }
            _ => println!("❌ Invalid choice!"),
        }
    }
}

fn task_manager() {
    println!("\n=== Task Manager ===");
    
    let mut tasks: Vec<(String, bool)> = Vec::new();
    
    loop {
        println!("\n📝 Tasks:");
        if tasks.is_empty() {
            println!("   (No tasks)");
        } else {
            for (i, (task, done)) in tasks.iter().enumerate() {
                let status = if *done { "✅" } else { "⬜" };
                println!("   {}. {} {}", i + 1, status, task);
            }
        }
        
        println!("\nOptions:");
        println!("1. Add task");
        println!("2. Complete task");
        println!("3. Remove task");
        println!("4. Show statistics");
        println!("5. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        match read_number() {
            1 => {
                print!("Task name: ");
                io::stdout().flush().unwrap();
                let task = read_string();
                if !task.is_empty() {
                    tasks.push((task, false));
                    println!("✅ Task added");
                }
            }
            2 => {
                if tasks.is_empty() {
                    println!("❌ No tasks to complete");
                    continue;
                }
                
                print!("Task number to complete: ");
                io::stdout().flush().unwrap();
                let num = read_number() as usize;
                
                if num > 0 && num <= tasks.len() {
                    tasks[num - 1].1 = true;
                    println!("✅ Task completed!");
                } else {
                    println!("❌ Invalid task number");
                }
            }
            3 => {
                if tasks.is_empty() {
                    println!("❌ No tasks to remove");
                    continue;
                }
                
                print!("Task number to remove: ");
                io::stdout().flush().unwrap();
                let num = read_number() as usize;
                
                if num > 0 && num <= tasks.len() {
                    tasks.remove(num - 1);
                    println!("✅ Task removed");
                } else {
                    println!("❌ Invalid task number");
                }
            }
            4 => {
                let total = tasks.len();
                let completed = tasks.iter().filter(|(_, done)| *done).count();
                let pending = total - completed;
                
                println!("\n📊 Statistics:");
                println!("   Total: {}", total);
                println!("   Completed: {} ({}%)", completed, 
                    if total > 0 { completed * 100 / total } else { 0 });
                println!("   Pending: {}", pending);
            }
            5 => break,
            _ => println!("❌ Invalid choice"),
        }
    }
}

fn grade_calculator() {
    println!("\n=== Grade Calculator ===");
    
    let mut students: Vec<(String, Vec<i32>)> = Vec::new();
    
    loop {
        println!("\n👨‍🎓 Students:");
        if students.is_empty() {
            println!("   (No students)");
        } else {
            for (i, (name, grades)) in students.iter().enumerate() {
                let avg = if !grades.is_empty() {
                    grades.iter().sum::<i32>() as f64 / grades.len() as f64
                } else {
                    0.0
                };
                
                let letter = match avg as i32 {
                    90..=100 => 'A',
                    80..=89 => 'B',
                    70..=79 => 'C',
                    60..=69 => 'D',
                    _ => 'F',
                };
                
                println!("   {}. {} - Avg: {:.1} ({})", i + 1, name, avg, letter);
            }
        }
        
        println!("\nOptions:");
        println!("1. Add student");
        println!("2. Add grade");
        println!("3. Show student details");
        println!("4. Class statistics");
        println!("5. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        match read_number() {
            1 => {
                print!("Student name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                if !name.is_empty() {
                    students.push((name, Vec::new()));
                    println!("✅ Student added");
                }
            }
            2 => {
                if students.is_empty() {
                    println!("❌ No students");
                    continue;
                }
                
                print!("Student number: ");
                io::stdout().flush().unwrap();
                let num = read_number() as usize;
                
                if num > 0 && num <= students.len() {
                    print!("Grade (0-100): ");
                    io::stdout().flush().unwrap();
                    let grade = read_number();
                    
                    if (0..=100).contains(&grade) {
                        students[num - 1].1.push(grade);
                        println!("✅ Grade added");
                    } else {
                        println!("❌ Invalid grade");
                    }
                } else {
                    println!("❌ Invalid student number");
                }
            }
            3 => {
                if students.is_empty() {
                    println!("❌ No students");
                    continue;
                }
                
                print!("Student number: ");
                io::stdout().flush().unwrap();
                let num = read_number() as usize;
                
                if num > 0 && num <= students.len() {
                    let (name, grades) = &students[num - 1];
                    
                    println!("\n📚 Student: {}", name);
                    println!("   Grades: {:?}", grades);
                    
                    if !grades.is_empty() {
                        let avg = grades.iter().sum::<i32>() as f64 / grades.len() as f64;
                        let max = grades.iter().max().unwrap();
                        let min = grades.iter().min().unwrap();
                        
                        println!("   Average: {:.1}", avg);
                        println!("   Highest: {}", max);
                        println!("   Lowest: {}", min);
                    }
                }
            }
            4 => {
                if students.is_empty() {
                    println!("❌ No students");
                    continue;
                }
                
                let mut all_grades = Vec::new();
                for (_, grades) in &students {
                    all_grades.extend(grades);
                }
                
                if all_grades.is_empty() {
                    println!("❌ No grades recorded");
                    continue;
                }
                
                let class_avg = all_grades.iter().sum::<i32>() as f64 / all_grades.len() as f64;
                
                println!("\n📊 Class Statistics:");
                println!("   Students: {}", students.len());
                println!("   Total grades: {}", all_grades.len());
                println!("   Class average: {:.1}", class_avg);
                
                let mut letter_counts = [0; 5]; // A, B, C, D, F
                for &grade in &all_grades {
                    let idx = match grade {
                        90..=100 => 0,
                        80..=89 => 1,
                        70..=79 => 2,
                        60..=69 => 3,
                        _ => 4,
                    };
                    letter_counts[idx] += 1;
                }
                
                println!("   Grade distribution:");
                let letters = ['A', 'B', 'C', 'D', 'F'];
                for i in 0..5 {
                    println!("      {}: {}", letters[i], letter_counts[i]);
                }
            }
            5 => break,
            _ => println!("❌ Invalid choice"),
        }
    }
}

fn menu_system() {
    println!("\n=== Menu System Builder ===");
    println!("Demonstrates nested menu structures\n");
    
    loop {
        println!("\n🍔 Restaurant Menu:");
        println!("1. Breakfast");
        println!("2. Lunch");
        println!("3. Dinner");
        println!("4. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        match read_number() {
            1 => {
                loop {
                    println!("\n🌅 Breakfast Menu:");
                    println!("1. Pancakes - $8");
                    println!("2. Omelette - $10");
                    println!("3. Cereal - $5");
                    println!("4. Back");
                    print!("\nChoice: ");
                    io::stdout().flush().unwrap();
                    
                    match read_number() {
                        1..=3 => {
                            print!("Quantity: ");
                            io::stdout().flush().unwrap();
                            let qty = read_number();
                            let price = match read_number() {
                                1 => 8,
                                2 => 10,
                                3 => 5,
                                _ => 0,
                            };
                            println!("✅ Added to order: ${}", price * qty);
                        }
                        4 => break,
                        _ => println!("❌ Invalid choice"),
                    }
                }
            }
            2 => println!("🍝 Lunch menu - Coming soon!"),
            3 => println!("🍽️  Dinner menu - Coming soon!"),
            4 => break,
            _ => println!("❌ Invalid choice"),
        }
    }
}

fn state_machine() {
    println!("\n=== State Machine ===");
    println!("Simulates a simple game state machine\n");
    
    #[derive(Debug, PartialEq)]
    enum GameState {
        Menu,
        Playing,
        Paused,
        GameOver,
    }
    
    let mut state = GameState::Menu;
    let mut score = 0;
    
    loop {
        println!("\nCurrent state: {:?}", state);
        println!("Score: {}", score);
        
        match state {
            GameState::Menu => {
                println!("\n🎮 Main Menu:");
                println!("1. Start Game");
                println!("2. Exit");
                print!("\nChoice: ");
                io::stdout().flush().unwrap();
                
                match read_number() {
                    1 => {
                        state = GameState::Playing;
                        score = 0;
                        println!("✅ Game started!");
                    }
                    2 => break,
                    _ => println!("❌ Invalid choice"),
                }
            }
            GameState::Playing => {
                println!("\n🎯 Playing:");
                println!("1. Continue playing (+10 points)");
                println!("2. Pause");
                println!("3. Quit");
                print!("\nChoice: ");
                io::stdout().flush().unwrap();
                
                match read_number() {
                    1 => {
                        score += 10;
                        println!("✅ +10 points!");
                        
                        if score >= 50 {
                            println!("🏆 You win!");
                            state = GameState::GameOver;
                        }
                    }
                    2 => {
                        state = GameState::Paused;
                        println!("⏸️  Game paused");
                    }
                    3 => {
                        state = GameState::GameOver;
                        println!("Game ended");
                    }
                    _ => println!("❌ Invalid choice"),
                }
            }
            GameState::Paused => {
                println!("\n⏸️  Paused:");
                println!("1. Resume");
                println!("2. Quit");
                print!("\nChoice: ");
                io::stdout().flush().unwrap();
                
                match read_number() {
                    1 => {
                        state = GameState::Playing;
                        println!("▶️  Resumed");
                    }
                    2 => {
                        state = GameState::GameOver;
                        println!("Game ended");
                    }
                    _ => println!("❌ Invalid choice"),
                }
            }
            GameState::GameOver => {
                println!("\n💀 Game Over!");
                println!("Final Score: {}", score);
                println!("\n1. Play Again");
                println!("2. Main Menu");
                print!("\nChoice: ");
                io::stdout().flush().unwrap();
                
                match read_number() {
                    1 => {
                        state = GameState::Playing;
                        score = 0;
                    }
                    2 => state = GameState::Menu,
                    _ => println!("❌ Invalid choice"),
                }
            }
        }
    }
}

fn data_validator() {
    println!("\n=== Data Validator ===");
    
    loop {
        println!("\n🔍 Validation Options:");
        println!("1. Validate Email");
        println!("2. Validate Password");
        println!("3. Validate Phone");
        println!("4. Validate Age");
        println!("5. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        match read_number() {
            1 => {
                print!("Enter email: ");
                io::stdout().flush().unwrap();
                let email = read_string();
                
                let valid = email.contains('@') && email.contains('.');
                
                if valid {
                    println!("✅ Valid email");
                } else {
                    println!("❌ Invalid email (must contain @ and .)");
                }
            }
            2 => {
                print!("Enter password: ");
                io::stdout().flush().unwrap();
                let password = read_string();
                
                let len_ok = password.len() >= 8;
                let has_num = password.chars().any(|c| c.is_numeric());
                let has_upper = password.chars().any(|c| c.is_uppercase());
                let has_lower = password.chars().any(|c| c.is_lowercase());
                
                if !len_ok {
                    println!("❌ Too short (min 8 chars)");
                } else if !has_num {
                    println!("❌ Must contain a number");
                } else if !has_upper || !has_lower {
                    println!("❌ Must contain upper and lower case");
                } else {
                    println!("✅ Strong password!");
                }
            }
            3 => {
                print!("Enter phone (10 digits): ");
                io::stdout().flush().unwrap();
                let phone = read_string();
                
                let digits_only: String = phone.chars().filter(|c| c.is_numeric()).collect();
                
                if digits_only.len() == 10 {
                    println!("✅ Valid phone number");
                } else {
                    println!("❌ Invalid (must be 10 digits)");
                }
            }
            4 => {
                print!("Enter age: ");
                io::stdout().flush().unwrap();
                let age = read_number();
                
                match age {
                    0..=0 => println!("❌ Invalid age"),
                    1..=17 => println!("⚠️  Minor"),
                    18..=120 => println!("✅ Valid adult age"),
                    _ => println!("❌ Invalid age"),
                }
            }
            5 => break,
            _ => println!("❌ Invalid choice"),
        }
    }
}

fn game_logic() {
    println!("\n=== Simple Game Logic ===");
    println!("Number guessing game\n");
    
    let secret = 42;
    let max_attempts = 7;
    let mut attempts = 0;
    let mut won = false;
    
    println!("I'm thinking of a number between 1 and 100");
    println!("You have {} attempts\n", max_attempts);
    
    loop {
        attempts += 1;
        
        print!("Attempt {}/{} - Your guess: ", attempts, max_attempts);
        io::stdout().flush().unwrap();
        let guess = read_number();
        
        match guess {
            g if g == secret => {
                println!("🎉 Correct! You won in {} attempts!", attempts);
                won = true;
                break;
            }
            g if g < secret => {
                let diff = secret - g;
                match diff {
                    1..=5 => println!("📈 Very close! Go higher"),
                    6..=15 => println!("📈 Close! Go higher"),
                    _ => println!("📈 Too low"),
                }
            }
            g if g > secret => {
                let diff = g - secret;
                match diff {
                    1..=5 => println!("📉 Very close! Go lower"),
                    6..=15 => println!("📉 Close! Go lower"),
                    _ => println!("📉 Too high"),
                }
            }
            _ => unreachable!(),
        }
        
        if attempts >= max_attempts {
            println!("\n💀 Game Over! The number was {}", secret);
            break;
        }
    }
    
    if won {
        let rating = match attempts {
            1 => "🏆 Perfect!",
            2..=3 => "⭐ Excellent!",
            4..=5 => "👍 Good!",
            _ => "✅ You got it!",
        };
        println!("{}", rating);
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
    fn test_grade_mapping() {
        let grade = match 85 {
            90..=100 => 'A',
            80..=89 => 'B',
            70..=79 => 'C',
            60..=69 => 'D',
            _ => 'F',
        };
        assert_eq!(grade, 'B');
    }
    
    #[test]
    fn test_email_validation() {
        let email = "test@example.com";
        let valid = email.contains('@') && email.contains('.');
        assert!(valid);
    }
    
    #[test]
    fn test_password_strength() {
        let password = "Secure123";
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_num = password.chars().any(|c| c.is_numeric());
        
        assert!(has_upper && has_lower && has_num);
    }
}
