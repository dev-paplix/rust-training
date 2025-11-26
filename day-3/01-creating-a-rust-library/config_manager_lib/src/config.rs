//! Application configuration

use crate::{ConfigBuilder, Result};

/// Application configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    host: String,
    port: u16,
    database_url: Option<String>,
    max_connections: u32,
    timeout_seconds: u64,
    debug_mode: bool,
}

impl AppConfig {
    /// Creates a new configuration builder
    /// 
    /// # Examples
    /// 
    /// ```
    /// use config_manager_lib::AppConfig;
    /// 
    /// let config = AppConfig::builder()
    ///     .host("localhost")
    ///     .port(8080)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::new()
    }

    /// Gets the host
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Gets the port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Gets the database URL if set
    pub fn database_url(&self) -> Option<&str> {
        self.database_url.as_deref()
    }

    /// Gets the maximum connections
    pub fn max_connections(&self) -> u32 {
        self.max_connections
    }

    /// Gets the timeout in seconds
    pub fn timeout_seconds(&self) -> u64 {
        self.timeout_seconds
    }

    /// Checks if debug mode is enabled
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }

    /// Creates config with default values
    pub(crate) fn new(
        host: String,
        port: u16,
        database_url: Option<String>,
        max_connections: u32,
        timeout_seconds: u64,
        debug_mode: bool,
    ) -> Self {
        AppConfig {
            host,
            port,
            database_url,
            max_connections,
            timeout_seconds,
            debug_mode,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = AppConfig::builder()
            .host("localhost")
            .port(8080)
            .build()
            .unwrap();

        assert_eq!(config.host(), "localhost");
        assert_eq!(config.port(), 8080);
    }

    #[test]
    fn test_config_with_database() {
        let config = AppConfig::builder()
            .host("localhost")
            .port(5432)
            .database_url("postgres://localhost/mydb")
            .build()
            .unwrap();

        assert_eq!(config.database_url(), Some("postgres://localhost/mydb"));
    }
}
