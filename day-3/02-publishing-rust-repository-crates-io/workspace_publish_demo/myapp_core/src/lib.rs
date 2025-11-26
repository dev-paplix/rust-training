//! # MyApp Core
//!
//! Core functionality for MyApp including data structures and traits.
//!
//! This crate provides the foundational types and traits that other MyApp crates depend on.
//! It has no dependencies on other MyApp crates, making it the base of the dependency tree.
//!
//! # Examples
//!
//! ```
//! use myapp_core::{User, DataStore};
//!
//! let user = User::new("alice", "alice@example.com");
//! assert_eq!(user.username(), "alice");
//! ```

use serde::{Serialize, Deserialize};
use thiserror::Error;

/// Error type for core operations.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Invalid input error
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    /// Not found error
    #[error("Item not found: {0}")]
    NotFound(String),
}

/// Result type for core operations.
pub type Result<T> = std::result::Result<T, CoreError>;

/// A user in the system.
///
/// # Examples
///
/// ```
/// use myapp_core::User;
///
/// let user = User::new("bob", "bob@example.com");
/// assert_eq!(user.username(), "bob");
/// assert_eq!(user.email(), "bob@example.com");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    username: String,
    email: String,
}

impl User {
    /// Creates a new user.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp_core::User;
    ///
    /// let user = User::new("alice", "alice@example.com");
    /// ```
    pub fn new(username: impl Into<String>, email: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            email: email.into(),
        }
    }

    /// Returns the username.
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Returns the email.
    pub fn email(&self) -> &str {
        &self.email
    }

    /// Updates the email address.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp_core::User;
    ///
    /// let mut user = User::new("alice", "alice@example.com");
    /// user.set_email("newemail@example.com");
    /// assert_eq!(user.email(), "newemail@example.com");
    /// ```
    pub fn set_email(&mut self, email: impl Into<String>) {
        self.email = email.into();
    }
}

/// Trait for data storage operations.
pub trait DataStore {
    /// The type of data stored.
    type Item;
    
    /// Adds an item to the store.
    fn add(&mut self, item: Self::Item);
    
    /// Gets an item by index.
    fn get(&self, index: usize) -> Option<&Self::Item>;
    
    /// Returns the number of items.
    fn count(&self) -> usize;
}

/// A simple in-memory data store.
///
/// # Examples
///
/// ```
/// use myapp_core::{MemoryStore, DataStore};
///
/// let mut store = MemoryStore::new();
/// store.add("item1");
/// store.add("item2");
/// assert_eq!(store.count(), 2);
/// ```
#[derive(Debug, Default)]
pub struct MemoryStore<T> {
    items: Vec<T>,
}

impl<T> MemoryStore<T> {
    /// Creates a new empty memory store.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T> DataStore for MemoryStore<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn count(&self) -> usize {
        self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("testuser", "test@example.com");
        assert_eq!(user.username(), "testuser");
        assert_eq!(user.email(), "test@example.com");
    }

    #[test]
    fn test_user_update_email() {
        let mut user = User::new("testuser", "test@example.com");
        user.set_email("new@example.com");
        assert_eq!(user.email(), "new@example.com");
    }

    #[test]
    fn test_memory_store() {
        let mut store = MemoryStore::new();
        store.add("item1");
        store.add("item2");
        store.add("item3");

        assert_eq!(store.count(), 3);
        assert_eq!(store.get(0), Some(&"item1"));
        assert_eq!(store.get(1), Some(&"item2"));
        assert_eq!(store.get(2), Some(&"item3"));
        assert_eq!(store.get(3), None);
    }
}
