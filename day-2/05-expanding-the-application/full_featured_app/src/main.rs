use full_featured_app::{UserRepository, AppConfig, AppResult};
use std::io::{self, Write};

fn main() -> AppResult<()> {
    println!("=== Full-Featured Application ===\n");
    println!("Demonstrates a complete Rust application with:");
    println!("  • Library/Binary split");
    println!("  • Configuration management");
    println!("  • Error handling");
    println!("  • File I/O (JSON & CSV)");
    println!("  • Module organization\n");
    
    let config = load_or_create_config()?;
    println!("✓ Configuration loaded: {} v{}", config.app.name, config.app.version);
    println!("  Environment: {}", config.app.environment);
    
    let mut repo = UserRepository::new();
    
    // Try to load existing data
    if let Ok(count) = repo.load_from_json("users.json") {
        println!("✓ Loaded {} users from file", count);
    }
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        let result = match choice {
            1 => add_user(&mut repo),
            2 => list_users(&repo),
            3 => search_users(&repo),
            4 => update_user(&mut repo),
            5 => delete_user(&mut repo),
            6 => show_statistics(&repo),
            7 => export_to_csv(&repo),
            8 => import_from_csv(&mut repo),
            9 => save_data(&repo),
            10 => load_data(&mut repo),
            11 => {
                // Auto-save before exit
                let _ = repo.save_to_json("users.json");
                println!("Data saved. Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice, try again.");
                Ok(())
            }
        };
        
        if let Err(e) = result {
            println!("\n✗ Error: {}", e);
        }
        
        println!();
    }
    
    Ok(())
}

