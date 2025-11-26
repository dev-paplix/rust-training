// HashMap Operations
// Demonstrates HashMap<K, V> and the Entry API

use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    println!("=== HashMap Operations ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Phone Book");
        println!("2. Word Frequency Counter");
        println!("3. Student Grades");
        println!("4. Inventory System");
        println!("5. Entry API Demo");
        println!("6. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => phone_book(),
            2 => word_frequency(),
            3 => student_grades(),
            4 => inventory_system(),
            5 => entry_api_demo(),
            6 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn phone_book() {
    println!("\n=== Phone Book ===");
    let mut phone_book: HashMap<String, String> = HashMap::new();
    
    loop {
        println!("\n1. Add contact");
        println!("2. Find contact");
        println!("3. Update contact");
        println!("4. Delete contact");
        println!("5. List all contacts");
        println!("6. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                print!("Enter phone number: ");
                io::stdout().flush().unwrap();
                let phone = read_string();
                
                phone_book.insert(name.clone(), phone);
                println!("✅ Added contact: {}", name);
            }
            2 => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                match phone_book.get(&name) {
                    Some(phone) => println!("📞 {}: {}", name, phone),
                    None => println!("❌ Contact not found"),
                }
            }
            3 => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                if !phone_book.contains_key(&name) {
                    println!("❌ Contact not found");
                    continue;
                }
                
                print!("Enter new phone number: ");
                io::stdout().flush().unwrap();
                let phone = read_string();
                
                phone_book.insert(name.clone(), phone);
                println!("✅ Updated contact: {}", name);
            }
            4 => {
                print!("Enter name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                if phone_book.remove(&name).is_some() {
                    println!("✅ Deleted contact: {}", name);
                } else {
                    println!("❌ Contact not found");
                }
            }
            5 => {
                if phone_book.is_empty() {
                    println!("\n📕 Phone book is empty");
                } else {
                    println!("\n📕 Contacts ({}):", phone_book.len());
                    let mut contacts: Vec<_> = phone_book.iter().collect();
                    contacts.sort_by_key(|(name, _)| name.as_str());
                    
                    for (name, phone) in contacts {
                        println!("   {} → {}", name, phone);
                    }
                }
            }
            6 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn word_frequency() {
    println!("\n=== Word Frequency Counter ===");
    println!("Enter text (press Enter twice to finish):");
    
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        if line.trim().is_empty() {
            break;
        }
        lines.push(line);
    }
    
    let text = lines.join(" ");
    
    if text.trim().is_empty() {
        println!("No text entered!");
        return;
    }
    
    // Count word frequency
    let mut word_count: HashMap<String, usize> = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word
            .to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        
        if !word.is_empty() {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }
    
    // Display results
    println!("\n📊 Word Frequency:");
    println!("   Total words: {}", text.split_whitespace().count());
    println!("   Unique words: {}", word_count.len());
    
    // Sort by frequency
    let mut freq_vec: Vec<_> = word_count.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1).then_with(|| a.0.cmp(b.0)));
    
    println!("\n🔝 Top 10 words:");
    for (i, (word, count)) in freq_vec.iter().take(10).enumerate() {
        println!("   {}. '{}': {} times", i + 1, word, count);
    }
    
    // Character frequency
    let mut char_count: HashMap<char, usize> = HashMap::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            *char_count.entry(ch.to_lowercase().next().unwrap()).or_insert(0) += 1;
        }
    }
    
    println!("\n🔤 Character frequency:");
    let mut char_vec: Vec<_> = char_count.iter().collect();
    char_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (ch, count) in char_vec.iter().take(10) {
        println!("   '{}': {}", ch, count);
    }
}

