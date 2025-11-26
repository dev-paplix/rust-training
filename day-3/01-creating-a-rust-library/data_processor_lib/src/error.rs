//! Error types for data processing

use std::fmt;

/// Main error type for the library
#[derive(Debug, Clone, PartialEq)]
pub enum DataError {
    /// Parse error with description
    ParseError(String),
    
    /// Validation error with description
    ValidationError(String),
    
    /// Invalid input error
    InvalidInput(String),
    
    /// Not found error
    NotFound,
    
    /// Empty data error
    Empty,
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DataError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            DataError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            DataError::NotFound => write!(f, "Resource not found"),
            DataError::Empty => write!(f, "Data is empty"),
        }
    }
}

impl std::error::Error for DataError {}

/// Convenient Result type alias
pub type Result<T> = std::result::Result<T, DataError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = DataError::ParseError("invalid number".to_string());
        assert_eq!(err.to_string(), "Parse error: invalid number");
    }

    #[test]
    fn test_error_equality() {
        let err1 = DataError::NotFound;
        let err2 = DataError::NotFound;
        assert_eq!(err1, err2);
    }
}
