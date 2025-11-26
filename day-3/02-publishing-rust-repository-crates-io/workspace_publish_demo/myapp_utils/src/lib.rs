//! # MyApp Utils
//!
//! Utility functions for MyApp providing helpers and common operations.
//!
//! This crate depends on `myapp_core` and provides higher-level utility functions
//! that work with core types.
//!
//! # Examples
//!
//! ```
//! use myapp_utils::{validate_email, format_username};
//!
//! assert!(validate_email("user@example.com"));
//! assert_eq!(format_username("alice"), "@alice");
//! ```

use myapp_core::User;
use serde::{Serialize, Deserialize};

/// Validates an email address (simple validation).
///
/// # Examples
///
/// ```
/// use myapp_utils::validate_email;
///
/// assert!(validate_email("user@example.com"));
/// assert!(!validate_email("invalid"));
/// ```
pub fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

/// Formats a username with @ prefix.
///
/// # Examples
///
/// ```
/// use myapp_utils::format_username;
///
/// assert_eq!(format_username("alice"), "@alice");
/// assert_eq!(format_username("bob"), "@bob");
/// ```
pub fn format_username(username: &str) -> String {
    format!("@{}", username)
}

/// Creates a user with validation.
///
/// # Examples
///
/// ```
/// use myapp_utils::create_validated_user;
///
/// let user = create_validated_user("alice", "alice@example.com").unwrap();
/// assert_eq!(user.username(), "alice");
/// ```
///
/// # Errors
///
/// Returns an error if the email is invalid.
pub fn create_validated_user(username: &str, email: &str) -> Result<User, String> {
    if !validate_email(email) {
        return Err(format!("Invalid email: {}", email));
    }
    Ok(User::new(username, email))
}

/// User statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub total_users: usize,
    pub active_users: usize,
}

impl UserStats {
    /// Creates new user statistics.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp_utils::UserStats;
    ///
    /// let stats = UserStats::new(100, 75);
    /// assert_eq!(stats.total_users, 100);
    /// assert_eq!(stats.active_users, 75);
    /// ```
    pub fn new(total_users: usize, active_users: usize) -> Self {
        Self {
            total_users,
            active_users,
        }
    }

    /// Calculates the activity rate as a percentage.
    ///
    /// # Examples
    ///
    /// ```
    /// use myapp_utils::UserStats;
    ///
    /// let stats = UserStats::new(100, 75);
    /// assert_eq!(stats.activity_rate(), 75.0);
    /// ```
    pub fn activity_rate(&self) -> f64 {
        if self.total_users == 0 {
            0.0
        } else {
            (self.active_users as f64 / self.total_users as f64) * 100.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_email() {
        assert!(validate_email("user@example.com"));
        assert!(validate_email("test.user@domain.co.uk"));
        assert!(!validate_email("invalid"));
        assert!(!validate_email("no-at-sign.com"));
    }

    #[test]
    fn test_format_username() {
        assert_eq!(format_username("alice"), "@alice");
        assert_eq!(format_username("bob123"), "@bob123");
    }

    #[test]
    fn test_create_validated_user() {
        let result = create_validated_user("alice", "alice@example.com");
        assert!(result.is_ok());
        
        let user = result.unwrap();
        assert_eq!(user.username(), "alice");
        assert_eq!(user.email(), "alice@example.com");
    }

    #[test]
    fn test_create_validated_user_invalid_email() {
        let result = create_validated_user("alice", "invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_stats() {
        let stats = UserStats::new(100, 75);
        assert_eq!(stats.total_users, 100);
        assert_eq!(stats.active_users, 75);
        assert_eq!(stats.activity_rate(), 75.0);
    }

    #[test]
    fn test_user_stats_zero_total() {
        let stats = UserStats::new(0, 0);
        assert_eq!(stats.activity_rate(), 0.0);
    }
}
