pub fn validate_email(email: &str) -> bool {
    email.contains('@') && 
    email.contains('.') &&
    email.len() >= 5 &&
    email.len() <= 100
}

pub fn validate_name(name: &str) -> bool {
    !name.is_empty() && 
    name.len() <= 100 &&
    !name.chars().all(|c| c.is_whitespace())
}

pub fn validate_age(age: u32) -> bool {
    age > 0 && age <= 150
}

pub fn truncate_string(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[0..max_len.saturating_sub(3)])
    }
}

pub fn format_list<T: std::fmt::Display>(items: &[T], separator: &str) -> String {
    items.iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_email_validation() {
        assert!(validate_email("test@example.com"));
        assert!(!validate_email("invalid"));
    }
    
    #[test]
    fn test_name_validation() {
        assert!(validate_name("Alice"));
        assert!(!validate_name(""));
    }
    
    #[test]
    fn test_age_validation() {
        assert!(validate_age(25));
        assert!(!validate_age(0));
        assert!(!validate_age(200));
    }
}
