// Word Counter and Text Analysis
// Demonstrates HashMap, HashSet, and string processing

use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    println!("=== Word Counter and Text Analyzer ===\n");
    println!("Enter your text below.");
    println!("When finished, press Enter, then Ctrl+Z, then Enter again (Windows)");
    println!("or Ctrl+D (Unix/Linux/Mac)");
    println!("─────────────────────────────────────\n");
    
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("Failed to read input");
    
    if text.trim().is_empty() {
        println!("No text entered!");
        return;
    }
    
    analyze_text(&text);
}

fn analyze_text(text: &str) {
    println!("\n=== Text Analysis Results ===\n");
    
    // Basic statistics
    let char_count = text.len();
    let word_count = count_words(text);
    let line_count = text.lines().count();
    let sentence_count = count_sentences(text);
    let paragraph_count = count_paragraphs(text);
    
    println!("📊 Basic Statistics:");
    println!("   Total Characters: {}", char_count);
    println!("   Total Words: {}", word_count);
    println!("   Total Lines: {}", line_count);
    println!("   Total Sentences: {}", sentence_count);
    println!("   Total Paragraphs: {}", paragraph_count);
    println!("   Average words per sentence: {:.1}", word_count as f64 / sentence_count.max(1) as f64);
    println!("   Average sentences per paragraph: {:.1}", sentence_count as f64 / paragraph_count.max(1) as f64);
    
    // Character breakdown
    let (alpha, digits, spaces, special) = character_breakdown(text);
    println!("\n📝 Character Breakdown:");
    println!("   Alphabetic: {} ({:.1}%)", alpha, (alpha as f64 / char_count as f64) * 100.0);
    println!("   Digits: {} ({:.1}%)", digits, (digits as f64 / char_count as f64) * 100.0);
    println!("   Whitespace: {} ({:.1}%)", spaces, (spaces as f64 / char_count as f64) * 100.0);
    println!("   Special/Punctuation: {} ({:.1}%)", special, (special as f64 / char_count as f64) * 100.0);
    
    // Word frequency
    println!("\n📈 Word Frequency Analysis:");
    let word_freq = word_frequency(text);
    display_top_words(&word_freq, 10);
    
    // Unique words
    let unique_words = unique_word_count(text);
    let lexical_diversity = (unique_words as f64 / word_count as f64) * 100.0;
    println!("\n🔤 Vocabulary Analysis:");
    println!("   Total unique words: {}", unique_words);
    println!("   Lexical diversity: {:.1}% (unique/total ratio)", lexical_diversity);
    println!("   Repeated words: {}", word_count - unique_words);
    
    // Word length distribution
    println!("\n📏 Word Length Distribution:");
    let length_dist = word_length_distribution(text);
    display_word_length_distribution(&length_dist);
    
    // Longest and shortest words
    if let Some((longest, shortest)) = find_longest_shortest_words(text) {
        println!("\n🏆 Extreme Words:");
        println!("   Longest: '{}' ({} characters)", longest, longest.len());
        println!("   Shortest: '{}' ({} character{})", shortest, shortest.len(), if shortest.len() > 1 { "s" } else { "" });
    }
    
    // Average word length
    let avg_length = average_word_length(text);
    println!("   Average length: {:.2} characters", avg_length);
    
    // Character frequency
    println!("\n🔡 Letter Frequency (Top 10):");
    let char_freq = character_frequency(text);
    display_top_chars(&char_freq, 10);
    
    // Common word patterns
    println!("\n🔍 Pattern Analysis:");
    analyze_patterns(text);
    
    // Readability metrics
    println!("\n📖 Readability Metrics:");
    calculate_readability(text, word_count, sentence_count);
}

fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut freq_map = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word
            .to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        
        if !word.is_empty() {
            *freq_map.entry(word).or_insert(0) += 1;
        }
    }
    
    freq_map
}

fn display_top_words(freq_map: &HashMap<String, usize>, n: usize) {
    let mut words: Vec<(&String, &usize)> = freq_map.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("   Top {} words:", n);
    for (i, (word, count)) in words.iter().take(n).enumerate() {
        println!("   {}. '{}': {} times", i + 1, word, count);
    }
}

fn unique_word_count(text: &str) -> usize {
    let mut unique_words = HashSet::new();
    
    for word in text.split_whitespace() {
        let word = word
            .to_lowercase()
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_string();
        
        if !word.is_empty() {
            unique_words.insert(word);
        }
    }
    
    unique_words.len()
}

fn character_frequency(text: &str) -> HashMap<char, usize> {
    let mut freq_map = HashMap::new();
    
    for ch in text.chars() {
        if ch.is_alphabetic() {
            let ch = ch.to_lowercase().next().unwrap();
            *freq_map.entry(ch).or_insert(0) += 1;
        }
    }
    
    freq_map
}

fn display_top_chars(freq_map: &HashMap<char, usize>, n: usize) {
    let mut chars: Vec<(&char, &usize)> = freq_map.iter().collect();
    chars.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("   Top {} characters:", n);
    for (i, (ch, count)) in chars.iter().take(n).enumerate() {
        println!("   {}. '{}': {} times", i + 1, ch, count);
    }
}

fn find_longest_shortest_words(text: &str) -> Option<(String, String)> {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| !c.is_alphanumeric())
                .to_string()
        })
        .filter(|w| !w.is_empty())
        .collect();
    
    if words.is_empty() {
        return None;
    }
    
    let longest = words.iter().max_by_key(|w| w.len())?.clone();
    let shortest = words.iter().min_by_key(|w| w.len())?.clone();
    
    Some((longest, shortest))
}

