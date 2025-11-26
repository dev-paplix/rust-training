//! C FFI Demonstration Library
//!
//! This library demonstrates how to create Rust functions that can be called from C code.
//! It covers:
//! - Basic numeric functions
//! - String handling with CStr/CString
//! - Array processing
//! - Struct handling with #[repr(C)]
//! - Error handling patterns
//!
//! # Building
//!
//! ```bash
//! cargo build --release
//! ```
//!
//! # Testing from C
//!
//! See the `examples/test.c` file for usage examples.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int};
use std::ptr;
use std::slice;

/// Error codes returned by this library
#[repr(C)]
pub enum ErrorCode {
    Success = 0,
    NullPointer = -1,
    InvalidUtf8 = -2,
    BufferTooSmall = -3,
}

/// A point in 2D space, compatible with C
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: c_double,
    pub y: c_double,
}

/// A result structure for operations that can fail
#[repr(C)]
pub struct OperationResult {
    pub success: bool,
    pub value: c_double,
    pub error_code: c_int,
}

// ============================================================================
// Basic Numeric Functions
// ============================================================================

/// Adds two integers
///
/// # Safety
///
/// This function is safe to call from C with any integer values.
///
/// # Example (from C)
///
/// ```c
/// int result = add(5, 3); // result = 8
/// ```
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a.saturating_add(b)
}

/// Multiplies two integers
///
/// # Safety
///
/// This function is safe to call from C with any integer values.
#[no_mangle]
pub extern "C" fn multiply(a: c_int, b: c_int) -> c_int {
    a.saturating_mul(b)
}

/// Divides two doubles safely
///
/// Returns an OperationResult indicating success or failure.
///
/// # Safety
///
/// This function is safe to call from C.
#[no_mangle]
pub extern "C" fn divide(a: c_double, b: c_double) -> OperationResult {
    if b == 0.0 {
        OperationResult {
            success: false,
            value: 0.0,
            error_code: -1,
        }
    } else {
        OperationResult {
            success: true,
            value: a / b,
            error_code: 0,
        }
    }
}

// ============================================================================
// String Functions
// ============================================================================

/// Greets a person by name
///
/// # Safety
///
/// The `name` pointer must be:
/// - A valid, non-null pointer to a null-terminated C string
/// - Valid for the duration of this function call
/// - Properly aligned
///
/// The returned string must be freed using `free_rust_string`.
///
/// # Example (from C)
///
/// ```c
/// char* greeting = greet("Alice");
/// printf("%s\n", greeting);
/// free_rust_string(greeting);
/// ```
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
///
/// # Safety
///
/// Same safety requirements as `greet`.
/// The returned string must be freed using `free_rust_string`.
#[no_mangle]
pub unsafe extern "C" fn to_uppercase(input: *const c_char) -> *mut c_char {
    if input.is_null() {
        return ptr::null_mut();
    }

    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return ptr::null_mut(),
    };

    let uppercase = c_str.to_uppercase();
    
    match CString::new(uppercase) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Gets the length of a UTF-8 string
///
/// # Safety
///
/// The `input` pointer must point to a valid null-terminated C string.
/// Returns -1 if the pointer is null or the string is invalid UTF-8.
#[no_mangle]
pub unsafe extern "C" fn string_length(input: *const c_char) -> c_int {
    if input.is_null() {
        return -1;
    }

    match CStr::from_ptr(input).to_str() {
        Ok(s) => s.chars().count() as c_int,
        Err(_) => -1,
    }
}

/// Frees a string allocated by Rust
///
/// # Safety
///
/// The `s` pointer must be:
/// - Either null (in which case this is a no-op)
/// - Or a pointer previously returned by a Rust function that returns *mut c_char
/// - Must not be used after calling this function
#[no_mangle]
pub unsafe extern "C" fn free_rust_string(s: *mut c_char) {
    if !s.is_null() {
        drop(CString::from_raw(s));
    }
}

// ============================================================================
// Array Functions
// ============================================================================

/// Sums an array of integers
///
/// # Safety
///
/// - `arr` must be a valid pointer to an array of at least `len` elements
/// - `len` must accurately represent the array length
/// - The array must remain valid for the duration of this call
///
/// # Example (from C)
///
/// ```c
/// int numbers[] = {1, 2, 3, 4, 5};
/// int sum = sum_array(numbers, 5); // sum = 15
/// ```
#[no_mangle]
pub unsafe extern "C" fn sum_array(arr: *const c_int, len: usize) -> c_int {
    if arr.is_null() || len == 0 {
        return 0;
    }

    let slice = slice::from_raw_parts(arr, len);
    slice.iter().sum()
}

/// Finds the maximum value in an array
///
/// # Safety
///
/// Same safety requirements as `sum_array`.
/// Returns i32::MIN if the array is null or empty.
#[no_mangle]
pub unsafe extern "C" fn max_array(arr: *const c_int, len: usize) -> c_int {
    if arr.is_null() || len == 0 {
        return c_int::MIN;
    }

    let slice = slice::from_raw_parts(arr, len);
    *slice.iter().max().unwrap_or(&c_int::MIN)
}

