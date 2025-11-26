//! Data parsing functionality

use crate::{DataError, Result};

/// Parser for converting strings to various types
pub struct Parser {
    data: String,
}

impl Parser {
    /// Creates a new parser with the given data
    pub fn new(data: String) -> Self {
        Parser { data }
    }

    /// Parses data as an integer
    /// 
    /// # Examples
    /// 
    /// ```
    /// use data_processor_lib::Parser;
    /// let parser = Parser::new("42".to_string());
    /// assert_eq!(parser.parse_int().unwrap(), 42);
    /// ```
    pub fn parse_int(&self) -> Result<i32> {
        self.data
            .trim()
            .parse::<i32>()
            .map_err(|e| DataError::ParseError(e.to_string()))
    }

    /// Parses data as a float
    pub fn parse_float(&self) -> Result<f64> {
        self.data
            .trim()
            .parse::<f64>()
            .map_err(|e| DataError::ParseError(e.to_string()))
    }

    /// Parses data as a boolean
    pub fn parse_bool(&self) -> Result<bool> {
        let trimmed = self.data.trim().to_lowercase();
        match trimmed.as_str() {
            "true" | "1" | "yes" | "y" => Ok(true),
            "false" | "0" | "no" | "n" => Ok(false),
            _ => Err(DataError::ParseError(format!("Invalid boolean value: {}", self.data))),
        }
    }

    /// Parses comma-separated values
    pub fn parse_csv(&self) -> Vec<String> {
        self.data
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }

    /// Parses key-value pairs (format: "key=value")
    pub fn parse_key_value(&self) -> Result<(String, String)> {
        let parts: Vec<&str> = self.data.split('=').collect();
        if parts.len() != 2 {
            return Err(DataError::ParseError("Invalid key=value format".to_string()));
        }
        
        Ok((parts[0].trim().to_string(), parts[1].trim().to_string()))
    }

    /// Gets the raw data
    pub fn data(&self) -> &str {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        let parser = Parser::new("42".to_string());
        assert_eq!(parser.parse_int().unwrap(), 42);

        let parser = Parser::new("  123  ".to_string());
        assert_eq!(parser.parse_int().unwrap(), 123);

        let parser = Parser::new("invalid".to_string());
        assert!(parser.parse_int().is_err());
    }

    #[test]
    fn test_parse_float() {
        let parser = Parser::new("3.14".to_string());
        assert!((parser.parse_float().unwrap() - 3.14).abs() < 0.01);
    }

    #[test]
    fn test_parse_bool() {
        let parser = Parser::new("true".to_string());
        assert_eq!(parser.parse_bool().unwrap(), true);

        let parser = Parser::new("0".to_string());
        assert_eq!(parser.parse_bool().unwrap(), false);

        let parser = Parser::new("invalid".to_string());
        assert!(parser.parse_bool().is_err());
    }

    #[test]
    fn test_parse_csv() {
        let parser = Parser::new("a, b, c".to_string());
        assert_eq!(parser.parse_csv(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_parse_key_value() {
        let parser = Parser::new("name=John".to_string());
        let (key, value) = parser.parse_key_value().unwrap();
        assert_eq!(key, "name");
        assert_eq!(value, "John");
    }
}
