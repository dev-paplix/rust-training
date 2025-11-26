//! Data processing and transformation

use crate::Result;

/// Main data processor
pub struct DataProcessor {
    data: Vec<String>,
}

impl DataProcessor {
    /// Creates a new data processor
    pub fn new() -> Self {
        DataProcessor { data: Vec::new() }
    }

    /// Adds data to the processor
    pub fn add(&mut self, item: String) -> &mut Self {
        self.data.push(item);
        self
    }

    /// Adds multiple items
    pub fn add_all(&mut self, items: Vec<String>) -> &mut Self {
        self.data.extend(items);
        self
    }

    /// Filters data by predicate
    pub fn filter<F>(&self, predicate: F) -> Vec<String>
    where
        F: Fn(&String) -> bool,
    {
        self.data.iter().filter(|item| predicate(item)).cloned().collect()
    }

    /// Maps data to new values
    pub fn map<F>(&self, mapper: F) -> Vec<String>
    where
        F: Fn(&String) -> String,
    {
        self.data.iter().map(mapper).collect()
    }

    /// Gets all data
    pub fn get_all(&self) -> &[String] {
        &self.data
    }

    /// Clears all data
    pub fn clear(&mut self) {
        self.data.clear();
    }

    /// Gets count of items
    pub fn count(&self) -> usize {
        self.data.len()
    }

    /// Checks if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for DataProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut processor = DataProcessor::new();
        processor.add("item1".to_string());
        assert_eq!(processor.count(), 1);
    }

    #[test]
    fn test_add_all() {
        let mut processor = DataProcessor::new();
        processor.add_all(vec!["a".to_string(), "b".to_string()]);
        assert_eq!(processor.count(), 2);
    }

    #[test]
    fn test_filter() {
        let mut processor = DataProcessor::new();
        processor.add_all(vec!["apple".to_string(), "banana".to_string(), "apricot".to_string()]);
        
        let filtered = processor.filter(|s| s.starts_with('a'));
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_map() {
        let mut processor = DataProcessor::new();
        processor.add_all(vec!["hello".to_string(), "world".to_string()]);
        
        let mapped = processor.map(|s| s.to_uppercase());
        assert_eq!(mapped, vec!["HELLO", "WORLD"]);
    }

    #[test]
    fn test_clear() {
        let mut processor = DataProcessor::new();
        processor.add("test".to_string());
        processor.clear();
        assert!(processor.is_empty());
    }
}
