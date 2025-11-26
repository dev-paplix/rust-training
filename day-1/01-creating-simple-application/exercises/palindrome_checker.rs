use std::io;

// Exercise 3: Palindrome Checker
// Checks if a word is a palindrome

fn main() {
    println!("=== Palindrome Checker ===");
    println!("Enter a word:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    
    let word = input.trim().to_lowercase();
    
    if word.is_empty() {
        println!("Please enter a valid word!");
        return;
    }
    
    if is_palindrome(&word) {
        println!("'{}' is a palindrome! ✅", word);
    } else {
        println!("'{}' is not a palindrome. ❌", word);
    }
    
    // Show the reversed word
    let reversed: String = word.chars().rev().collect();
    println!("Reversed: {}", reversed);
}

fn is_palindrome(word: &str) -> bool {
    let chars: Vec<char> = word.chars().collect();
    let len = chars.len();
    
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_palindromes() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("level"));
        assert!(is_palindrome("noon"));
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("hello"));
        assert!(!is_palindrome("rust"));
    }
}