/// Sorts an array in place
///
/// # Safety
///
/// - `arr` must be a valid pointer to an array of at least `len` elements
/// - The array must be mutable
/// - The array must remain valid for the duration of this call
#[no_mangle]
pub unsafe extern "C" fn sort_array(arr: *mut c_int, len: usize) {
    if arr.is_null() || len == 0 {
        return;
    }

    let slice = slice::from_raw_parts_mut(arr, len);
    slice.sort_unstable();
}

// ============================================================================
// Struct Functions
// ============================================================================

/// Creates a new Point
///
/// # Safety
///
/// This function is safe to call from C.
#[no_mangle]
pub extern "C" fn point_new(x: c_double, y: c_double) -> Point {
    Point { x, y }
}

/// Calculates the distance between two points
///
/// # Safety
///
/// This function is safe to call from C.
#[no_mangle]
pub extern "C" fn point_distance(p1: Point, p2: Point) -> c_double {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

/// Calculates the midpoint between two points
///
/// # Safety
///
/// This function is safe to call from C.
#[no_mangle]
pub extern "C" fn point_midpoint(p1: Point, p2: Point) -> Point {
    Point {
        x: (p1.x + p2.x) / 2.0,
        y: (p1.y + p2.y) / 2.0,
    }
}

/// Moves a point by the given offsets
///
/// # Safety
///
/// - `point` must be a valid, non-null pointer to a Point
/// - The Point must remain valid for the duration of this call
#[no_mangle]
pub unsafe extern "C" fn point_translate(point: *mut Point, dx: c_double, dy: c_double) {
    if point.is_null() {
        return;
    }

    (*point).x += dx;
    (*point).y += dy;
}

// ============================================================================
// Advanced Error Handling
// ============================================================================

/// Parses an integer from a string with error handling
///
/// # Safety
///
/// - `input` must be a valid null-terminated C string
/// - `output` must be a valid pointer to store the result
///
/// Returns ErrorCode indicating success or failure.
#[no_mangle]
pub unsafe extern "C" fn parse_int(input: *const c_char, output: *mut c_int) -> ErrorCode {
    if input.is_null() {
        return ErrorCode::NullPointer;
    }

    if output.is_null() {
        return ErrorCode::NullPointer;
    }

    let c_str = match CStr::from_ptr(input).to_str() {
        Ok(s) => s,
        Err(_) => return ErrorCode::InvalidUtf8,
    };

    match c_str.parse::<c_int>() {
        Ok(value) => {
            *output = value;
            ErrorCode::Success
        }
        Err(_) => ErrorCode::InvalidUtf8,
    }
}

/// Copies a string to a provided buffer
///
/// # Safety
///
/// - `src` must be a valid null-terminated C string
/// - `dest` must be a valid buffer of at least `dest_len` bytes
///
/// Returns ErrorCode indicating success or failure.
#[no_mangle]
pub unsafe extern "C" fn copy_string(
    src: *const c_char,
    dest: *mut c_char,
    dest_len: usize,
) -> ErrorCode {
    if src.is_null() || dest.is_null() {
        return ErrorCode::NullPointer;
    }

    if dest_len == 0 {
        return ErrorCode::BufferTooSmall;
    }

    let c_str = match CStr::from_ptr(src).to_str() {
        Ok(s) => s,
        Err(_) => return ErrorCode::InvalidUtf8,
    };

    let bytes = c_str.as_bytes();
    
    if bytes.len() + 1 > dest_len {
        return ErrorCode::BufferTooSmall;
    }

    ptr::copy_nonoverlapping(bytes.as_ptr(), dest as *mut u8, bytes.len());
    *dest.add(bytes.len()) = 0; // null terminator

    ErrorCode::Success
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(5, 3), 8);
        assert_eq!(add(-10, 5), -5);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(5, 3), 15);
        assert_eq!(multiply(-10, 5), -50);
        assert_eq!(multiply(0, 100), 0);
    }

    #[test]
    fn test_divide() {
        let result = divide(10.0, 2.0);
        assert!(result.success);
        assert_eq!(result.value, 5.0);

        let result = divide(10.0, 0.0);
        assert!(!result.success);
    }

    #[test]
    fn test_sum_array() {
        let numbers = vec![1, 2, 3, 4, 5];
        unsafe {
            assert_eq!(sum_array(numbers.as_ptr(), numbers.len()), 15);
        }
    }

    #[test]
    fn test_max_array() {
        let numbers = vec![1, 5, 3, 9, 2];
        unsafe {
            assert_eq!(max_array(numbers.as_ptr(), numbers.len()), 9);
        }
    }

    #[test]
    fn test_point_operations() {
        let p1 = point_new(0.0, 0.0);
        let p2 = point_new(3.0, 4.0);
        
        let distance = point_distance(p1, p2);
        assert!((distance - 5.0).abs() < 0.001);

        let mid = point_midpoint(p1, p2);
        assert_eq!(mid.x, 1.5);
        assert_eq!(mid.y, 2.0);
    }

    #[test]
    fn test_string_length() {
        let test_str = CString::new("Hello").unwrap();
        unsafe {
            assert_eq!(string_length(test_str.as_ptr()), 5);
        }
    }
}
