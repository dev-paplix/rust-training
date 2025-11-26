//! WebAssembly Demo Module
//!
//! This library demonstrates how to create WebAssembly modules using wasm-bindgen
//! that can be called from JavaScript in web browsers or Node.js.
//!
//! # Features
//!
//! - Basic arithmetic functions
//! - String manipulation
//! - DOM manipulation
//! - JavaScript interop
//! - Console logging
//! - Data structures

use wasm_bindgen::prelude::*;
use web_sys::console;

// ============================================================================
// Basic Functions
// ============================================================================

/// Adds two numbers together.
///
/// # Example (JavaScript)
///
/// ```javascript
/// import { add } from './wasm_demo';
/// const result = add(5, 3); // 8
/// ```
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Divides two numbers, returning an Option.
#[wasm_bindgen]
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Calculates the factorial of a number.
#[wasm_bindgen]
pub fn factorial(n: u32) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        (1..=n as u64).product()
    }
}

/// Checks if a number is prime.
#[wasm_bindgen]
pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    
    let limit = (n as f64).sqrt() as u32;
    for i in (3..=limit).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// ============================================================================
// String Functions
// ============================================================================

/// Greets a person by name.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome from Rust WebAssembly.", name)
}

/// Converts a string to uppercase.
#[wasm_bindgen]
pub fn to_uppercase(text: &str) -> String {
    text.to_uppercase()
}

/// Reverses a string.
#[wasm_bindgen]
pub fn reverse_string(text: &str) -> String {
    text.chars().rev().collect()
}

/// Counts the number of words in a text.
#[wasm_bindgen]
pub fn word_count(text: &str) -> usize {
    text.split_whitespace().count()
}

/// Checks if a string is a palindrome.
#[wasm_bindgen]
pub fn is_palindrome(text: &str) -> bool {
    let cleaned: String = text
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    cleaned == cleaned.chars().rev().collect::<String>()
}

// ============================================================================
// Console Logging
// ============================================================================

/// Logs a message to the browser console.
#[wasm_bindgen]
pub fn log(message: &str) {
    console::log_1(&JsValue::from_str(message));
}

/// Logs a warning to the browser console.
#[wasm_bindgen]
pub fn warn(message: &str) {
    console::warn_1(&JsValue::from_str(message));
}

/// Logs an error to the browser console.
#[wasm_bindgen]
pub fn error(message: &str) {
    console::error_1(&JsValue::from_str(message));
}

// ============================================================================
// DOM Manipulation
// ============================================================================

/// Sets the text content of an element by ID.
///
/// Returns true if successful, false otherwise.
#[wasm_bindgen]
pub fn set_element_text(id: &str, text: &str) -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return false,
    };
    
    let element = match document.get_element_by_id(id) {
        Some(e) => e,
        None => return false,
    };
    
    element.set_text_content(Some(text));
    true
}

/// Gets the text content of an element by ID.
///
/// Returns the text content or an empty string if not found.
#[wasm_bindgen]
pub fn get_element_text(id: &str) -> String {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return String::new(),
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return String::new(),
    };
    
    let element = match document.get_element_by_id(id) {
        Some(e) => e,
        None => return String::new(),
    };
    
    element.text_content().unwrap_or_default()
}

/// Creates a new paragraph element with the given text and appends it to the body.
///
/// Returns true if successful, false otherwise.
#[wasm_bindgen]
pub fn append_paragraph(text: &str) -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return false,
    };
    
    let paragraph = match document.create_element("p") {
        Ok(p) => p,
        Err(_) => return false,
    };
    
    paragraph.set_text_content(Some(text));
    
    let body = match document.body() {
        Some(b) => b,
        None => return false,
    };
    
    match body.append_child(&paragraph) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// ============================================================================
// Data Structures
// ============================================================================

/// A counter that can be incremented and decremented.
#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    /// Creates a new counter starting at 0.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { value: 0 }
    }

    /// Gets the current counter value.
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }

    /// Increments the counter by 1.
    pub fn increment(&mut self) {
        self.value += 1;
    }

    /// Decrements the counter by 1.
    pub fn decrement(&mut self) {
        self.value -= 1;
    }

    /// Increments the counter by a specific amount.
    pub fn increment_by(&mut self, amount: i32) {
        self.value += amount;
    }

    /// Resets the counter to 0.
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

/// A 2D point with x and y coordinates.
#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
impl Point {
    /// Creates a new point.
    #[wasm_bindgen(constructor)]
    pub fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    /// Gets the x coordinate.
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> f64 {
        self.x
    }