fn display_menu() {
    println!("\n--- Main Menu ---");
    println!(" 1. Add User");
    println!(" 2. List Users");
    println!(" 3. Search Users");
    println!(" 4. Update User");
    println!(" 5. Delete User");
    println!(" 6. Show Statistics");
    println!(" 7. Export to CSV");
    println!(" 8. Import from CSV");
    println!(" 9. Save Data");
    println!("10. Load Data");
    println!("11. Exit");
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

fn load_or_create_config() -> AppResult<AppConfig> {
    match AppConfig::load_from_file("app_config.toml") {
        Ok(config) => Ok(config),
        Err(_) => {
            let config = AppConfig::default();
            config.save_to_file("app_config.toml")?;
            println!("  (Created default configuration)");
            Ok(config)
        }
    }
}

fn add_user(repo: &mut UserRepository) -> AppResult<()> {
    println!("\n--- Add User ---");
    
    let name = get_input("Name: ");
    let email = get_input("Email: ");
    let age_str = get_input("Age: ");
    
    let age = age_str.parse::<u32>()
        .map_err(|_| "Invalid age".to_string())?;
    
    let id = repo.create_user(name, email, age)?;
    println!("\n✓ User created successfully! ID: {}", id);
    
    Ok(())
}

fn list_users(repo: &UserRepository) -> AppResult<()> {
    println!("\n--- User List ---");
    
    let users = repo.all_users();
    
    if users.is_empty() {
        println!("No users found.");
        return Ok(());
    }
    
    println!("{:<5} {:<20} {:<30} {:<5} {:<20}", "ID", "Name", "Email", "Age", "Created");
    println!("{:-<80}", "");
    
    for user in users {
        println!("{:<5} {:<20} {:<30} {:<5} {:<20}", 
                 user.id(), 
                 truncate(user.name(), 20),
                 truncate(user.email(), 30),
                 user.age(),
                 truncate(user.created_at(), 20));
    }
    
    println!("\nTotal: {} user(s)", users.len());
    
    Ok(())
}

fn search_users(repo: &UserRepository) -> AppResult<()> {
    println!("\n--- Search Users ---");
    
    let query = get_input("Search query: ");
    let results = repo.search(&query);
    
    if results.is_empty() {
        println!("\nNo users found matching '{}'", query);
        return Ok(());
    }
    
    println!("\nFound {} user(s):", results.len());
    println!("{:<5} {:<20} {:<30} {:<5}", "ID", "Name", "Email", "Age");
    println!("{:-<60}", "");
    
    for user in results {
        println!("{:<5} {:<20} {:<30} {:<5}", 
                 user.id(), 
                 truncate(user.name(), 20),
                 truncate(user.email(), 30),
                 user.age());
    }
    
    Ok(())
}

fn update_user(repo: &mut UserRepository) -> AppResult<()> {
    println!("\n--- Update User ---");
    
    let id_str = get_input("User ID: ");
    let id = id_str.parse::<u32>()
        .map_err(|_| "Invalid ID".to_string())?;
    
    if let Some(user) = repo.find_user(id) {
        println!("\nCurrent details:");
        println!("  Name: {}", user.name());
        println!("  Email: {}", user.email());
        println!("  Age: {}", user.age());
        
        let name = get_input("\nNew name (or press Enter to keep): ");
        let email = get_input("New email (or press Enter to keep): ");
        let age_str = get_input("New age (or press Enter to keep): ");
        
        let final_name = if name.is_empty() { user.name().to_string() } else { name };
        let final_email = if email.is_empty() { user.email().to_string() } else { email };
        let final_age = if age_str.is_empty() { 
            user.age() 
        } else { 
            age_str.parse().map_err(|_| "Invalid age".to_string())? 
        };
        
        repo.update_user(id, final_name, final_email, final_age)?;
        println!("\n✓ User updated successfully!");
    } else {
        println!("\n✗ User not found.");
    }
    
    Ok(())
}

fn delete_user(repo: &mut UserRepository) -> AppResult<()> {
    println!("\n--- Delete User ---");
    
    let id_str = get_input("User ID: ");
    let id = id_str.parse::<u32>()
        .map_err(|_| "Invalid ID".to_string())?;
    
    if let Some(user) = repo.find_user(id) {
        println!("\nUser to delete:");
        println!("  Name: {}", user.name());
        println!("  Email: {}", user.email());
        
        let confirm = get_input("\nConfirm deletion (yes/no): ");
        
        if confirm.to_lowercase() == "yes" {
            repo.delete_user(id)?;
            println!("\n✓ User deleted successfully!");
        } else {
            println!("\nDeletion cancelled.");
        }
    } else {
        println!("\n✗ User not found.");
    }
    
    Ok(())
}

fn show_statistics(repo: &UserRepository) -> AppResult<()> {
    println!("\n--- Statistics ---");
    
    let stats = repo.get_statistics();
    
    println!("Total Users: {}", stats.total_users);
    
    if stats.total_users > 0 {
        println!("\nAge Statistics:");
        println!("  Youngest: {}", stats.age_stats.0);
        println!("  Oldest: {}", stats.age_stats.1);
        println!("  Average: {:.1}", stats.age_stats.2);
        
        println!("\nEmail Domains:");
        for (domain, count) in &stats.email_domains {
            println!("  {}: {}", domain, count);
        }
        
        println!("\nName Length:");
        println!("  Shortest: {}", stats.name_lengths.0);
        println!("  Longest: {}", stats.name_lengths.1);
        println!("  Average: {:.1}", stats.name_lengths.2);
    }
    
    Ok(())
}

fn export_to_csv(repo: &UserRepository) -> AppResult<()> {
    println!("\n--- Export to CSV ---");
    
    let filename = get_input("CSV filename (default: users.csv): ");
    let filename = if filename.is_empty() {
        "users.csv".to_string()
    } else {
        filename
    };
    
    repo.export_to_csv(&filename)?;
    println!("\n✓ Exported {} users to '{}'", repo.count(), filename);
    
    Ok(())
}

fn import_from_csv(repo: &mut UserRepository) -> AppResult<()> {
    println!("\n--- Import from CSV ---");
    
    let filename = get_input("CSV filename: ");
    
    let count = repo.import_from_csv(&filename)?;
    println!("\n✓ Imported {} users from '{}'", count, filename);
    
    Ok(())
}

fn save_data(repo: &UserRepository) -> AppResult<()> {
    println!("\n--- Save Data ---");
    
    let filename = get_input("Filename (default: users.json): ");
    let filename = if filename.is_empty() {
        "users.json".to_string()
    } else {
        filename
    };
    
    repo.save_to_json(&filename)?;
    println!("\n✓ Saved {} users to '{}'", repo.count(), filename);
    
    Ok(())
}

fn load_data(repo: &mut UserRepository) -> AppResult<()> {
    println!("\n--- Load Data ---");
    
    let filename = get_input("Filename (default: users.json): ");
    let filename = if filename.is_empty() {
        "users.json".to_string()
    } else {
        filename
    };
    
    let count = repo.load_from_json(&filename)?;
    println!("\n✓ Loaded {} users from '{}'", count, filename);
    
    Ok(())
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_creation() {
        let config = AppConfig::default();
        assert!(!config.app.name.is_empty());
    }
}
