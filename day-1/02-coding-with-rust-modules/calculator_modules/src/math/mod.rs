// Math module - organizes mathematical operations
pub mod basic;
pub mod advanced;
pub mod statistics;

// Re-export commonly used functions
pub use basic::{add, subtract, multiply, divide};
pub use advanced::{power, sqrt};
