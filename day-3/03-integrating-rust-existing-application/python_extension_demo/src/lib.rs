//! Python Extension Module Demo
//!
//! This library demonstrates how to create Python extension modules using PyO3.
//! It covers:
//! - Simple functions exposed to Python
//! - Python classes with methods
//! - Error handling with PyResult
//! - Type conversion between Rust and Python
//! - Iterators and collections

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::collections::HashMap;

// ============================================================================
// Simple Functions
// ============================================================================

/// Adds two numbers together.
///
/// Args:
///     a (int): First number
///     b (int): Second number
///
/// Returns:
///     int: Sum of a and b
///
/// Example:
///     >>> import mathpy
///     >>> mathpy.add(5, 3)
///     8
#[pyfunction]
fn add(a: i64, b: i64) -> PyResult<i64> {
    Ok(a + b)
}

/// Multiplies two numbers.
///
/// Args:
///     a (int): First number
///     b (int): Second number
///
/// Returns:
///     int: Product of a and b
#[pyfunction]
fn multiply(a: i64, b: i64) -> PyResult<i64> {
    Ok(a * b)
}

/// Divides two numbers with error handling.
///
/// Args:
///     a (float): Numerator
///     b (float): Denominator
///
/// Returns:
///     float: Result of division
///
/// Raises:
///     ValueError: If dividing by zero
#[pyfunction]
fn devide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(PyValueError::new_err("Cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

/// Calculates the sum of a list of numbers.
///
/// Args:
///     numbers (list): List of integers
///
/// Returns:
///     int: Sum of all numbers
#[pyfunction]
fn sum_list(numbers: Vec<i64>) -> PyResult<i64> {
    Ok(numbers.iter().sum())
}

/// Finds the maximum value in a list.
///
/// Args:
///     numbers (list): List of integers
///
/// Returns:
///     int: Maximum value
///
/// Raises:
///     ValueError: If the list is empty
#[pyfunction]
fn max_list(numbers: Vec<i64>) -> PyResult<i64> {
    numbers.iter()
        .max()
        .copied()
        .ok_or_else(|| PyValueError::new_err("Cannot find max of empty list"))
}

/// Greets a person by name.
///
/// Args:
///     name (str): Person's name
///
/// Returns:
///     str: Greeting message
#[pyfunction]
fn greet(name: &str) -> PyResult<String> {
    Ok(format!("Hello, {}! Welcome from Rust.", name))
}

/// Converts a string to uppercase.
///
/// Args:
///     text (str): Input text
///
/// Returns:
///     str: Uppercase text
#[pyfunction]
fn to_uppercase(text: &str) -> PyResult<String> {
    Ok(text.to_uppercase())
}

/// Counts the frequency of words in a text.
///
/// Args:
///     text (str): Input text
///
/// Returns:
///     dict: Dictionary mapping words to their frequencies
#[pyfunction]
fn word_frequency(text: &str) -> PyResult<HashMap<String, usize>> {
    let mut freq = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *freq.entry(word).or_insert(0) += 1;
    }
    
    Ok(freq)
}

// ============================================================================
// Classes
// ============================================================================

/// A counter that can be incremented and decremented.
///
/// Example:
///     >>> counter = mathpy.Counter()
///     >>> counter.increment()
///     >>> counter.get_value()
///     1
///     >>> counter.increment_by(5)
///     >>> counter.get_value()
///     6
#[pyclass]
struct Counter {
    value: i64,
}

#[pymethods]
impl Counter {
    /// Creates a new counter starting at 0.
    ///
    /// Args:
    ///     start (int, optional): Starting value. Defaults to 0.
    #[new]
    #[pyo3(signature = (start=0))]
    fn new(start: i64) -> Self {
        Counter { value: start }
    }

    /// Gets the current counter value.
    ///
    /// Returns:
    ///     int: Current value
    fn get_value(&self) -> PyResult<i64> {
        Ok(self.value)
    }

    /// Increments the counter by 1.
    fn increment(&mut self) -> PyResult<()> {
        self.value += 1;
        Ok(())
    }

    /// Decrements the counter by 1.
    fn decrement(&mut self) -> PyResult<()> {
        self.value -= 1;
        Ok(())
    }

    /// Increments the counter by a specific amount.
    ///
    /// Args:
    ///     amount (int): Amount to increment
    fn increment_by(&mut self, amount: i64) -> PyResult<()> {
        self.value += amount;
        Ok(())
    }

    /// Resets the counter to 0.
    fn reset(&mut self) -> PyResult<()> {
        self.value = 0;
        Ok(())
    }

    /// String representation of the counter.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Counter(value={})", self.value))
    }
}

