use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod config;
use config::AppConfig;

/// Demonstrates configuration management using TOML files
fn main() {
    println!("=== Configuration Management Demo ===\n");
    
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => create_default_config(),
            2 => load_and_display_config(),
            3 => update_config(),
            4 => validate_config(),
            5 => show_config_examples(),
            6 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
        
        println!();
    }
}

fn display_menu() {
    println!("\n--- Configuration Management ---");
    println!("1. Create Default Config");
    println!("2. Load and Display Config");
    println!("3. Update Configuration");
    println!("4. Validate Configuration");
    println!("5. Show Config Examples");
    println!("6. Exit");
}

fn get_user_choice() -> u32 {
    use std::io::{self, Write};
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    input.trim().parse().unwrap_or(0)
}

fn get_input(prompt: &str) -> String {
    use std::io::{self, Write};
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn create_default_config() {
    println!("\n--- Create Default Configuration ---");
    
    let config = AppConfig::default();
    let filename = get_input("Config filename (default: config.toml): ");
    let filename = if filename.is_empty() {
        "config.toml".to_string()
    } else {
        filename
    };
    
    match config.save_to_file(&filename) {
        Ok(()) => {
            println!("\n✓ Configuration saved to '{}'", filename);
            println!("\nDefault configuration:");
            println!("{:#?}", config);
        }
        Err(e) => println!("\n✗ Error saving config: {}", e),
    }
}

fn load_and_display_config() {
    println!("\n--- Load Configuration ---");
    
    let filename = get_input("Config filename (default: config.toml): ");
    let filename = if filename.is_empty() {
        "config.toml".to_string()
    } else {
        filename
    };
    
    if !Path::new(&filename).exists() {
        println!("\n✗ File '{}' not found.", filename);
        return;
    }
    
    match AppConfig::load_from_file(&filename) {
        Ok(config) => {
            println!("\n✓ Configuration loaded from '{}'", filename);
            println!("\nConfiguration:");
            println!("{:#?}", config);
            
            println!("\n--- Settings Summary ---");
            println!("Application: {} v{}", config.app.name, config.app.version);
            println!("Environment: {}", config.app.environment);
            println!("Debug Mode: {}", config.app.debug);
            
            println!("\nServer:");
            println!("  Host: {}", config.server.host);
            println!("  Port: {}", config.server.port);
            println!("  Workers: {}", config.server.workers);
            println!("  Max Connections: {}", config.server.max_connections);
            
            println!("\nDatabase:");
            println!("  URL: {}", config.database.url);
            println!("  Pool Size: {}", config.database.pool_size);
            
            println!("\nLogging:");
            println!("  Level: {}", config.logging.level);
            println!("  Format: {}", config.logging.format);
            println!("  File: {:?}", config.logging.file);
            
            if !config.features.is_empty() {
                println!("\nFeature Flags:");
                for (key, value) in &config.features {
                    println!("  {}: {}", key, value);
                }
            }
        }
        Err(e) => println!("\n✗ Error loading config: {}", e),
    }
}

fn update_config() {
    println!("\n--- Update Configuration ---");
    
    let filename = get_input("Config filename (default: config.toml): ");
    let filename = if filename.is_empty() {
        "config.toml".to_string()
    } else {
        filename
    };
    
    let mut config = match AppConfig::load_from_file(&filename) {
        Ok(c) => c,
        Err(_) => {
            println!("\nFile not found. Creating new configuration.");
            AppConfig::default()
        }
    };
    
    println!("\nWhat would you like to update?");
    println!("1. Application Name");
    println!("2. Environment");
    println!("3. Server Port");
    println!("4. Debug Mode");
    println!("5. Log Level");
    
    let choice = get_input("\nChoice: ");
    
    match choice.as_str() {
        "1" => {
            let name = get_input("New application name: ");
            config.app.name = name;
        }
        "2" => {
            let env = get_input("Environment (development/staging/production): ");
            config.app.environment = env;
        }
        "3" => {
            let port = get_input("New port number: ");
            if let Ok(port) = port.parse::<u16>() {
                config.server.port = port;
            } else {
                println!("Invalid port number.");
                return;
            }
        }
        "4" => {
            let debug = get_input("Enable debug mode? (yes/no): ");
            config.app.debug = debug.to_lowercase() == "yes";
        }
        "5" => {
            let level = get_input("Log level (debug/info/warn/error): ");
            config.logging.level = level;
        }
        _ => {
            println!("Invalid choice.");
            return;
        }
    }
    
    match config.save_to_file(&filename) {
        Ok(()) => println!("\n✓ Configuration updated and saved."),
        Err(e) => println!("\n✗ Error saving config: {}", e),
    }
}

fn validate_config() {
    println!("\n--- Validate Configuration ---");
    
    let filename = get_input("Config filename (default: config.toml): ");
    let filename = if filename.is_empty() {
        "config.toml".to_string()
    } else {
        filename
    };
    
    match AppConfig::load_from_file(&filename) {
        Ok(config) => {
            println!("\n✓ Configuration file is valid.");
            
            let mut warnings = Vec::new();
            
            // Validation checks
            if config.server.port < 1024 {
                warnings.push("Port < 1024 requires root privileges");
            }
            
            if config.server.workers == 0 {
                warnings.push("Workers should be > 0");
            }
            
            if config.database.pool_size == 0 {
                warnings.push("Database pool size should be > 0");
            }
            
            if !["debug", "info", "warn", "error"].contains(&config.logging.level.as_str()) {
                warnings.push("Unknown log level");
            }
            
            if config.app.debug && config.app.environment == "production" {
                warnings.push("Debug mode enabled in production environment");
            }
            
            if warnings.is_empty() {
                println!("No issues found.");
            } else {
                println!("\n⚠ Warnings:");
                for warning in warnings {
                    println!("  • {}", warning);
                }
            }
        }
        Err(e) => println!("\n✗ Invalid configuration: {}", e),
    }
}

fn show_config_examples() {
    println!("\n--- Configuration Examples ---\n");
    
    println!("Development Config:");
    println!("-------------------");
    let dev_config = AppConfig::development();
    println!("{}", toml::to_string_pretty(&dev_config).unwrap());
    
    println!("\nProduction Config:");
    println!("------------------");
    let prod_config = AppConfig::production();
    println!("{}", toml::to_string_pretty(&prod_config).unwrap());
    
    println!("\nTesting Config:");
    println!("---------------");
    let test_config = AppConfig::testing();
    println!("{}", toml::to_string_pretty(&test_config).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app.name, "MyApp");
        assert_eq!(config.server.port, 8080);
    }
    
    #[test]
    fn test_config_save_load() {
        let config = AppConfig::default();
        let filename = "test_config.toml";
        
        config.save_to_file(filename).unwrap();
        let loaded = AppConfig::load_from_file(filename).unwrap();
        
        assert_eq!(config.app.name, loaded.app.name);
        assert_eq!(config.server.port, loaded.server.port);
        
        // Cleanup
        let _ = fs::remove_file(filename);
    }
}
