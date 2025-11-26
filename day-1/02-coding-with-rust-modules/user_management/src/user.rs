// User module - demonstrates privacy and encapsulation

use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a user in the system
pub struct User {
    id: u64,
    username: String,
    email: String,
    password_hash: String,  // Private - never exposed
    created_at: u64,
    is_active: bool,
}

impl User {
    /// Creates a new user (crate-private)
    pub(crate) fn new(id: u64, username: String, email: String, password_hash: String) -> Self {
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        User {
            id,
            username,
            email,
            password_hash,
            created_at,
            is_active: true,
        }
    }
    
    /// Gets user ID (public)
    pub fn id(&self) -> u64 {
        self.id
    }
    
    /// Gets username (public)
    pub fn username(&self) -> &str {
        &self.username
    }
    
    /// Gets email (public)
    pub fn email(&self) -> &str {
        &self.email
    }
    
    /// Checks if user is active (public)
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    
    /// Gets password hash (crate-private for auth module)
    pub(crate) fn password_hash(&self) -> &str {
        &self.password_hash
    }
    
    /// Deactivates user account
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
    
    /// Activates user account
    pub fn activate(&mut self) {
        self.is_active = true;
    }
    
    /// Displays public user information
    pub fn display_public_info(&self) {
        println!("\n👤 User Information:");
        println!("   ID: {}", self.id);
        println!("   Username: {}", self.username);
        println!("   Email: {}", self.email);
        println!("   Status: {}", if self.is_active { "Active" } else { "Inactive" });
        println!("   Created: {} seconds since epoch", self.created_at);
    }
}

// Private helper function
fn validate_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}

pub fn is_valid_email(email: &str) -> bool {
    validate_email(email)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("user@example.com"));
        assert!(!validate_email("invalid-email"));
        assert!(!validate_email("@example.com"));
    }
}