fn average_word_length(text: &str) -> f64 {
    let words: Vec<&str> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .collect();
    
    if words.is_empty() {
        return 0.0;
    }
    
    let total_length: usize = words.iter().map(|w| w.len()).sum();
    total_length as f64 / words.len() as f64
}

fn count_sentences(text: &str) -> usize {
    text.chars()
        .filter(|&c| c == '.' || c == '!' || c == '?')
        .count()
        .max(1)
}

fn count_paragraphs(text: &str) -> usize {
    text.split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .count()
        .max(1)
}

fn character_breakdown(text: &str) -> (usize, usize, usize, usize) {
    let mut alphabetic = 0;
    let mut digits = 0;
    let mut whitespace = 0;
    let mut special = 0;
    
    for ch in text.chars() {
        if ch.is_alphabetic() {
            alphabetic += 1;
        } else if ch.is_numeric() {
            digits += 1;
        } else if ch.is_whitespace() {
            whitespace += 1;
        } else {
            special += 1;
        }
    }
    
    (alphabetic, digits, whitespace, special)
}

fn word_length_distribution(text: &str) -> HashMap<usize, usize> {
    let mut distribution = HashMap::new();
    
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphanumeric());
        if !word.is_empty() {
            *distribution.entry(word.len()).or_insert(0) += 1;
        }
    }
    
    distribution
}

fn display_word_length_distribution(dist: &HashMap<usize, usize>) {
    let mut lengths: Vec<(&usize, &usize)> = dist.iter().collect();
    lengths.sort_by_key(|&(len, _)| len);
    
    for (length, count) in lengths {
        let bar = "█".repeat((*count).min(50));
        println!("   {} chars: {} {}", length, bar, count);
    }
}

fn analyze_patterns(text: &str) {
    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
        .filter(|w| !w.is_empty())
        .collect();
    
    // Count capitalized words (in original text)
    let capitalized = text
        .split_whitespace()
        .filter(|w| {
            w.chars()
                .next()
                .map(|c| c.is_uppercase())
                .unwrap_or(false)
        })
        .count();
    
    println!("   Capitalized words: {}", capitalized);
    
    // Find words with numbers
    let words_with_numbers = words.iter().filter(|w| w.chars().any(|c| c.is_numeric())).count();
    println!("   Words containing numbers: {}", words_with_numbers);
    
    // Common prefixes/suffixes
    let words_ending_ing = words.iter().filter(|w| w.ends_with("ing")).count();
    let words_ending_ed = words.iter().filter(|w| w.ends_with("ed")).count();
    let words_ending_ly = words.iter().filter(|w| w.ends_with("ly")).count();
    
    println!("   Words ending in '-ing': {}", words_ending_ing);
    println!("   Words ending in '-ed': {}", words_ending_ed);
    println!("   Words ending in '-ly': {}", words_ending_ly);
}

fn calculate_readability(text: &str, word_count: usize, sentence_count: usize) {
    let syllable_count = estimate_syllables(text);
    
    // Flesch Reading Ease (higher = easier to read, 0-100 scale)
    let flesch_score = 206.835 
        - 1.015 * (word_count as f64 / sentence_count as f64)
        - 84.6 * (syllable_count as f64 / word_count as f64);
    
    println!("   Flesch Reading Ease: {:.1}", flesch_score.max(0.0).min(100.0));
    println!("   {}", interpret_flesch_score(flesch_score));
    
    // Average syllables per word
    println!("   Average syllables per word: {:.2}", syllable_count as f64 / word_count as f64);
}

fn estimate_syllables(text: &str) -> usize {
    let mut total = 0;
    
    for word in text.split_whitespace() {
        let word = word.trim_matches(|c: char| !c.is_alphabetic()).to_lowercase();
        if word.is_empty() {
            continue;
        }
        
        let mut count = 0;
        let mut prev_vowel = false;
        
        for ch in word.chars() {
            let is_vowel = matches!(ch, 'a' | 'e' | 'i' | 'o' | 'u' | 'y');
            if is_vowel && !prev_vowel {
                count += 1;
            }
            prev_vowel = is_vowel;
        }
        
        // Adjust for silent 'e'
        if word.ends_with('e') && count > 1 {
            count -= 1;
        }
        
        total += count.max(1);
    }
    
    total
}

fn interpret_flesch_score(score: f64) -> &'static str {
    match score as i32 {
        90..=100 => "   → Very easy to read (5th grade level)",
        80..=89 => "   → Easy to read (6th grade level)",
        70..=79 => "   → Fairly easy (7th grade level)",
        60..=69 => "   → Standard (8th-9th grade level)",
        50..=59 => "   → Fairly difficult (10th-12th grade level)",
        30..=49 => "   → Difficult (College level)",
        _ => "   → Very difficult (College graduate level)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_word_count() {
        assert_eq!(count_words("hello world"), 2);
        assert_eq!(count_words("one two three"), 3);
    }
    
    #[test]
    fn test_word_frequency() {
        let text = "hello world hello";
        let freq = word_frequency(text);
        assert_eq!(freq.get("hello"), Some(&2));
        assert_eq!(freq.get("world"), Some(&1));
    }
    
    #[test]
    fn test_unique_words() {
        let text = "hello world hello rust world";
        assert_eq!(unique_word_count(text), 3);
    }
}
