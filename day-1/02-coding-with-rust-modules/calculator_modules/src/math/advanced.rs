// Advanced mathematical operations

/// Calculates base raised to the power of exponent
pub fn power(base: f64, exponent: f64) -> f64 {
    base.powf(exponent)
}

/// Calculates square root
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

/// Calculates factorial (only for reasonable sizes)
pub fn factorial(n: u32) -> u64 {
    if n > 20 {
        panic!("Factorial too large! Maximum is 20");
    }
    (1..=n as u64).product()
}

/// Calculates absolute value
pub fn abs(x: f64) -> f64 {
    x.abs()
}

/// Rounds to nearest integer
pub fn round(x: f64) -> f64 {
    x.round()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_power() {
        assert_eq!(power(2.0, 3.0), 8.0);
        assert_eq!(power(5.0, 2.0), 25.0);
    }
    
    #[test]
    fn test_sqrt() {
        assert_eq!(sqrt(9.0), 3.0);
        assert_eq!(sqrt(16.0), 4.0);
    }
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}
