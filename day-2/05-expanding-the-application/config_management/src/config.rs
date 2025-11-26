use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub app: AppSettings,
    pub server: ServerSettings,
    pub database: DatabaseSettings,
    pub logging: LoggingSettings,
    pub features: HashMap<String, bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub environment: String,
    pub debug: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
    pub workers: usize,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseSettings {
    pub url: String,
    pub pool_size: u32,
    pub timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub format: String,
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
    
    pub fn development() -> Self {
        let mut features = HashMap::new();
        features.insert("api_v2".to_string(), true);
        features.insert("analytics".to_string(), false);
        features.insert("caching".to_string(), false);
        
        Self {
            app: AppSettings {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                environment: "development".to_string(),
                debug: true,
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 3000,
                workers: 2,
                max_connections: 100,
            },
            database: DatabaseSettings {
                url: "sqlite://dev.db".to_string(),
                pool_size: 5,
                timeout_seconds: 30,
            },
            logging: LoggingSettings {
                level: "debug".to_string(),
                format: "pretty".to_string(),
                file: Some("logs/dev.log".to_string()),
            },
            features,
        }
    }
    
    pub fn production() -> Self {
        let mut features = HashMap::new();
        features.insert("api_v2".to_string(), true);
        features.insert("analytics".to_string(), true);
        features.insert("caching".to_string(), true);
        
        Self {
            app: AppSettings {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                environment: "production".to_string(),
                debug: false,
            },
            server: ServerSettings {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: 8,
                max_connections: 1000,
            },
            database: DatabaseSettings {
                url: "postgresql://user:pass@db.example.com/prod".to_string(),
                pool_size: 20,
                timeout_seconds: 10,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                format: "json".to_string(),
                file: Some("/var/log/myapp/prod.log".to_string()),
            },
            features,
        }
    }
    
    pub fn testing() -> Self {
        let mut features = HashMap::new();
        features.insert("api_v2".to_string(), true);
        features.insert("analytics".to_string(), false);
        features.insert("caching".to_string(), false);
        
        Self {
            app: AppSettings {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                environment: "testing".to_string(),
                debug: true,
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 0, // Random port
                workers: 1,
                max_connections: 10,
            },
            database: DatabaseSettings {
                url: "sqlite::memory:".to_string(),
                pool_size: 1,
                timeout_seconds: 5,
            },
            logging: LoggingSettings {
                level: "debug".to_string(),
                format: "pretty".to_string(),
                file: None,
            },
            features,
        }
    }
    
    pub fn is_feature_enabled(&self, feature: &str) -> bool {
        self.features.get(feature).copied().unwrap_or(false)
    }
    
    pub fn enable_feature(&mut self, feature: &str) {
        self.features.insert(feature.to_string(), true);
    }
    
    pub fn disable_feature(&mut self, feature: &str) {
        self.features.insert(feature.to_string(), false);
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        let mut features = HashMap::new();
        features.insert("api_v2".to_string(), false);
        features.insert("analytics".to_string(), false);
        features.insert("caching".to_string(), false);
        
        Self {
            app: AppSettings {
                name: "MyApp".to_string(),
                version: "1.0.0".to_string(),
                environment: "development".to_string(),
                debug: false,
            },
            server: ServerSettings {
                host: "127.0.0.1".to_string(),
                port: 8080,
                workers: 4,
                max_connections: 500,
            },
            database: DatabaseSettings {
                url: "sqlite://app.db".to_string(),
                pool_size: 10,
                timeout_seconds: 30,
            },
            logging: LoggingSettings {
                level: "info".to_string(),
                format: "json".to_string(),
                file: Some("logs/app.log".to_string()),
            },
            features,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app.name, "MyApp");
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.database.pool_size, 10);
    }
    
    #[test]
    fn test_environment_configs() {
        let dev = AppConfig::development();
        assert_eq!(dev.app.environment, "development");
        assert!(dev.app.debug);
        
        let prod = AppConfig::production();
        assert_eq!(prod.app.environment, "production");
        assert!(!prod.app.debug);
        
        let test = AppConfig::testing();
        assert_eq!(test.app.environment, "testing");
        assert_eq!(test.database.url, "sqlite::memory:");
    }
    
    #[test]
    fn test_feature_flags() {
        let mut config = AppConfig::default();
        
        config.enable_feature("new_feature");
        assert!(config.is_feature_enabled("new_feature"));
        
        config.disable_feature("new_feature");
        assert!(!config.is_feature_enabled("new_feature"));
    }
    
    #[test]
    fn test_save_and_load() {
        let config = AppConfig::default();
        let path = "test_config.toml";
        
        config.save_to_file(path).unwrap();
        let loaded = AppConfig::load_from_file(path).unwrap();
        
        assert_eq!(config.app.name, loaded.app.name);
        assert_eq!(config.server.port, loaded.server.port);
        
        std::fs::remove_file(path).unwrap();
    }
}
