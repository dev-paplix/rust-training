//! String validation functions

/// Checks if a string is a palindrome
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::is_palindrome;
/// assert!(is_palindrome("racecar"));
/// assert!(is_palindrome("A man a plan a canal Panama"));
/// assert!(!is_palindrome("hello"));
/// ```
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap())
        .collect();
    
    cleaned == cleaned.chars().rev().collect::<String>()
}

/// Checks if a string is a valid email (basic check)
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::is_email;
/// assert!(is_email("user@example.com"));
/// assert!(!is_email("invalid"));
/// ```
pub fn is_email(s: &str) -> bool {
    let parts: Vec<&str> = s.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    
    let local = parts[0];
    let domain = parts[1];
    
    !local.is_empty() && !domain.is_empty() && domain.contains('.')
}

/// Checks if a string is a valid URL (basic check)
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::is_url;
/// assert!(is_url("https://example.com"));
/// assert!(is_url("http://test.org"));
/// assert!(!is_url("not a url"));
/// ```
pub fn is_url(s: &str) -> bool {
    s.starts_with("http://") || s.starts_with("https://")
}

/// Checks if string contains only alphabetic characters
pub fn is_alpha(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphabetic())
}

/// Checks if string contains only numeric characters
pub fn is_numeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_numeric())
}

/// Checks if string contains only alphanumeric characters
pub fn is_alphanumeric(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_is_email() {
        assert!(is_email("user@example.com"));
        assert!(is_email("test@test.co.uk"));
        assert!(!is_email("invalid"));
        assert!(!is_email("@example.com"));
    }

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://test.org"));
        assert!(!is_url("not a url"));
    }

    #[test]
    fn test_is_alpha() {
        assert!(is_alpha("hello"));
        assert!(!is_alpha("hello123"));
        assert!(!is_alpha(""));
    }

    #[test]
    fn test_is_numeric() {
        assert!(is_numeric("12345"));
        assert!(!is_numeric("123abc"));
        assert!(!is_numeric(""));
    }

    #[test]
    fn test_is_alphanumeric() {
        assert!(is_alphanumeric("hello123"));
        assert!(!is_alphanumeric("hello 123"));
        assert!(!is_alphanumeric(""));
    }
}
