// Basic arithmetic operations

/// Adds two numbers
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// Subtracts b from a
pub fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

/// Multiplies two numbers
pub fn multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Divides a by b
/// Returns None if b is zero
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_eq!(add(-2.0, 3.0), 1.0);
    }
    
    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(subtract(3.0, 5.0), -2.0);
    }
    
    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4.0, 5.0), 20.0);
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }
    
    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }
}
