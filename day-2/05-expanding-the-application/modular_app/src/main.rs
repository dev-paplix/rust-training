use std::io::{self, Write};

mod models;
mod handlers;
mod utils;

use models::User;
use handlers::UserHandler;

/// Demonstrates modular application structure
fn main() {
    let mut handler = UserHandler::new();
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => add_user(&mut handler),
            2 => list_users(&handler),
            3 => search_user(&handler),
            4 => update_user(&mut handler),
            5 => delete_user(&mut handler),
            6 => show_statistics(&handler),
            7 => save_to_file(&handler),
            8 => load_from_file(&mut handler),
            0 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("\n❌ Invalid choice. Please try again."),
        }
        
        pause();
    }
}

fn display_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║      Modular App Demo                 ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Add User");
    println!("2. List Users");
    println!("3. Search User");
    println!("4. Update User");
    println!("5. Delete User");
    println!("6. Show Statistics");
    println!("7. Save to File");
    println!("8. Load from File");
    println!("0. Exit");
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn add_user(handler: &mut UserHandler) {
    println!("\n=== Add New User ===");
    
    let name = get_input("Enter name: ");
    let email = get_input("Enter email: ");
    
    // Validate input
    if !utils::validators::validate_name(&name) {
        println!("❌ Invalid name format");
        return;
    }
    
    if !utils::validators::validate_email(&email) {
        println!("❌ Invalid email format");
        return;
    }
    
    match User::new(name, email) {
        Ok(user) => {
            match handler.add_user(user) {
                Ok(id) => println!("✓ User added successfully with ID: {}", id),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(e) => println!("❌ Error creating user: {}", e),
    }
}

fn list_users(handler: &UserHandler) {
    println!("\n=== User List ===");
    
    let users = handler.list_users();
    
    if users.is_empty() {
        println!("No users found.");
        return;
    }
    
    println!("\nTotal users: {}\n", users.len());
    println!("{:<5} {:<20} {:<30}", "ID", "Name", "Email");
    println!("{}", "-".repeat(60));
    
    for user in users {
        println!("{:<5} {:<20} {:<30}", 
                 user.id(), 
                 utils::formatters::truncate_string(user.name(), 20),
                 utils::formatters::truncate_string(user.email(), 30));
    }
}

fn search_user(handler: &UserHandler) {
    println!("\n=== Search User ===");
    
    let query = get_input("Enter search query (name or email): ");
    
    let results = handler.search(&query);
    
    if results.is_empty() {
        println!("No users found matching '{}'", query);
        return;
    }
    
    println!("\nFound {} user(s):\n", results.len());
    println!("{:<5} {:<20} {:<30}", "ID", "Name", "Email");
    println!("{}", "-".repeat(60));
    
    for user in results {
        println!("{:<5} {:<20} {:<30}", 
                 user.id(), 
                 user.name(),
                 user.email());
    }
}

fn update_user(handler: &mut UserHandler) {
    println!("\n=== Update User ===");
    
    let id_str = get_input("Enter user ID: ");
    let id: u32 = match id_str.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ Invalid ID");
            return;
        }
    };
    
    // Check if user exists
    if handler.get_user(id).is_none() {
        println!("❌ User with ID {} not found", id);
        return;
    }
    
    let name = get_input("Enter new name: ");
    let email = get_input("Enter new email: ");
    
    if !utils::validators::validate_name(&name) {
        println!("❌ Invalid name format");
        return;
    }
    
    if !utils::validators::validate_email(&email) {
        println!("❌ Invalid email format");
        return;
    }
    
    match handler.update_user(id, name, email) {
        Ok(_) => println!("✓ User updated successfully"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn delete_user(handler: &mut UserHandler) {
    println!("\n=== Delete User ===");
    
    let id_str = get_input("Enter user ID: ");
    let id: u32 = match id_str.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("❌ Invalid ID");
            return;
        }
    };
    
    // Show user details before deletion
    if let Some(user) = handler.get_user(id) {
        println!("\nUser to delete:");
        println!("  ID:    {}", user.id());
        println!("  Name:  {}", user.name());
        println!("  Email: {}", user.email());
        
        let confirm = get_input("\nAre you sure? (yes/no): ");
        if confirm.to_lowercase() != "yes" {
            println!("Deletion cancelled");
            return;
        }
    } else {
        println!("❌ User with ID {} not found", id);
        return;
    }
    
    match handler.delete_user(id) {
        Ok(_) => println!("✓ User deleted successfully"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn show_statistics(handler: &UserHandler) {
    println!("\n=== Statistics ===");
    
    let stats = handler.get_statistics();
    
    println!("Total Users: {}", stats.total_users);
    
    if !stats.email_domains.is_empty() {
        println!("\nEmail Domain Distribution:");
        let mut domains: Vec<_> = stats.email_domains.iter().collect();
        domains.sort_by(|a, b| b.1.cmp(a.1));
        
        for (domain, count) in domains {
            let bar = "█".repeat(*count);
            println!("  {:<20} {} ({})", domain, bar, count);
        }
    }
    
    if !stats.name_lengths.is_empty() {
        println!("\nName Length Distribution:");
        println!("  Shortest: {} characters", stats.name_lengths.0);
        println!("  Longest:  {} characters", stats.name_lengths.1);
        println!("  Average:  {:.1} characters", stats.name_lengths.2);
    }
}

fn save_to_file(handler: &UserHandler) {
    println!("\n=== Save to File ===");
    
    let filename = get_input("Enter filename (e.g., users.json): ");
    
    match handler.save_to_file(&filename) {
        Ok(_) => println!("✓ Data saved to '{}'", filename),
        Err(e) => println!("❌ Error saving file: {}", e),
    }
}

fn load_from_file(handler: &mut UserHandler) {
    println!("\n=== Load from File ===");
    
    let filename = get_input("Enter filename: ");
    
    match handler.load_from_file(&filename) {
        Ok(count) => println!("✓ Loaded {} users from '{}'", count, filename),
        Err(e) => println!("❌ Error loading file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new("Alice".to_string(), "alice@example.com".to_string());
        assert!(user.is_ok());
    }
    
    #[test]
    fn test_add_and_retrieve_user() {
        let mut handler = UserHandler::new();
        let user = User::new("Bob".to_string(), "bob@example.com".to_string()).unwrap();
        let id = handler.add_user(user).unwrap();
        
        let retrieved = handler.get_user(id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "Bob");
    }
    
    #[test]
    fn test_search_functionality() {
        let mut handler = UserHandler::new();
        handler.add_user(User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap()).unwrap();
        handler.add_user(User::new("Bob".to_string(), "bob@example.com".to_string()).unwrap()).unwrap();
        
        let results = handler.search("Alice");
        assert_eq!(results.len(), 1);
    }
}
