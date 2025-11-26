// Authentication module - handles user authentication
use crate::user::User;
use std::collections::HashMap;

/// Authentication service
pub struct AuthService {
    users: HashMap<String, User>,
    next_id: u64,
}

impl AuthService {
    /// Creates a new authentication service
    pub fn new() -> Self {
        AuthService {
            users: HashMap::new(),
            next_id: 1,
        }
    }
    
    /// Registers a new user
    pub fn register(&mut self, username: String, email: String, password: String) -> Result<u64, String> {
        // Validate username
        if username.len() < 3 {
            return Err("Username must be at least 3 characters".to_string());
        }
        
        // Check if username exists
        if self.users.contains_key(&username) {
            return Err("Username already exists".to_string());
        }
        
        // Validate email
        if !crate::user::is_valid_email(&email) {
            return Err("Invalid email format".to_string());
        }
        
        // Validate password
        if password.len() < 6 {
            return Err("Password must be at least 6 characters".to_string());
        }
        
        // Hash password (simplified - in production use proper hashing!)
        let password_hash = hash_password(&password);
        
        // Create user
        let user_id = self.next_id;
        let user = User::new(user_id, username.clone(), email, password_hash);
        
        self.users.insert(username, user);
        self.next_id += 1;
        
        Ok(user_id)
    }
    
    /// Authenticates a user
    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        if let Some(user) = self.users.get(username) {
            if !user.is_active() {
                return false;
            }
            let password_hash = hash_password(password);
            user.password_hash() == password_hash
        } else {
            false
        }
    }
    
    /// Gets a user by username
    pub fn get_user(&self, username: &str) -> Option<&User> {
        self.users.get(username)
    }
    
    /// Gets total number of users
    pub fn user_count(&self) -> usize {
        self.users.len()
    }
}

/// Hashes a password (simplified - use proper crypto in production!)
fn hash_password(password: &str) -> String {
    // WARNING: This is NOT secure! Use bcrypt, argon2, etc. in production
    format!("HASH_{}", password.chars().map(|c| (c as u32).to_string()).collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_registration() {
        let mut auth = AuthService::new();
        let result = auth.register(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string()
        );
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_authentication() {
        let mut auth = AuthService::new();
        auth.register(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string()
        ).unwrap();
        
        assert!(auth.authenticate("testuser", "password123"));
        assert!(!auth.authenticate("testuser", "wrongpassword"));
    }
    
    #[test]
    fn test_duplicate_username() {
        let mut auth = AuthService::new();
        auth.register(
            "testuser".to_string(),
            "test@example.com".to_string(),
            "password123".to_string()
        ).unwrap();
        
        let result = auth.register(
            "testuser".to_string(),
            "another@example.com".to_string(),
            "password456".to_string()
        );
        assert!(result.is_err());
    }
}
