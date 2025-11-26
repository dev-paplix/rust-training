//! # MyApp
//!
//! The main MyApp library combining core and utils with a high-level API.
//!
//! This crate re-exports functionality from `myapp_core` and `myapp_utils` while
//! providing additional high-level features.
//!
//! # Examples
//!
//! ```
//! use myapp::{User, create_user, UserManager};
//!
//! // Create a user
//! let user = create_user("alice", "alice@example.com").unwrap();
//!
//! // Use the manager
//! let mut manager = UserManager::new();
//! manager.add_user(user);
//! assert_eq!(manager.user_count(), 1);
//! ```

// Re-export from core
pub use myapp_core::{User, DataStore, MemoryStore, CoreError};

// Re-export from utils
pub use myapp_utils::{validate_email, format_username, UserStats};

use serde::{Serialize, Deserialize};

/// Creates a validated user (convenience function).
///
/// # Examples
///
/// ```
/// use myapp::create_user;
///
/// let user = create_user("alice", "alice@example.com").unwrap();
/// assert_eq!(user.username(), "alice");
/// ```
///
/// # Errors
///
/// Returns an error if the email is invalid.
pub fn create_user(username: &str, email: &str) -> std::result::Result<User, String> {
    myapp_utils::create_validated_user(username, email)
}

/// High-level user manager.
///
/// # Examples
///
/// ```
/// use myapp::{UserManager, User};
///
/// let mut manager = UserManager::new();
/// let user = User::new("alice", "alice@example.com");
/// manager.add_user(user);
/// assert_eq!(manager.user_count(), 1);
/// ```
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UserManager {
    users: Vec<User>,
}

impl UserManager {
    /// Creates a new user manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp::UserManager;
    ///
    /// let manager = UserManager::new();
    /// assert_eq!(manager.user_count(), 0);
    /// ```
    pub fn new() -> Self {
        Self { users: Vec::new() }
    }

    /// Adds a user to the manager.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp::{UserManager, User};
    ///
    /// let mut manager = UserManager::new();
    /// manager.add_user(User::new("alice", "alice@example.com"));
    /// assert_eq!(manager.user_count(), 1);
    /// ```
    pub fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    /// Returns the number of users.
    pub fn user_count(&self) -> usize {
        self.users.len()
    }

    /// Finds a user by username.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp::{UserManager, User};
    ///
    /// let mut manager = UserManager::new();
    /// manager.add_user(User::new("alice", "alice@example.com"));
    ///
    /// let found = manager.find_by_username("alice");
    /// assert!(found.is_some());
    /// assert_eq!(found.unwrap().username(), "alice");
    /// ```
    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username() == username)
    }

    /// Returns statistics about managed users.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp::{UserManager, User};
    ///
    /// let mut manager = UserManager::new();
    /// manager.add_user(User::new("alice", "alice@example.com"));
    /// manager.add_user(User::new("bob", "bob@example.com"));
    ///
    /// let stats = manager.statistics();
    /// assert_eq!(stats.total_users, 2);
    /// ```
    pub fn statistics(&self) -> UserStats {
        UserStats::new(self.users.len(), self.users.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let user = create_user("alice", "alice@example.com").unwrap();
        assert_eq!(user.username(), "alice");
        assert_eq!(user.email(), "alice@example.com");
    }

    #[test]
    fn test_create_user_invalid() {
        let result = create_user("alice", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_manager() {
        let mut manager = UserManager::new();
        assert_eq!(manager.user_count(), 0);

        manager.add_user(User::new("alice", "alice@example.com"));
        assert_eq!(manager.user_count(), 1);

        manager.add_user(User::new("bob", "bob@example.com"));
        assert_eq!(manager.user_count(), 2);
    }

    #[test]
    fn test_find_by_username() {
        let mut manager = UserManager::new();
        manager.add_user(User::new("alice", "alice@example.com"));
        manager.add_user(User::new("bob", "bob@example.com"));

        let found = manager.find_by_username("alice");
        assert!(found.is_some());
        assert_eq!(found.unwrap().username(), "alice");

        let not_found = manager.find_by_username("charlie");
        assert!(not_found.is_none());
    }

    #[test]
    fn test_statistics() {
        let mut manager = UserManager::new();
        manager.add_user(User::new("alice", "alice@example.com"));
        manager.add_user(User::new("bob", "bob@example.com"));

        let stats = manager.statistics();
        assert_eq!(stats.total_users, 2);
        assert_eq!(stats.active_users, 2);
    }
}