fn student_grades() {
    println!("\n=== Student Grades ===");
    let mut grades: HashMap<String, Vec<i32>> = HashMap::new();
    
    loop {
        println!("\n1. Add student");
        println!("2. Add grade");
        println!("3. View student");
        println!("4. Class statistics");
        println!("5. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter student name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                grades.entry(name.clone()).or_insert_with(Vec::new);
                println!("✅ Added student: {}", name);
            }
            2 => {
                print!("Enter student name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                print!("Enter grade (0-100): ");
                io::stdout().flush().unwrap();
                let grade = read_number();
                
                if grade < 0 || grade > 100 {
                    println!("❌ Invalid grade!");
                    continue;
                }
                
                grades.entry(name.clone())
                    .or_insert_with(Vec::new)
                    .push(grade);
                println!("✅ Added grade {} for {}", grade, name);
            }
            3 => {
                print!("Enter student name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                match grades.get(&name) {
                    Some(student_grades) if !student_grades.is_empty() => {
                        let avg = student_grades.iter().sum::<i32>() as f64 
                            / student_grades.len() as f64;
                        println!("\n📚 {}", name);
                        println!("   Grades: {:?}", student_grades);
                        println!("   Average: {:.2}", avg);
                        println!("   Count: {}", student_grades.len());
                    }
                    Some(_) => println!("❌ No grades recorded for {}", name),
                    None => println!("❌ Student not found"),
                }
            }
            4 => {
                if grades.is_empty() {
                    println!("\n❌ No students in database");
                    continue;
                }
                
                println!("\n📊 Class Statistics:");
                println!("   Total students: {}", grades.len());
                
                let mut all_grades = Vec::new();
                for student_grades in grades.values() {
                    all_grades.extend(student_grades);
                }
                
                if !all_grades.is_empty() {
                    let avg = all_grades.iter().sum::<i32>() as f64 / all_grades.len() as f64;
                    println!("   Total grades: {}", all_grades.len());
                    println!("   Class average: {:.2}", avg);
                    
                    // Top students
                    let mut student_avgs: Vec<_> = grades.iter()
                        .filter(|(_, g)| !g.is_empty())
                        .map(|(name, g)| {
                            let avg = g.iter().sum::<i32>() as f64 / g.len() as f64;
                            (name, avg)
                        })
                        .collect();
                    student_avgs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    
                    println!("\n🏆 Top students:");
                    for (i, (name, avg)) in student_avgs.iter().take(5).enumerate() {
                        println!("   {}. {}: {:.2}", i + 1, name, avg);
                    }
                }
            }
            5 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn inventory_system() {
    println!("\n=== Inventory System ===");
    let mut inventory: HashMap<String, (i32, f64)> = HashMap::new(); // (quantity, price)
    
    loop {
        println!("\n1. Add product");
        println!("2. Update quantity");
        println!("3. Update price");
        println!("4. View product");
        println!("5. List all products");
        println!("6. Calculate total value");
        println!("7. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                print!("Enter quantity: ");
                io::stdout().flush().unwrap();
                let qty = read_number();
                
                print!("Enter price: ");
                io::stdout().flush().unwrap();
                let price = read_float();
                
                inventory.insert(name.clone(), (qty, price));
                println!("✅ Added product: {}", name);
            }
            2 => {
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                print!("Enter quantity change (+/-): ");
                io::stdout().flush().unwrap();
                let change = read_number();
                
                inventory.entry(name.clone())
                    .and_modify(|(qty, _)| *qty += change)
                    .or_insert((change, 0.0));
                
                println!("✅ Updated quantity for {}", name);
            }
            3 => {
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                if !inventory.contains_key(&name) {
                    println!("❌ Product not found");
                    continue;
                }
                
                print!("Enter new price: ");
                io::stdout().flush().unwrap();
                let price = read_float();
                
                inventory.entry(name.clone())
                    .and_modify(|(_, p)| *p = price);
                
                println!("✅ Updated price for {}", name);
            }
            4 => {
                print!("Enter product name: ");
                io::stdout().flush().unwrap();
                let name = read_string();
                
                match inventory.get(&name) {
                    Some((qty, price)) => {
                        println!("\n📦 {}", name);
                        println!("   Quantity: {}", qty);
                        println!("   Price: ${:.2}", price);
                        println!("   Value: ${:.2}", *qty as f64 * price);
                    }
                    None => println!("❌ Product not found"),
                }
            }
            5 => {
                if inventory.is_empty() {
                    println!("\n❌ Inventory is empty");
                } else {
                    println!("\n📦 Inventory ({} products):", inventory.len());
                    let mut items: Vec<_> = inventory.iter().collect();
                    items.sort_by_key(|(name, _)| name.as_str());
                    
                    for (name, (qty, price)) in items {
                        let value = *qty as f64 * price;
                        println!("   {} | Qty: {} | ${:.2} | Value: ${:.2}", 
                            name, qty, price, value);
                    }
                }
            }
            6 => {
                let total: f64 = inventory.values()
                    .map(|(qty, price)| *qty as f64 * price)
                    .sum();
                println!("\n💰 Total inventory value: ${:.2}", total);
            }
            7 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn entry_api_demo() {
    println!("\n=== Entry API Demonstration ===");
    
    let mut scores: HashMap<String, i32> = HashMap::new();
    
    println!("\nDemonstrating different Entry API patterns:\n");
    
    // or_insert
    println!("1. or_insert - Insert only if key doesn't exist");
    scores.entry("Alice".to_string()).or_insert(100);
    scores.entry("Alice".to_string()).or_insert(200); // Won't change
    println!("   Alice: {} (second insert didn't change it)", scores["Alice"]);
    
    // or_insert_with
    println!("\n2. or_insert_with - Insert with function (lazy)");
    scores.entry("Bob".to_string()).or_insert_with(|| {
        println!("   Computing default value for Bob...");
        50
    });
    println!("   Bob: {}", scores["Bob"]);
    
    // and_modify + or_insert
    println!("\n3. and_modify + or_insert - Modify if exists, insert if not");
    scores.entry("Alice".to_string())
        .and_modify(|v| *v += 10)
        .or_insert(0);
    println!("   Alice: {} (modified)", scores["Alice"]);
    
    scores.entry("Charlie".to_string())
        .and_modify(|v| *v += 10)
        .or_insert(0);
    println!("   Charlie: {} (inserted)", scores["Charlie"]);
    
    // Counter pattern
    println!("\n4. Counter pattern - Word counting");
    let text = "hello world hello rust world rust rust";
    let mut word_count: HashMap<&str, usize> = HashMap::new();
    
    for word in text.split_whitespace() {
        *word_count.entry(word).or_insert(0) += 1;
    }
    
    println!("   Word counts: {:?}", word_count);
    
    // Grouping pattern
    println!("\n5. Grouping pattern - Group by first letter");
    let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
    let mut groups: HashMap<char, Vec<&str>> = HashMap::new();
    
    for word in words {
        groups.entry(word.chars().next().unwrap())
            .or_insert_with(Vec::new)
            .push(word);
    }
    
    for (letter, words) in &groups {
        println!("   {}: {:?}", letter, words);
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
    use super::*;
    
    #[test]
    fn test_hashmap_insert_get() {
        let mut map = HashMap::new();
        map.insert("key", "value");
        assert_eq!(map.get("key"), Some(&"value"));
    }
    
    #[test]
    fn test_entry_api() {
        let mut map = HashMap::new();
        *map.entry("count").or_insert(0) += 1;
        *map.entry("count").or_insert(0) += 1;
        assert_eq!(map["count"], 2);
    }
    
    #[test]
    fn test_word_count() {
        let mut counts = HashMap::new();
        for word in "hello world hello".split_whitespace() {
            *counts.entry(word).or_insert(0) += 1;
        }
        assert_eq!(counts["hello"], 2);
        assert_eq!(counts["world"], 1);
    }
}
