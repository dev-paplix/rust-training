use crate::models::User;
use crate::errors::AppResult;
use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UserData {
    users: Vec<User>,
    next_id: u32,
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
    
    pub fn create_user(&mut self, name: String, email: String, age: u32) -> AppResult<u32> {
        // Check for duplicate email
        if self.users.iter().any(|u| u.email() == email) {
            return Err(format!("User with email {} already exists", email));
        }
        
        let user = User::new(name, email, age)?;
        let id = self.next_id;
        let user = user.with_id(id);
        
        self.users.push(user);
        self.next_id += 1;
        
        Ok(id)
    }
    
    pub fn find_user(&self, id: u32) -> Option<&User> {
        self.users.iter().find(|u| u.id() == id)
    }
    
    pub fn find_user_mut(&mut self, id: u32) -> Option<&mut User> {
        self.users.iter_mut().find(|u| u.id() == id)
    }
    
    pub fn update_user(&mut self, id: u32, name: String, email: String, age: u32) -> AppResult<()> {
        // Check if email is already used by another user
        if let Some(existing) = self.users.iter().find(|u| u.email() == email && u.id() != id) {
            return Err(format!("Email {} is already used by user {}", email, existing.id()));
        }
        
        let user = self.find_user_mut(id)
            .ok_or_else(|| format!("User with ID {} not found", id))?;
        
        user.set_name(name)?;
        user.set_email(email)?;
        user.set_age(age)?;
        
        Ok(())
    }
    
    pub fn delete_user(&mut self, id: u32) -> AppResult<()> {
        let pos = self.users.iter()
            .position(|u| u.id() == id)
            .ok_or_else(|| format!("User with ID {} not found", id))?;
        
        self.users.remove(pos);
        Ok(())
    }
    
    pub fn all_users(&self) -> &[User] {
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
        
        if total_users == 0 {
            return UserStatistics::empty();
        }
        
        // Age statistics
        let ages: Vec<u32> = self.users.iter().map(|u| u.age()).collect();
        let min_age = *ages.iter().min().unwrap();
        let max_age = *ages.iter().max().unwrap();
        let avg_age = ages.iter().sum::<u32>() as f64 / ages.len() as f64;
        
        // Email domains
        let mut email_domains: HashMap<String, usize> = HashMap::new();
        for user in &self.users {
            if let Some(domain) = user.email_domain() {
                *email_domains.entry(domain.to_string()).or_insert(0) += 1;
            }
        }
        
        // Name lengths
        let lengths: Vec<usize> = self.users.iter().map(|u| u.name().len()).collect();
        let min_len = *lengths.iter().min().unwrap();
        let max_len = *lengths.iter().max().unwrap();
        let avg_len = lengths.iter().sum::<usize>() as f64 / lengths.len() as f64;
        
        UserStatistics {
            total_users,
            age_stats: (min_age, max_age, avg_age),
            email_domains,
            name_lengths: (min_len, max_len, avg_len),
        }
    }
    
    pub fn save_to_json(&self, filename: &str) -> AppResult<()> {
        let data = UserData {
            users: self.users.clone(),
            next_id: self.next_id,
        };
        
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Serialization error: {}", e))?;
        
        fs::write(filename, json)
            .map_err(|e| format!("File write error: {}", e))
    }
    
    pub fn load_from_json(&mut self, filename: &str) -> AppResult<usize> {
        let content = fs::read_to_string(filename)
            .map_err(|e| format!("File read error: {}", e))?;
        
        let data: UserData = serde_json::from_str(&content)
            .map_err(|e| format!("Deserialization error: {}", e))?;
        
        let count = data.users.len();
        self.users = data.users;
        self.next_id = data.next_id;
        
        Ok(count)
    }
    
    pub fn export_to_csv(&self, filename: &str) -> AppResult<()> {
        use csv::Writer;
        
        let mut wtr = Writer::from_path(filename)
            .map_err(|e| format!("CSV writer error: {}", e))?;
        
        wtr.write_record(&["id", "name", "email", "age", "created_at"])
            .map_err(|e| format!("CSV write error: {}", e))?;
        
        for user in &self.users {
            wtr.write_record(&[
                user.id().to_string(),
                user.name().to_string(),
                user.email().to_string(),
                user.age().to_string(),
                user.created_at().to_string(),
            ]).map_err(|e| format!("CSV write error: {}", e))?;
        }
        
        wtr.flush().map_err(|e| format!("CSV flush error: {}", e))
    }
    
    pub fn import_from_csv(&mut self, filename: &str) -> AppResult<usize> {
        use csv::Reader;
        
        let mut rdr = Reader::from_path(filename)
            .map_err(|e| format!("CSV reader error: {}", e))?;
        
        let mut count = 0;
        
        for result in rdr.records() {
            let record = result.map_err(|e| format!("CSV read error: {}", e))?;
            
            if record.len() < 3 {
                continue;
            }
            
            let name = record.get(1).unwrap_or("").to_string();
            let email = record.get(2).unwrap_or("").to_string();
            let age = record.get(3).unwrap_or("0").parse::<u32>().unwrap_or(0);
            
            if let Ok(_) = self.create_user(name, email, age) {
                count += 1;
            }
        }
        
        Ok(count)
    }
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub struct UserStatistics {
    pub total_users: usize,
    pub age_stats: (u32, u32, f64), // min, max, average
    pub email_domains: HashMap<String, usize>,
    pub name_lengths: (usize, usize, f64), // min, max, average
}

impl UserStatistics {
    fn empty() -> Self {
        Self {
            total_users: 0,
            age_stats: (0, 0, 0.0),
            email_domains: HashMap::new(),
            name_lengths: (0, 0, 0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_user() {
        let mut repo = UserRepository::new();
        let id = repo.create_user("Alice".to_string(), "alice@example.com".to_string(), 25);
        assert!(id.is_ok());
        assert_eq!(repo.count(), 1);
    }
    
    #[test]
    fn test_duplicate_email() {
        let mut repo = UserRepository::new();
        repo.create_user("Alice".to_string(), "alice@example.com".to_string(), 25).unwrap();
        let result = repo.create_user("Bob".to_string(), "alice@example.com".to_string(), 30);
        assert!(result.is_err());
    }
}
