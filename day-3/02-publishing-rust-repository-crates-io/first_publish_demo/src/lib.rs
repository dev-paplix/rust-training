//! # First Publish Demo
//!
//! A simple demonstration library for learning how to publish Rust crates to crates.io.
//!
//! This crate provides basic utility functions to demonstrate:
//! - Proper crate-level documentation
//! - Function documentation with examples
//! - Unit tests and doc tests
//! - Complete Cargo.toml metadata
//! - Licensing and README
//!
//! # Quick Start
//!
//! ```
//! use first_publish_demo::{greet, add, multiply};
//!
//! // Simple greeting
//! assert_eq!(greet("World"), "Hello, World!");
//!
//! // Math operations
//! assert_eq!(add(2, 3), 5);
//! assert_eq!(multiply(4, 5), 20);
//! ```
//!
//! # Publishing Checklist
//!
//! This crate demonstrates all requirements for publishing:
//! - ✅ Complete Cargo.toml with required fields
//! - ✅ LICENSE file (dual MIT/Apache-2.0)
//! - ✅ README.md with installation and usage
//! - ✅ Comprehensive documentation
//! - ✅ All tests passing
//! - ✅ Example code that compiles

/// The current version of the crate.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Greets a person by name.
///
/// # Examples
///
/// ```
/// use first_publish_demo::greet;
///
/// assert_eq!(greet("Alice"), "Hello, Alice!");
/// assert_eq!(greet("Bob"), "Hello, Bob!");
/// ```
///
/// # Arguments
///
/// * `name` - The name of the person to greet
///
/// # Returns
///
/// A greeting string
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// use first_publish_demo::add;
///
/// assert_eq!(add(2, 3), 5);
/// assert_eq!(add(-1, 1), 0);
/// assert_eq!(add(0, 0), 0);
/// ```
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The sum of `a` and `b`
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
///
/// # Examples
///
/// ```
/// use first_publish_demo::multiply;
///
/// assert_eq!(multiply(3, 4), 12);
/// assert_eq!(multiply(-2, 5), -10);
/// assert_eq!(multiply(0, 100), 0);
/// ```
///
/// # Arguments
///
/// * `a` - The first number
/// * `b` - The second number
///
/// # Returns
///
/// The product of `a` and `b`
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Checks if a number is even.
///
/// # Examples
///
/// ```
/// use first_publish_demo::is_even;
///
/// assert!(is_even(2));
/// assert!(is_even(0));
/// assert!(!is_even(1));
/// assert!(!is_even(-3));
/// ```
///
/// # Arguments
///
/// * `n` - The number to check
///
/// # Returns
///
/// `true` if the number is even, `false` otherwise
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Reverses a string.
///
/// # Examples
///
/// ```
/// use first_publish_demo::reverse_string;
///
/// assert_eq!(reverse_string("hello"), "olleh");
/// assert_eq!(reverse_string("Rust"), "tsuR");
/// assert_eq!(reverse_string(""), "");
/// ```
///
/// # Arguments
///
/// * `s` - The string to reverse
///
/// # Returns
///
/// A new string with characters in reverse order
pub fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

/// A simple calculator struct demonstrating struct documentation.
///
/// # Examples
///
/// ```
/// use first_publish_demo::Calculator;
///
/// let mut calc = Calculator::new();
/// calc.add(10);
/// calc.multiply(2);
/// assert_eq!(calc.result(), 20);
/// ```
pub struct Calculator {
    value: i32,
}

impl Calculator {
    /// Creates a new calculator with initial value of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use first_publish_demo::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.result(), 0);
    /// ```
    pub fn new() -> Self {
        Self { value: 0 }
    }

    /// Adds a number to the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use first_publish_demo::Calculator;
    ///
    /// let mut calc = Calculator::new();
    /// calc.add(5);
    /// assert_eq!(calc.result(), 5);
    /// ```
    pub fn add(&mut self, n: i32) {
        self.value += n;
    }

    /// Multiplies the current value by a number.
    ///
    /// # Examples
    ///
    /// ```
    /// use first_publish_demo::Calculator;
    ///
    /// let mut calc = Calculator::new();
    /// calc.add(10);
    /// calc.multiply(3);
    /// assert_eq!(calc.result(), 30);
    /// ```
    pub fn multiply(&mut self, n: i32) {
        self.value *= n;
    }

    /// Returns the current value.
    ///
    /// # Examples
    ///
    /// ```
    /// use first_publish_demo::Calculator;
    ///
    /// let calc = Calculator::new();
    /// assert_eq!(calc.result(), 0);
    /// ```
    pub fn result(&self) -> i32 {
        self.value
    }

    /// Resets the calculator to 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use first_publish_demo::Calculator;
    ///
    /// let mut calc = Calculator::new();
    /// calc.add(100);
    /// calc.reset();
    /// assert_eq!(calc.result(), 0);
    /// ```
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(-5, -3), -8);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 100), 0);
        assert_eq!(multiply(-3, -3), 9);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(is_even(0));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(-3));
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
        assert_eq!(reverse_string(""), "");
        assert_eq!(reverse_string("a"), "a");
    }

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new();
        assert_eq!(calc.result(), 0);

        calc.add(10);
        assert_eq!(calc.result(), 10);

        calc.multiply(2);
        assert_eq!(calc.result(), 20);

        calc.add(-5);
        assert_eq!(calc.result(), 15);

        calc.reset();
        assert_eq!(calc.result(), 0);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert!(VERSION.starts_with("0."));
    }
}
