//! String transformation functions

/// Capitalizes the first letter of a string
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::capitalize;
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize(""), "");
/// ```
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

/// Reverses a string
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::reverse;
/// assert_eq!(reverse("hello"), "olleh");
/// assert_eq!(reverse("rust"), "tsur");
/// ```
pub fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}

/// Converts a string to title case
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::to_title_case;
/// assert_eq!(to_title_case("hello world"), "Hello World");
/// ```
pub fn to_title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| capitalize(word))
        .collect::<Vec<_>>()
        .join(" ")
}

/// Converts a string to snake_case
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::to_snake_case;
/// assert_eq!(to_snake_case("Hello World"), "hello_world");
/// assert_eq!(to_snake_case("HelloWorld"), "hello_world");
/// ```
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    let mut prev_was_separator = false;
    
    while let Some(c) = chars.next() {
        if c == ' ' || c == '_' {
            if !result.is_empty() && !prev_was_separator {
                result.push('_');
                prev_was_separator = true;
            }
        } else if c.is_uppercase() && !result.is_empty() {
            if !prev_was_separator {
                if let Some(&next) = chars.peek() {
                    if !next.is_uppercase() && next != ' ' {
                        result.push('_');
                    }
                }
            }
            result.push(c.to_lowercase().next().unwrap());
            prev_was_separator = false;
        } else {
            result.push(c.to_lowercase().next().unwrap());
            prev_was_separator = false;
        }
    }
    
    result
}

/// Converts a string to camelCase
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::to_camel_case;
/// assert_eq!(to_camel_case("hello world"), "helloWorld");
/// assert_eq!(to_camel_case("hello_world"), "helloWorld");
/// ```
pub fn to_camel_case(s: &str) -> String {
    let words: Vec<&str> = s.split(|c: char| c.is_whitespace() || c == '_').collect();
    
    if words.is_empty() {
        return String::new();
    }
    
    let mut result = words[0].to_lowercase();
    
    for word in &words[1..] {
        if !word.is_empty() {
            result.push_str(&capitalize(word));
        }
    }
    
    result
}

/// Truncates a string to a maximum length
/// 
/// Adds "..." if truncated
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let end = max_len.saturating_sub(3);
        format!("{}...", &s[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("h"), "H");
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse("hello"), "olleh");
        assert_eq!(reverse(""), "");
    }

    #[test]
    fn test_to_title_case() {
        assert_eq!(to_title_case("hello world"), "Hello World");
        assert_eq!(to_title_case("the quick brown fox"), "The Quick Brown Fox");
    }

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("Hello World"), "hello_world");
        assert_eq!(to_snake_case("HelloWorld"), "hello_world");
    }

    #[test]
    fn test_to_camel_case() {
        assert_eq!(to_camel_case("hello world"), "helloWorld");
        assert_eq!(to_camel_case("hello_world"), "helloWorld");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("hello world", 20), "hello world");
        assert_eq!(truncate("hello world", 8), "hello...");
    }
}
