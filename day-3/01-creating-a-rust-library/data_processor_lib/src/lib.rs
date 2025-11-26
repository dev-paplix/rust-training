//! # Data Processor Library
//! 
//! A library for parsing, validating, and processing data.
//! 
//! # Examples
//! 
//! ```
//! use data_processor_lib::{Parser, Validator};
//! 
//! let parser = Parser::new("42".to_string());
//! assert_eq!(parser.parse_int().unwrap(), 42);
//! 
//! let validator = Validator;
//! assert!(validator.is_valid_age(25));
//! ```

pub mod error;
pub mod parser;
pub mod validator;
pub mod processor;

pub use error::{DataError, Result};
pub use parser::Parser;
pub use validator::Validator;
pub use processor::DataProcessor;

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