    /// Sets the x coordinate.
    #[wasm_bindgen(setter)]
    pub fn set_x(&mut self, x: f64) {
        self.x = x;
    }

    /// Gets the y coordinate.
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> f64 {
        self.y
    }

    /// Sets the y coordinate.
    #[wasm_bindgen(setter)]
    pub fn set_y(&mut self, y: f64) {
        self.y = y;
    }

    /// Calculates the distance to another point.
    pub fn distance_to(&self, other: &Point) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculates the midpoint with another point.
    pub fn midpoint(&self, other: &Point) -> Point {
        Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        }
    }

    /// Moves the point by the given offsets.
    pub fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }

    /// Returns a string representation of the point.
    pub fn to_string(&self) -> String {
        format!("Point({}, {})", self.x, self.y)
    }
}

/// A calculator that maintains a running total.
#[wasm_bindgen]
pub struct Calculator {
    value: f64,
}

#[wasm_bindgen]
impl Calculator {
    /// Creates a new calculator starting at 0.
    #[wasm_bindgen(constructor)]
    pub fn new() -> Calculator {
        Calculator { value: 0.0 }
    }

    /// Gets the current result.
    #[wasm_bindgen(getter)]
    pub fn result(&self) -> f64 {
        self.value
    }

    /// Adds a number to the current value.
    pub fn add(&mut self, n: f64) {
        self.value += n;
    }

    /// Subtracts a number from the current value.
    pub fn subtract(&mut self, n: f64) {
        self.value -= n;
    }

    /// Multiplies the current value by a number.
    pub fn multiply(&mut self, n: f64) {
        self.value *= n;
    }

    /// Divides the current value by a number.
    /// Returns false if dividing by zero.
    pub fn divide(&mut self, n: f64) -> bool {
        if n == 0.0 {
            false
        } else {
            self.value /= n;
            true
        }
    }

    /// Resets the calculator to 0.
    pub fn reset(&mut self) {
        self.value = 0.0;
    }
}

// ============================================================================
// Initialization
// ============================================================================

/// Initializes the WASM module and logs a greeting.
/// This is automatically called when the module is loaded.
#[wasm_bindgen(start)]
pub fn init() {
    console::log_1(&JsValue::from_str(
        "Rust WebAssembly module loaded successfully!"
    ));
}

// ============================================================================
// Tests
// ============================================================================

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
        assert_eq!(multiply(5, 3), 15);
        assert_eq!(multiply(-10, 5), -50);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_is_prime() {
        assert!(!is_prime(0));
        assert!(!is_prime(1));
        assert!(is_prime(2));
        assert!(is_prime(3));
        assert!(!is_prime(4));
        assert!(is_prime(17));
        assert!(!is_prime(18));
    }

    #[test]
    fn test_greet() {
        assert!(greet("Alice").contains("Alice"));
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("Rust"), "tsuR");
    }

    #[test]
    fn test_word_count() {
        assert_eq!(word_count("hello world"), 2);
        assert_eq!(word_count("one two three four"), 4);
    }

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_counter() {
        let mut counter = Counter::new();
        assert_eq!(counter.value(), 0);
        
        counter.increment();
        assert_eq!(counter.value(), 1);
        
        counter.increment_by(5);
        assert_eq!(counter.value(), 6);
        
        counter.decrement();
        assert_eq!(counter.value(), 5);
        
        counter.reset();
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_point() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(3.0, 4.0);
        
        let distance = p1.distance_to(&p2);
        assert!((distance - 5.0).abs() < 0.001);
        
        let mid = p1.midpoint(&p2);
        assert_eq!(mid.x(), 1.5);
        assert_eq!(mid.y(), 2.0);
        
        let mut p3 = Point::new(10.0, 20.0);
        p3.translate(5.0, -3.0);
        assert_eq!(p3.x(), 15.0);
        assert_eq!(p3.y(), 17.0);
    }

    #[test]
    fn test_calculator() {
        let mut calc = Calculator::new();
        assert_eq!(calc.result(), 0.0);
        
        calc.add(10.0);
        calc.multiply(2.0);
        assert_eq!(calc.result(), 20.0);
        
        calc.subtract(5.0);
        assert_eq!(calc.result(), 15.0);
        
        assert!(calc.divide(3.0));
        assert_eq!(calc.result(), 5.0);
        
        assert!(!calc.divide(0.0));
    }
}
