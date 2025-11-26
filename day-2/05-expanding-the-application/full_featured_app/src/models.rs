use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
    created_at: String,
}

impl User {
    pub fn new(name: String, email: String, age: u32) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if !email.contains('@') || !email.contains('.') {
            return Err("Invalid email format".to_string());
        }
        
        if age == 0 || age > 150 {
            return Err("Age must be between 1 and 150".to_string());
        }
        
        Ok(Self {
            id: 0,
            name,
            email,
            age,
            created_at: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
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
    
    pub fn age(&self) -> u32 {
        self.age
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
        if !email.contains('@') || !email.contains('.') {
            return Err("Invalid email format".to_string());
        }
        self.email = email;
        Ok(())
    }
    
    pub fn set_age(&mut self, age: u32) -> Result<(), String> {
        if age == 0 || age > 150 {
            return Err("Age must be between 1 and 150".to_string());
        }
        self.age = age;
        Ok(())
    }
    
    pub fn matches_query(&self, query: &str) -> bool {
        let query_lower = query.to_lowercase();
        self.name.to_lowercase().contains(&query_lower) ||
        self.email.to_lowercase().contains(&query_lower) ||
        self.age.to_string().contains(&query_lower)
    }
    
    pub fn email_domain(&self) -> Option<&str> {
        self.email.split('@').nth(1)
    }
    
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new("Alice".to_string(), "alice@example.com".to_string(), 25);
        assert!(user.is_ok());
    }
    
    #[test]
    fn test_user_validation() {
        assert!(User::new("".to_string(), "alice@example.com".to_string(), 25).is_err());
        assert!(User::new("Alice".to_string(), "invalid".to_string(), 25).is_err());
        assert!(User::new("Alice".to_string(), "alice@example.com".to_string(), 0).is_err());
        assert!(User::new("Alice".to_string(), "alice@example.com".to_string(), 200).is_err());
    }
    
    #[test]
    fn test_search() {
        let user = User::new("Alice Smith".to_string(), "alice@example.com".to_string(), 25).unwrap();
        assert!(user.matches_query("alice"));
        assert!(user.matches_query("smith"));
        assert!(user.matches_query("25"));
        assert!(user.matches_query("example"));
    }
}
