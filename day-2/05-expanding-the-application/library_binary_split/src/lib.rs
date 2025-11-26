// Library crate for user management
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Result<Self, String> {
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Invalid email address".to_string());
        }
        
        Ok(User { id, username, email })
    }
}

pub struct UserRepository {
    users: Vec<User>,
    next_id: u32,
}

impl UserRepository {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn create(&mut self, username: String, email: String) -> Result<User, String> {
        let user = User::new(self.next_id, username, email)?;
        self.users.push(user.clone());
        self.next_id += 1;
        Ok(user)
    }
    
    pub fn find_by_id(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    
    pub fn find_by_username(&self, username: &str) -> Option<&User> {
        self.users.iter().find(|u| u.username == username)
    }
    
    pub fn all(&self) -> &[User] {
        &self.users
    }
    
    pub fn update(&mut self, id: u32, username: String, email: String) -> Result<(), String> {
        let user = self.users.iter_mut()
            .find(|u| u.id == id)
            .ok_or("User not found")?;
        
        if username.is_empty() {
            return Err("Username cannot be empty".to_string());
        }
        
        if !email.contains('@') {
            return Err("Invalid email address".to_string());
        }
        
        user.username = username;
        user.email = email;
        Ok(())
    }
    
    pub fn delete(&mut self, id: u32) -> Result<(), String> {
        let pos = self.users.iter()
            .position(|u| u.id == id)
            .ok_or("User not found")?;
        
        self.users.remove(pos);
        Ok(())
    }
    
    pub fn count(&self) -> usize {
        self.users.len()
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_user_creation() {
        let user = User::new(1, "alice".to_string(), "alice@example.com".to_string());
        assert!(user.is_ok());
        
        let user = user.unwrap();
        assert_eq!(user.id, 1);
        assert_eq!(user.username, "alice");
        assert_eq!(user.email, "alice@example.com");
    }
    
    #[test]
    fn test_user_validation() {
        let user = User::new(1, "".to_string(), "alice@example.com".to_string());
        assert!(user.is_err());
        
        let user = User::new(1, "alice".to_string(), "invalid".to_string());
        assert!(user.is_err());
    }
    
    #[test]
    fn test_repository_create() {
        let mut repo = UserRepository::new();
        let user = repo.create("alice".to_string(), "alice@example.com".to_string());
        
        assert!(user.is_ok());
        assert_eq!(repo.count(), 1);
    }
    
    #[test]
    fn test_repository_find() {
        let mut repo = UserRepository::new();
        let user = repo.create("alice".to_string(), "alice@example.com".to_string()).unwrap();
        
        let found = repo.find_by_id(user.id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().username, "alice");
        
        let found = repo.find_by_username("alice");
        assert!(found.is_some());
    }
    
    #[test]
    fn test_repository_update() {
        let mut repo = UserRepository::new();
        let user = repo.create("alice".to_string(), "alice@example.com".to_string()).unwrap();
        
        let result = repo.update(user.id, "alice_updated".to_string(), "alice2@example.com".to_string());
        assert!(result.is_ok());
        
        let updated = repo.find_by_id(user.id).unwrap();
        assert_eq!(updated.username, "alice_updated");
        assert_eq!(updated.email, "alice2@example.com");
    }
    
    #[test]
    fn test_repository_delete() {
        let mut repo = UserRepository::new();
        let user = repo.create("alice".to_string(), "alice@example.com".to_string()).unwrap();
        
        assert_eq!(repo.count(), 1);
        
        let result = repo.delete(user.id);
        assert!(result.is_ok());
        assert_eq!(repo.count(), 0);
    }
}
