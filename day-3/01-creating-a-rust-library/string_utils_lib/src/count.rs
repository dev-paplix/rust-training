//! String counting and analysis functions

/// Counts vowels in a string
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::count_vowels;
/// assert_eq!(count_vowels("hello"), 2);
/// assert_eq!(count_vowels("aeiou"), 5);
/// ```
pub fn count_vowels(s: &str) -> usize {
    s.chars()
        .filter(|c| matches!(c.to_lowercase().next(), Some('a' | 'e' | 'i' | 'o' | 'u')))
        .count()
}

/// Counts consonants in a string
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::count_consonants;
/// assert_eq!(count_consonants("hello"), 3);
/// ```
pub fn count_consonants(s: &str) -> usize {
    s.chars()
        .filter(|c| c.is_alphabetic() && !matches!(c.to_lowercase().next(), Some('a' | 'e' | 'i' | 'o' | 'u')))
        .count()
}

/// Counts words in a string
/// 
/// # Examples
/// 
/// ```
/// use string_utils_lib::count_words;
/// assert_eq!(count_words("hello world"), 2);
/// assert_eq!(count_words("  spaces   everywhere  "), 2);
/// ```
pub fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Counts lines in a string
pub fn count_lines(s: &str) -> usize {
    if s.is_empty() {
        0
    } else {
        s.lines().count()
    }
}

/// Counts occurrences of a character
pub fn count_char(s: &str, ch: char) -> usize {
    s.chars().filter(|&c| c == ch).count()
}

/// Counts occurrences of a substring
pub fn count_substring(s: &str, pattern: &str) -> usize {
    if pattern.is_empty() {
        return 0;
    }
    s.matches(pattern).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_vowels() {
        assert_eq!(count_vowels("hello"), 2);
        assert_eq!(count_vowels("aeiou"), 5);
        assert_eq!(count_vowels("xyz"), 0);
    }

    #[test]
    fn test_count_consonants() {
        assert_eq!(count_consonants("hello"), 3);
        assert_eq!(count_consonants("aeiou"), 0);
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("  spaces   everywhere  "), 2);
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_count_lines() {
        assert_eq!(count_lines("line1\nline2\nline3"), 3);
        assert_eq!(count_lines(""), 0);
    }

    #[test]
    fn test_count_char() {
        assert_eq!(count_char("hello", 'l'), 2);
        assert_eq!(count_char("hello", 'z'), 0);
    }

    #[test]
    fn test_count_substring() {
        assert_eq!(count_substring("hello hello world", "hello"), 2);
        assert_eq!(count_substring("test", ""), 0);
    }
}
