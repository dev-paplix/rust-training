// Binary crate that uses the library
use library_binary_split::UserRepository;
use std::io::{self, Write};

fn main() {
    println!("=== Library/Binary Split Demo ===\n");
    println!("This demonstrates separating library and binary crates.");
    println!("The library (lib.rs) contains reusable logic.");
    println!("The binary (main.rs) provides the user interface.\n");
    
    let mut repo = UserRepository::new();
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => create_user(&mut repo),
            2 => list_users(&repo),
            3 => find_user(&repo),
            4 => update_user(&mut repo),
            5 => delete_user(&mut repo),
            6 => show_stats(&repo),
            7 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
        
        println!();
    }
}

fn display_menu() {
    println!("\n--- User Management ---");
    println!("1. Create User");
    println!("2. List Users");
    println!("3. Find User");
    println!("4. Update User");
    println!("5. Delete User");
    println!("6. Show Statistics");
    println!("7. Exit");
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

fn create_user(repo: &mut UserRepository) {
    println!("\n--- Create New User ---");
    
    let username = get_input("Username: ");
    let email = get_input("Email: ");
    
    match repo.create(username, email) {
        Ok(user) => {
            println!("\n✓ User created successfully!");
            println!("  ID: {}", user.id);
            println!("  Username: {}", user.username);
            println!("  Email: {}", user.email);
        }
        Err(e) => println!("\n✗ Error: {}", e),
    }
}

fn list_users(repo: &UserRepository) {
    println!("\n--- User List ---");
    
    let users = repo.all();
    
    if users.is_empty() {
        println!("No users found.");
        return;
    }
    
    println!("{:<5} {:<20} {:<30}", "ID", "Username", "Email");
    println!("{:-<55}", "");
    
    for user in users {
        println!("{:<5} {:<20} {:<30}", user.id, user.username, user.email);
    }
    
    println!("\nTotal: {} user(s)", users.len());
}

fn find_user(repo: &UserRepository) {
    println!("\n--- Find User ---");
    println!("1. Search by ID");
    println!("2. Search by Username");
    
    let choice = get_input("\nSearch by (1/2): ");
    
    match choice.as_str() {
        "1" => {
            let id_str = get_input("Enter ID: ");
            if let Ok(id) = id_str.parse::<u32>() {
                if let Some(user) = repo.find_by_id(id) {
                    println!("\n✓ User found:");
                    println!("  ID: {}", user.id);
                    println!("  Username: {}", user.username);
                    println!("  Email: {}", user.email);
                } else {
                    println!("\n✗ User not found.");
                }
            } else {
                println!("\n✗ Invalid ID.");
            }
        }
        "2" => {
            let username = get_input("Enter username: ");
            if let Some(user) = repo.find_by_username(&username) {
                println!("\n✓ User found:");
                println!("  ID: {}", user.id);
                println!("  Username: {}", user.username);
                println!("  Email: {}", user.email);
            } else {
                println!("\n✗ User not found.");
            }
        }
        _ => println!("\n✗ Invalid choice."),
    }
}

fn update_user(repo: &mut UserRepository) {
    println!("\n--- Update User ---");
    
    let id_str = get_input("Enter user ID: ");
    
    if let Ok(id) = id_str.parse::<u32>() {
        if let Some(user) = repo.find_by_id(id) {
            println!("\nCurrent details:");
            println!("  Username: {}", user.username);
            println!("  Email: {}", user.email);
            
            let username = get_input("\nNew username: ");
            let email = get_input("New email: ");
            
            match repo.update(id, username, email) {
                Ok(()) => println!("\n✓ User updated successfully!"),
                Err(e) => println!("\n✗ Error: {}", e),
            }
        } else {
            println!("\n✗ User not found.");
        }
    } else {
        println!("\n✗ Invalid ID.");
    }
}

fn delete_user(repo: &mut UserRepository) {
    println!("\n--- Delete User ---");
    
    let id_str = get_input("Enter user ID: ");
    
    if let Ok(id) = id_str.parse::<u32>() {
        if let Some(user) = repo.find_by_id(id) {
            println!("\nUser to delete:");
            println!("  Username: {}", user.username);
            println!("  Email: {}", user.email);
            
            let confirm = get_input("\nConfirm deletion (yes/no): ");
            
            if confirm.to_lowercase() == "yes" {
                match repo.delete(id) {
                    Ok(()) => println!("\n✓ User deleted successfully!"),
                    Err(e) => println!("\n✗ Error: {}", e),
                }
            } else {
                println!("\nDeletion cancelled.");
            }
        } else {
            println!("\n✗ User not found.");
        }
    } else {
        println!("\n✗ Invalid ID.");
    }
}

fn show_stats(repo: &UserRepository) {
    println!("\n--- Statistics ---");
    println!("Total Users: {}", repo.count());
    
    let users = repo.all();
    
    if !users.is_empty() {
        // Count email domains
        let mut domains = std::collections::HashMap::new();
        for user in users {
            if let Some(domain) = user.email.split('@').nth(1) {
                *domains.entry(domain).or_insert(0) += 1;
            }
        }
        
        println!("\nEmail Domains:");
        for (domain, count) in domains {
            println!("  {}: {}", domain, count);
        }
        
        // Username length statistics
        let lengths: Vec<usize> = users.iter().map(|u| u.username.len()).collect();
        let min = lengths.iter().min().unwrap();
        let max = lengths.iter().max().unwrap();
        let avg = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
        
        println!("\nUsername Length:");
        println!("  Min: {}", min);
        println!("  Max: {}", max);
        println!("  Average: {:.1}", avg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_and_list() {
        let mut repo = UserRepository::new();
        repo.create("alice".to_string(), "alice@example.com".to_string()).unwrap();
        repo.create("bob".to_string(), "bob@test.com".to_string()).unwrap();
        
        assert_eq!(repo.count(), 2);
        assert_eq!(repo.all().len(), 2);
    }
}
