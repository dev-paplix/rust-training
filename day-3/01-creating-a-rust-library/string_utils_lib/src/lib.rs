//! # String Utilities Library
//! 
//! A comprehensive library for string manipulation and analysis.
//! 
//! # Examples
//! 
//! ```
//! use string_utils_lib::{capitalize, reverse, count_vowels};
//! 
//! assert_eq!(capitalize("hello"), "Hello");
//! assert_eq!(reverse("hello"), "olleh");
//! assert_eq!(count_vowels("hello"), 2);
//! ```

pub mod transform;
pub mod validate;
pub mod count;

pub use transform::{capitalize, reverse, to_title_case, to_snake_case, to_camel_case};
pub use validate::{is_palindrome, is_email, is_url};
pub use count::{count_vowels, count_consonants, count_words};

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
