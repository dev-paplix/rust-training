//! # Simple Math Library
//! 
//! A comprehensive mathematical library providing basic arithmetic,
//! statistics, and a calculator with method chaining.
//! 
//! # Examples
//! 
//! ```
//! use simple_math_lib::{Calculator, Statistics};
//! 
//! // Using the calculator
//! let mut calc = Calculator::new(10.0);
//! calc.add(5.0).multiply(2.0).subtract(10.0);
//! assert_eq!(calc.result(), 20.0);
//! 
//! // Using statistics
//! let numbers = vec![1.0, 2.0, 3.0, 4.0, 5.0];
//! assert_eq!(Statistics::mean(&numbers), Some(3.0));
//! ```

pub mod calculator;
pub mod statistics;
pub mod geometry;

pub use calculator::Calculator;
pub use statistics::Statistics;
pub use geometry::{Circle, Rectangle, Triangle};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Adds two numbers
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::add;
/// assert_eq!(add(2.0, 3.0), 5.0);
/// ```
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts two numbers
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::subtract;
/// assert_eq!(subtract(5.0, 3.0), 2.0);
/// ```
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::multiply;
/// assert_eq!(multiply(4.0, 3.0), 12.0);
/// ```
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides two numbers
/// 
/// Returns `None` if divisor is zero
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::divide;
/// assert_eq!(divide(10.0, 2.0), Some(5.0));
/// assert_eq!(divide(10.0, 0.0), None);
/// ```
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Calculates the power of a number
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::power;
/// assert_eq!(power(2.0, 3.0), 8.0);
/// ```
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Calculates the square root
/// 
/// Returns `None` for negative numbers
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::sqrt;
/// assert_eq!(sqrt(16.0), Some(4.0));
/// assert_eq!(sqrt(-1.0), None);
/// ```
pub fn sqrt(n: f64) -> Option<f64> {
    if n < 0.0 {
        None
    } else {
        Some(n.sqrt())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-1.0, 1.0), 0.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(subtract(0.0, 5.0), -5.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4.0, 3.0), 12.0);
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_power() {
        assert_eq!(power(2.0, 3.0), 8.0);
        assert_eq!(power(5.0, 2.0), 25.0);
    }

    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(16.0), Some(4.0));
        assert_eq!(sqrt(0.0), Some(0.0));
        assert_eq!(sqrt(-1.0), None);
    }

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
