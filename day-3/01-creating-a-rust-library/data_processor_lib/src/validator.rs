//! Data validation functionality

use crate::{DataError, Result};

/// Validator for checking data constraints
pub struct Validator;

impl Validator {
    /// Validates that a number is within a range
    /// 
    /// # Examples
    /// 
    /// ```
    /// use data_processor_lib::Validator;
    /// let validator = Validator;
    /// assert!(validator.is_in_range(5, 0, 10));
    /// assert!(!validator.is_in_range(15, 0, 10));
    /// ```
    pub fn is_in_range(&self, value: i32, min: i32, max: i32) -> bool {
        value >= min && value <= max
    }

    /// Validates that a string is not empty
    pub fn is_not_empty(&self, s: &str) -> Result<()> {
        if s.trim().is_empty() {
            Err(DataError::ValidationError("String is empty".to_string()))
        } else {
            Ok(())
        }
    }

    /// Validates that a string has a minimum length
    pub fn min_length(&self, s: &str, min: usize) -> Result<()> {
        if s.len() < min {
            Err(DataError::ValidationError(format!("String too short, minimum length is {}", min)))
        } else {
            Ok(())
        }
    }

    /// Validates that a string has a maximum length
    pub fn max_length(&self, s: &str, max: usize) -> Result<()> {
        if s.len() > max {
            Err(DataError::ValidationError(format!("String too long, maximum length is {}", max)))
        } else {
            Ok(())
        }
    }

    /// Validates that a number is positive
    pub fn is_positive(&self, n: i32) -> Result<()> {
        if n > 0 {
            Ok(())
        } else {
            Err(DataError::ValidationError("Number must be positive".to_string()))
        }
    }

    /// Validates that an age is valid (0-150)
    pub fn is_valid_age(&self, age: i32) -> bool {
        age >= 0 && age <= 150
    }

    /// Validates email format (basic)
    pub fn is_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }

    /// Validates that a collection is not empty
    pub fn is_not_empty_vec<T>(&self, vec: &[T]) -> Result<()> {
        if vec.is_empty() {
            Err(DataError::Empty)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_in_range() {
        let validator = Validator;
        assert!(validator.is_in_range(5, 0, 10));
        assert!(!validator.is_in_range(15, 0, 10));
    }

    #[test]
    fn test_is_not_empty() {
        let validator = Validator;
        assert!(validator.is_not_empty("hello").is_ok());
        assert!(validator.is_not_empty("   ").is_err());
    }

    #[test]
    fn test_min_length() {
        let validator = Validator;
        assert!(validator.min_length("hello", 3).is_ok());
        assert!(validator.min_length("hi", 5).is_err());
    }

    #[test]
    fn test_max_length() {
        let validator = Validator;
        assert!(validator.max_length("hello", 10).is_ok());
        assert!(validator.max_length("hello world", 5).is_err());
    }

    #[test]
    fn test_is_positive() {
        let validator = Validator;
        assert!(validator.is_positive(5).is_ok());
        assert!(validator.is_positive(-5).is_err());
    }

    #[test]
    fn test_is_valid_age() {
        let validator = Validator;
        assert!(validator.is_valid_age(25));
        assert!(!validator.is_valid_age(-1));
        assert!(!validator.is_valid_age(200));
    }

    #[test]
    fn test_is_valid_email() {
        let validator = Validator;
        assert!(validator.is_valid_email("user@example.com"));
        assert!(!validator.is_valid_email("invalid"));
    }
}