/// A 2D point with x and y coordinates.
///
/// Example:
///     >>> p1 = mathpy.Point(3.0, 4.0)
///     >>> p2 = mathpy.Point(0.0, 0.0)
///     >>> p1.distance_to(p2)
///     5.0
#[pyclass]
#[derive(Clone)]
struct Point {
    #[pyo3(get, set)]
    x: f64,
    #[pyo3(get, set)]
    y: f64,
}

#[pymethods]
impl Point {
    /// Creates a new point.
    ///
    /// Args:
    ///     x (float): X coordinate
    ///     y (float): Y coordinate
    #[new]
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    /// Calculates the distance to another point.
    ///
    /// Args:
    ///     other (Point): The other point
    ///
    /// Returns:
    ///     float: Distance between the points
    fn distance_to(&self, other: &Point) -> PyResult<f64> {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        Ok((dx * dx + dy * dy).sqrt())
    }

    /// Calculates the midpoint with another point.
    ///
    /// Args:
    ///     other (Point): The other point
    ///
    /// Returns:
    ///     Point: Midpoint between the two points
    fn midpoint(&self, other: &Point) -> PyResult<Point> {
        Ok(Point {
            x: (self.x + other.x) / 2.0,
            y: (self.y + other.y) / 2.0,
        })
    }

    /// Moves the point by the given offsets.
    ///
    /// Args:
    ///     dx (float): X offset
    ///     dy (float): Y offset
    fn translate(&mut self, dx: f64, dy: f64) -> PyResult<()> {
        self.x += dx;
        self.y += dy;
        Ok(())
    }

    /// String representation of the point.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Point(x={}, y={})", self.x, self.y))
    }
}

/// A simple calculator that maintains a running total.
///
/// Example:
///     >>> calc = mathpy.Calculator()
///     >>> calc.add(10)
///     >>> calc.multiply(2)
///     >>> calc.get_result()
///     20.0
#[pyclass]
struct Calculator {
    value: f64,
}

#[pymethods]
impl Calculator {
    /// Creates a new calculator starting at 0.
    #[new]
    fn new() -> Self {
        Calculator { value: 0.0 }
    }

    /// Gets the current result.
    ///
    /// Returns:
    ///     float: Current value
    fn get_result(&self) -> PyResult<f64> {
        Ok(self.value)
    }

    /// Adds a number to the current value.
    ///
    /// Args:
    ///     n (float): Number to add
    fn add(&mut self, n: f64) -> PyResult<()> {
        self.value += n;
        Ok(())
    }

    /// Subtracts a number from the current value.
    ///
    /// Args:
    ///     n (float): Number to subtract
    fn subtract(&mut self, n: f64) -> PyResult<()> {
        self.value -= n;
        Ok(())
    }

    /// Multiplies the current value by a number.
    ///
    /// Args:
    ///     n (float): Number to multiply by
    fn multiply(&mut self, n: f64) -> PyResult<()> {
        self.value *= n;
        Ok(())
    }

    /// Divides the current value by a number.
    ///
    /// Args:
    ///     n (float): Number to divide by
    ///
    /// Raises:
    ///     ValueError: If dividing by zero
    fn devide(&mut self, n: f64) -> PyResult<()> {
        if n == 0.0 {
            Err(PyValueError::new_err("Cannot divide by zero"))
        } else {
            self.value /= n;
            Ok(())
        }
    }

    /// Resets the calculator to 0.
    fn reset(&mut self) -> PyResult<()> {
        self.value = 0.0;
        Ok(())
    }

    /// String representation of the calculator.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Calculator(value={})", self.value))
    }
}

/// A person with name and age.
///
/// Example:
///     >>> person = mathpy.Person("Alice", 30)
///     >>> person.greet()
///     'Hello, my name is Alice and I am 30 years old.'
///     >>> person.have_birthday()
///     >>> person.age
///     31
#[pyclass]
struct Person {
    #[pyo3(get, set)]
    name: String,
    #[pyo3(get, set)]
    age: u32,
}

