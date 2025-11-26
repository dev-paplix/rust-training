use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub path: String,
    pub auto_save: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub file: Option<String>,
}

impl AppConfig {
    pub fn load_from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;
        
        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse config: {}", e))
    }
    
    pub fn save_to_file(&self, path: &str) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app: AppSettings {
                name: "Full Featured App".to_string(),
                version: "1.0.0".to_string(),
                environment: "development".to_string(),
                debug: true,
            },
            database: DatabaseSettings {
                path: "users.json".to_string(),
                auto_save: true,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                file: Some("app.log".to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app.name, "Full Featured App");
        assert!(config.app.debug);
    }
}
