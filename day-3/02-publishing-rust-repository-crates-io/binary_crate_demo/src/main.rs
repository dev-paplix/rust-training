use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

/// A demonstration CLI tool for learning how to publish binary crates
#[derive(Parser)]
#[command(name = "demo-tool")]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet {
        /// Name of the person to greet
        name: String,
        
        /// Number of times to greet
        #[arg(short, long, default_value_t = 1)]
        count: u32,
    },
    
    /// Calculate sum of numbers
    Sum {
        /// Numbers to sum
        numbers: Vec<i32>,
    },
    
    /// Convert JSON to pretty format
    Json {
        /// JSON string to format
        json: String,
    },
    
    /// Show system information
    Info,
}

#[derive(Serialize, Deserialize)]
struct SystemInfo {
    os: String,
    architecture: String,
    version: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Greet { name, count } => {
            greet_command(&name, count)?;
        }
        Commands::Sum { numbers } => {
            sum_command(&numbers)?;
        }
        Commands::Json { json } => {
            json_command(&json)?;
        }
        Commands::Info => {
            info_command()?;
        }
    }

    Ok(())
}

fn greet_command(name: &str, count: u32) -> Result<()> {
    for i in 1..=count {
        if count > 1 {
            println!("{} {}: Hello, {}!", 
                "Greeting".green().bold(), 
                i, 
                name.cyan().bold()
            );
        } else {
            println!("{} Hello, {}!", 
                "→".green(), 
                name.cyan().bold()
            );
        }
    }
    Ok(())
}

fn sum_command(numbers: &[i32]) -> Result<()> {
    if numbers.is_empty() {
        anyhow::bail!("No numbers provided");
    }

    let sum: i32 = numbers.iter().sum();
    
    println!("{} Calculating sum...", "→".green());
    println!("  Numbers: {}", 
        numbers.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(" + ")
    );
    println!("  {}: {}", "Result".yellow().bold(), sum.to_string().cyan().bold());
    
    Ok(())
}

fn json_command(json_str: &str) -> Result<()> {
    let value: serde_json::Value = serde_json::from_str(json_str)
        .context("Failed to parse JSON")?;
    
    let pretty = serde_json::to_string_pretty(&value)
        .context("Failed to format JSON")?;
    
    println!("{} Formatted JSON:", "→".green());
    println!("{}", pretty);
    
    Ok(())
}

fn info_command() -> Result<()> {
    let info = SystemInfo {
        os: std::env::consts::OS.to_string(),
        architecture: std::env::consts::ARCH.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    println!("{}", "System Information".green().bold().underline());
    println!("  {}: {}", "OS".yellow(), info.os.cyan());
    println!("  {}: {}", "Architecture".yellow(), info.architecture.cyan());
    println!("  {}: {}", "Tool Version".yellow(), info.version.cyan());
    
    // JSON output option
    let json = serde_json::to_string_pretty(&info)?;
    println!("\n{} JSON Format:", "→".green());
    println!("{}", json);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_command() {
        let result = greet_command("Alice", 1);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sum_command() {
        let result = sum_command(&[1, 2, 3, 4, 5]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sum_command_empty() {
        let result = sum_command(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_json_command() {
        let json = r#"{"name":"test","value":42}"#;
        let result = json_command(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_json_command_invalid() {
        let json = "invalid json";
        let result = json_command(json);
        assert!(result.is_err());
    }
}
