//! Node.js Integration Demo
//!
//! This library demonstrates a simple approach to integrating Rust with Node.js using:
//! - Basic C-compatible FFI functions
//! - Node.js ffi-napi for calling Rust from JavaScript
//!
//! This is simpler than full N-API bindings but still demonstrates
//! the power of calling high-performance Rust code from JavaScript.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::ptr;
use serde::{Deserialize, Serialize};

// ============================================================================
// Basic Numeric Functions
// ============================================================================

/// Adds two numbers
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Multiplies two numbers
#[no_mangle]
pub extern "C" fn multiply(a: c_int, b: c_int) -> c_int {
    a * b
}

/// Divides two numbers (returns 0 if division by zero)
#[no_mangle]
pub extern "C" fn divide(a: c_double, b: c_double) -> c_double {
    if b == 0.0 {
        0.0
    } else {
        a / b
    }
}

/// Checks if division is valid (non-zero denominator)
#[no_mangle]
pub extern "C" fn can_divide(b: c_double) -> bool {
    b != 0.0
}

// ============================================================================
// String Functions
// ============================================================================

/// Greets a person by name
#[no_mangle]
pub unsafe extern "C" fn greet(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return ptr::null_mut();
    }
    
    let c_str = match CStr::from_ptr(name).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    let greeting = format!("Hello, {}! Welcome from Rust.", c_str);
    match CString::new(greeting) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Converts a string to uppercase
#[no_mangle]
pub unsafe extern "C" fn to_uppercase(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return ptr::null_mut();
    }
    
    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    match CString::new(c_str.to_uppercase()) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Reverses a string
#[no_mangle]
pub unsafe extern "C" fn reverse_string(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return ptr::null_mut();
    }
    
    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };
    
    let reversed: String = c_str.chars().rev().collect();
    match CString::new(reversed) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Frees a string allocated by Rust
#[no_mangle]
pub unsafe extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// ============================================================================
// Computational Functions
// ============================================================================

/// Calculates factorial
#[no_mangle]
pub extern "C" fn factorial(n: c_int) -> i64 {
    if n <= 1 {
        1
    } else {
        (1..=n as i64).product()
    }
}

/// Checks if a number is prime
#[no_mangle]
pub extern "C" fn is_prime(n: c_int) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as i32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// ============================================================================
// JSON-based Data Exchange
// ============================================================================

#[derive(Serialize, Deserialize)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Calculates distance between two points (passed as JSON)
#[no_mangle]
pub unsafe extern "C" fn point_distance(json: *const c_char) -> c_double {
    if json.is_null() {
        return 0.0;
    }
    
    let c_str = match CStr::from_ptr(json).to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };
    
    let points: Result<(Point, Point), _> = serde_json::from_str(c_str);
    match points {
        Ok((p1, p2)) => p1.distance_to(&p2),
        Err(_) => 0.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(-10, 5), -5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(6, 7), 42);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(17));
        assert!(!is_prime(18));
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001);
    }
}
