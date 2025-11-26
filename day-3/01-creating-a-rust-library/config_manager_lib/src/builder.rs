//! Configuration builder

use crate::{AppConfig, ConfigError, Result};

/// Builder for AppConfig
#[derive(Debug, Default)]
pub struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    database_url: Option<String>,
    max_connections: Option<u32>,
    timeout_seconds: Option<u64>,
    debug_mode: Option<bool>,
}

impl ConfigBuilder {
    /// Creates a new configuration builder
    pub fn new() -> Self {
        ConfigBuilder::default()
    }

    /// Sets the host
    /// 
    /// # Examples
    /// 
    /// ```
    /// use config_manager_lib::AppConfig;
    /// 
    /// let config = AppConfig::builder()
    ///     .host("127.0.0.1")
    ///     .port(8080)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Sets the port
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    /// Sets the database URL
    pub fn database_url(mut self, url: impl Into<String>) -> Self {
        self.database_url = Some(url.into());
        self
    }

    /// Sets the maximum number of connections
    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = Some(max);
        self
    }

    /// Sets the timeout in seconds
    pub fn timeout_seconds(mut self, timeout: u64) -> Self {
        self.timeout_seconds = Some(timeout);
        self
    }

    /// Sets debug mode
    pub fn debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = Some(debug);
        self
    }

    /// Builds the configuration
    /// 
    /// Returns an error if required fields are missing or invalid
    pub fn build(self) -> Result<AppConfig> {
        let host = self.host.ok_or_else(|| ConfigError::MissingField("host".to_string()))?;
        
        if host.is_empty() {
            return Err(ConfigError::InvalidValue("Host cannot be empty".to_string()));
        }

        let port = self.port.ok_or_else(|| ConfigError::MissingField("port".to_string()))?;

        let max_connections = self.max_connections.unwrap_or(100);
        if max_connections == 0 {
            return Err(ConfigError::InvalidValue("Max connections must be greater than 0".to_string()));
        }

        let timeout_seconds = self.timeout_seconds.unwrap_or(30);
        let debug_mode = self.debug_mode.unwrap_or(false);

        Ok(AppConfig::new(
            host,
            port,
            self.database_url,
            max_connections,
            timeout_seconds,
            debug_mode,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_success() {
        let config = ConfigBuilder::new()
            .host("localhost")
            .port(8080)
            .build()
            .unwrap();

        assert_eq!(config.host(), "localhost");
        assert_eq!(config.port(), 8080);
    }

    #[test]
    fn test_builder_missing_host() {
        let result = ConfigBuilder::new()
            .port(8080)
            .build();

        assert!(matches!(result, Err(ConfigError::MissingField(_))));
    }

    #[test]
    fn test_builder_empty_host() {
        let result = ConfigBuilder::new()
            .host("")
            .port(8080)
            .build();

        assert!(matches!(result, Err(ConfigError::InvalidValue(_))));
    }

    #[test]
    fn test_builder_with_all_options() {
        let config = ConfigBuilder::new()
            .host("localhost")
            .port(5432)
            .database_url("postgres://localhost/test")
            .max_connections(50)
            .timeout_seconds(60)
            .debug_mode(true)
            .build()
            .unwrap();

        assert_eq!(config.host(), "localhost");
        assert_eq!(config.port(), 5432);
        assert_eq!(config.max_connections(), 50);
        assert_eq!(config.timeout_seconds(), 60);
        assert!(config.is_debug_mode());
    }

    #[test]
    fn test_builder_defaults() {
        let config = ConfigBuilder::new()
            .host("localhost")
            .port(8080)
            .build()
            .unwrap();

        assert_eq!(config.max_connections(), 100);
        assert_eq!(config.timeout_seconds(), 30);
        assert!(!config.is_debug_mode());
    }
}
