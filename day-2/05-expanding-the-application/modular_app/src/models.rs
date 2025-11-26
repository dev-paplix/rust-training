use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
    created_at: String,
}

impl User {
    pub fn new(name: String, email: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        Ok(Self {
            id: 0, // Will be set by handler
            name,
            email,
            created_at: current_timestamp(),
        })
    }
    
    pub fn with_id(mut self, id: u32) -> Self {
        self.id = id;
        self
    }
    
    // Getters
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn email(&self) -> &str {
        &self.email
    }
    
    pub fn created_at(&self) -> &str {
        &self.created_at
    }
    
    // Setters
    pub fn set_name(&mut self, name: String) -> Result<(), String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        self.name = name;
        Ok(())
    }
    
    pub fn set_email(&mut self, email: String) -> Result<(), String> {
        if !email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        self.email = email;
        Ok(())
    }
    
    pub fn matches_query(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.name.to_lowercase().contains(&query_lower) ||
        self.email.to_lowercase().contains(&query_lower)
    }
    
    pub fn domain(&self) -> Option<&str> {
        self.email.split('@').nth(1)
    }
}

fn current_timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap();
    format!("{}", duration.as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new("Alice".to_string(), "alice@example.com".to_string());
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.name(), "Alice");
        assert_eq!(user.email(), "alice@example.com");
    }
    
    #[test]
    fn test_user_validation() {
        let user = User::new("".to_string(), "alice@example.com".to_string());
        assert!(user.is_err());
        
        let user = User::new("Alice".to_string(), "invalid-email".to_string());
        assert!(user.is_err());
    }
    
    #[test]
    fn test_user_search() {
        let user = User::new("Alice Smith".to_string(), "alice@example.com".to_string()).unwrap();
        assert!(user.matches_query("alice"));
        assert!(user.matches_query("smith"));
        assert!(user.matches_query("example"));
        assert!(!user.matches_query("bob"));
    }
    
    #[test]
    fn test_domain_extraction() {
        let user = User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap();
        assert_eq!(user.domain(), Some("example.com"));
    }
}