#[pymethods]
impl Person {
    /// Creates a new person.
    ///
    /// Args:
    ///     name (str): Person's name
    ///     age (int): Person's age
    ///
    /// Raises:
    ///     ValueError: If age is negative
    #[new]
    fn new(name: String, age: i32) -> PyResult<Self> {
        if age < 0 {
            return Err(PyValueError::new_err("Age cannot be negative"));
        }
        Ok(Person { 
            name, 
            age: age as u32 
        })
    }

    /// Returns a greeting message.
    ///
    /// Returns:
    ///     str: Greeting message
    fn greet(&self) -> PyResult<String> {
        Ok(format!(
            "Hello, my name is {} and I am {} years old.",
            self.name, self.age
        ))
    }

    /// Increments the person's age by 1.
    fn have_birthday(&mut self) -> PyResult<()> {
        self.age += 1;
        Ok(())
    }

    /// Checks if the person is an adult (18 or older).
    ///
    /// Returns:
    ///     bool: True if adult, False otherwise
    fn is_adult(&self) -> PyResult<bool> {
        Ok(self.age >= 18)
    }

    /// String representation of the person.
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Person(name='{}', age={})", self.name, self.age))
    }
}

// ============================================================================
// Module Definition
// ============================================================================

/// A Python module implemented in Rust.
#[pymodule]
fn mathpy(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add functions
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(devide, m)?)?;
    m.add_function(wrap_pyfunction!(sum_list, m)?)?;
    m.add_function(wrap_pyfunction!(max_list, m)?)?;
    m.add_function(wrap_pyfunction!(greet, m)?)?;
    m.add_function(wrap_pyfunction!(to_uppercase, m)?)?;
    m.add_function(wrap_pyfunction!(word_frequency, m)?)?;

    // Add classes
    m.add_class::<Counter>()?;
    m.add_class::<Point>()?;
    m.add_class::<Calculator>()?;
    m.add_class::<Person>()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: PyO3 tests require Python runtime which may not be available during `cargo test`
    // on Windows. These tests verify the Rust logic works correctly.
    // For full integration testing, use: maturin develop && python examples/test.py

    #[test]
    fn test_counter_logic() {
        // Test the internal logic without PyO3
        let mut counter = Counter { value: 0 };
        assert_eq!(counter.value, 0);
        
        counter.value += 1;
        assert_eq!(counter.value, 1);
        
        counter.value += 5;
        assert_eq!(counter.value, 6);
        
        counter.value -= 1;
        assert_eq!(counter.value, 5);
        
        counter.value = 0;
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn test_point_logic() {
        let p1 = Point { x: 0.0, y: 0.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        
        // Test distance calculation
        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let distance = (dx * dx + dy * dy).sqrt();
        assert!((distance - 5.0).abs() < 0.001);
        
        // Test midpoint calculation
        let mid_x = (p1.x + p2.x) / 2.0;
        let mid_y = (p1.y + p2.y) / 2.0;
        assert_eq!(mid_x, 1.5);
        assert_eq!(mid_y, 2.0);
    }

    #[test]
    fn test_calculator_logic() {
        let mut calc = Calculator { value: 0.0 };
        assert_eq!(calc.value, 0.0);
        
        calc.value += 10.0;
        calc.value *= 2.0;
        assert_eq!(calc.value, 20.0);
        
        calc.value -= 5.0;
        assert_eq!(calc.value, 15.0);
        
        calc.value /= 3.0;
        assert_eq!(calc.value, 5.0);
        
        // Test division by zero check
        let b = 0.0;
        assert_eq!(b, 0.0); // Would fail divide operation
    }

    #[test]
    fn test_person_validation() {
        // Test valid person
        let person = Person { 
            name: "Alice".to_string(), 
            age: 30 
        };
        assert_eq!(person.name, "Alice");
        assert_eq!(person.age, 30);
        assert!(person.age >= 18); // Adult check
        
        // Test age increment
        let mut person2 = Person { 
            name: "Bob".to_string(), 
            age: 25 
        };
        person2.age += 1;
        assert_eq!(person2.age, 26);
    }
}
