//! # Configuration Manager Library
//! 
//! A library for managing application configuration with builder pattern.
//! 
//! # Examples
//! 
//! ```
//! use config_manager_lib::AppConfig;
//! 
//! let config = AppConfig::builder()
//!     .host("localhost")
//!     .port(8080)
//!     .build()
//!     .unwrap();
//! 
//! assert_eq!(config.host(), "localhost");
//! assert_eq!(config.port(), 8080);
//! ```

pub mod error;
pub mod config;
pub mod builder;

pub use error::{ConfigError, Result};
pub use config::AppConfig;
pub use builder::ConfigBuilder;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
