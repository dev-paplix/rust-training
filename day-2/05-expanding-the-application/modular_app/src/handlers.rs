use crate::models::User;
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    users: Vec<User>,
    next_id: u32,
}

pub struct UserHandler {
    users: Vec<User>,
    next_id: u32,
}

impl UserHandler {
    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            next_id: 1,
        }
    }
    
    pub fn add_user(&mut self, user: User) -> Result<u32, String> {
        // Check for duplicate email
        if self.users.iter().any(|u| u.email() == user.email()) {
            return Err(format!("User with email {} already exists", user.email()));
        }
        
        let id = self.next_id;
        let user = user.with_id(id);
        self.users.push(user);
        self.next_id += 1;
        
        Ok(id)
    }
    
    pub fn get_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id() == id)
    }
    
    pub fn get_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id() == id)
    }
    
    pub fn update_user(&mut self, id: u32, name: String, email: String) -> Result<(), String> {
        // Check if email is already used by another user
        if let Some(existing) = self.users.iter().find(|u| u.email() == email && u.id() != id) {
            return Err(format!("Email {} is already used by user {}", email, existing.id()));
        }
        
        let user = self.get_user_mut(id)
            .ok_or_else(|| format!("User with ID {} not found", id))?;
        
        user.set_name(name)?;
        user.set_email(email)?;
        
        Ok(())
    }
    
    pub fn delete_user(&mut self, id: u32) -> Result<(), String> {
        let pos = self.users.iter()
            .position(|u| u.id() == id)
            .ok_or_else(|| format!("User with ID {} not found", id))?;
        
        self.users.remove(pos);
        Ok(())
    }
    
    pub fn list_users(&self) -> &[User] {
        &self.users
    }
    
    pub fn search(&self, query: &str) -> Vec<&User> {
        self.users.iter()
            .filter(|u| u.matches_query(query))
            .collect()
    }
    
    pub fn count(&self) -> usize {
        self.users.len()
    }
    
    pub fn get_statistics(&self) -> UserStatistics {
        let total_users = self.users.len();
        
        // Count users by email domain
        let mut email_domains: HashMap<String, usize> = HashMap::new();
        for user in &self.users {
            if let Some(domain) = user.domain() {
                *email_domains.entry(domain.to_string()).or_insert(0) += 1;
            }
        }
        
        // Calculate name length statistics
        let name_lengths = if !self.users.is_empty() {
            let lengths: Vec<usize> = self.users.iter()
                .map(|u| u.name().len())
                .collect();
            
            let min = *lengths.iter().min().unwrap();
            let max = *lengths.iter().max().unwrap();
            let avg = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
            
            (min, max, avg)
        } else {
            (0, 0, 0.0)
        };
        
        UserStatistics {
            total_users,
            email_domains,
            name_lengths,
        }
    }
    
    pub fn save_to_file(&self, filename: &str) -> Result<(), String> {
        let data = UserData {
            users: self.users.clone(),
            next_id: self.next_id,
        };
        
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        fs::write(filename, json)
            .map_err(|e| format!("File write error: {}", e))
    }
    
    pub fn load_from_file(&mut self, filename: &str) -> Result<usize, String> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("File read error: {}", e))?;
        
        let data: UserData = serde_json::from_str(&content)
            .map_err(|e| format!("Deserialization error: {}", e))?;
        
        let count = data.users.len();
        self.users = data.users;
        self.next_id = data.next_id;
        
        Ok(count)
    }
}

pub struct UserStatistics {
    pub total_users: usize,
    pub email_domains: HashMap<String, usize>,
    pub name_lengths: (usize, usize, f64), // min, max, average
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add_user() {
        let mut handler = UserHandler::new();
        let user = User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap();
        
        let id = handler.add_user(user);
        assert!(id.is_ok());
        assert_eq!(handler.count(), 1);
    }
    
    #[test]
    fn test_duplicate_email() {
        let mut handler = UserHandler::new();
        let user1 = User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap();
        let user2 = User::new("Bob".to_string(), "alice@example.com".to_string()).unwrap();
        
        assert!(handler.add_user(user1).is_ok());
        assert!(handler.add_user(user2).is_err());
    }
    
    #[test]
    fn test_update_user() {
        let mut handler = UserHandler::new();
        let user = User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap();
        let id = handler.add_user(user).unwrap();
        
        let result = handler.update_user(id, "Alice Smith".to_string(), "asmith@example.com".to_string());
        assert!(result.is_ok());
        
        let updated = handler.get_user(id).unwrap();
        assert_eq!(updated.name(), "Alice Smith");
    }
    
    #[test]
    fn test_delete_user() {
        let mut handler = UserHandler::new();
        let user = User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap();
        let id = handler.add_user(user).unwrap();
        
        assert!(handler.delete_user(id).is_ok());
        assert_eq!(handler.count(), 0);
    }
    
    #[test]
    fn test_search() {
        let mut handler = UserHandler::new();
        handler.add_user(User::new("Alice".to_string(), "alice@example.com".to_string()).unwrap()).unwrap();
        handler.add_user(User::new("Bob".to_string(), "bob@test.com".to_string()).unwrap()).unwrap();
        handler.add_user(User::new("Alice Smith".to_string(), "asmith@example.com".to_string()).unwrap()).unwrap();
        
        let results = handler.search("Alice");
        assert_eq!(results.len(), 2);
        
        let results = handler.search("example");
        assert_eq!(results.len(), 2);
    }
}
