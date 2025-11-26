pub mod validators {
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
    
    pub fn validate_phone(phone: &str) -> bool {
        let digit_count = phone.chars().filter(|c| c.is_numeric()).count();
        digit_count >= 10 && digit_count <= 15
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_email_validation() {
            assert!(validate_email("test@example.com"));
            assert!(validate_email("user+tag@domain.co.uk"));
            assert!(!validate_email("invalid"));
            assert!(!validate_email("@domain.com"));
            assert!(!validate_email("user@"));
        }
        
        #[test]
        fn test_name_validation() {
            assert!(validate_name("Alice"));
            assert!(validate_name("Bob Smith"));
            assert!(!validate_name(""));
            assert!(!validate_name("   "));
        }
    }
}

pub mod formatters {
    pub fn truncate_string(s: &str, max_len: usize) -> String {
        if s.len() <= max_len {
            s.to_string()
        } else {
            format!("{}...", &s[0..max_len.saturating_sub(3)])
        }
    }
    
    pub fn format_currency(amount: f64) -> String {
        format!("${:.2}", amount)
    }
    
    pub fn format_percentage(value: f64) -> String {
        format!("{:.1}%", value * 100.0)
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
        fn test_truncate_string() {
            assert_eq!(truncate_string("Hello", 10), "Hello");
            assert_eq!(truncate_string("Hello World", 8), "Hello...");
            assert_eq!(truncate_string("Hi", 5), "Hi");
        }
        
        #[test]
        fn test_format_currency() {
            assert_eq!(format_currency(10.5), "$10.50");
            assert_eq!(format_currency(100.0), "$100.00");
        }
    }
}

pub mod helpers {
    use std::io::{self, Write};
    
    pub fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }
    
    pub fn confirm(message: &str) -> bool {
        let response = get_input(&format!("{} (yes/no): ", message));
        response.to_lowercase() == "yes" || response.to_lowercase() == "y"
    }
    
    pub fn parse_number<T: std::str::FromStr>(input: &str) -> Option<T> {
        input.parse().ok()
    }
}
