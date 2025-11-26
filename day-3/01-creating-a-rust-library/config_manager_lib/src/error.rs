//! Error types for configuration management

use std::fmt;

/// Configuration error types
#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    /// Missing required field
    MissingField(String),
    
    /// Invalid value for field
    InvalidValue(String),
    
    /// Validation error
    ValidationError(String),
    
    /// Parse error
    ParseError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ConfigError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ConfigError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Convenient Result type alias
pub type Result<T> = std::result::Result<T, ConfigError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ConfigError::MissingField("host".to_string());
        assert_eq!(err.to_string(), "Missing required field: host");
    }
}
