use std::io::{self, Write};
use std::collections::HashMap;

/// Demonstrates common loop patterns and practical applications
fn main() {
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => counting_and_accumulation(),
            2 => searching_patterns(),
            3 => filtering_and_transformation(),
            4 => nested_iteration_patterns(),
            5 => sliding_window(),
            6 => two_pointer_technique(),
            7 => retry_and_backoff(),
            8 => event_loop_simulation(),
            0 => {
                println!("\nThank you for exploring loop patterns!");
                break;
            }
            _ => println!("\n❌ Invalid choice. Please try again."),
        }
        
        pause();
    }
}

fn display_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║       Loop Patterns in Rust           ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Counting & Accumulation");
    println!("2. Searching Patterns");
    println!("3. Filtering & Transformation");
    println!("4. Nested Iteration Patterns");
    println!("5. Sliding Window");
    println!("6. Two Pointer Technique");
    println!("7. Retry & Backoff");
    println!("8. Event Loop Simulation");
    println!("0. Exit");
    print!("\nEnter your choice: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse().unwrap_or(-1)
}

fn pause() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

// 1. Counting and Accumulation
fn counting_and_accumulation() {
    println!("\n=== Counting & Accumulation Patterns ===\n");
    
    println!("Example 1: Count occurrences");
    let text = "hello world, hello rust, hello everyone";
    let target = "hello";
    let count = text.split_whitespace()
        .filter(|word| word.starts_with(target))
        .count();
    println!("Text: '{}'", text);
    println!("'{}' appears {} times", target, count);
    
    println!("\nExample 2: Character frequency");
    let text = "hello world";
    let mut freq: HashMap<char, usize> = HashMap::new();
    
    for ch in text.chars() {
        if ch != ' ' {
            *freq.entry(ch).or_insert(0) += 1;
        }
    }
    
    println!("Text: '{}'", text);
    println!("Character frequencies:");
    for (ch, count) in freq.iter() {
        println!("  '{}': {}", ch, count);
    }
    
    println!("\nExample 3: Sum and average");
    let numbers = vec![10, 20, 30, 40, 50];
    let sum: i32 = numbers.iter().sum();
    let avg = sum as f64 / numbers.len() as f64;
    
    println!("Numbers: {:?}", numbers);
    println!("Sum: {}", sum);
    println!("Average: {:.2}", avg);
    
    println!("\nExample 4: Running totals");
    let transactions = vec![100, -50, 200, -30, 150];
    let mut balance = 0;
    
    println!("Transactions:");
    for (i, &amount) in transactions.iter().enumerate() {
        balance += amount;
        println!("  Transaction {}: {:+5} -> Balance: {}", 
                 i + 1, amount, balance);
    }
    
    println!("\nExample 5: Group and count");
    let grades = vec!['A', 'B', 'A', 'C', 'B', 'A', 'D', 'B', 'C', 'A'];
    let mut grade_count: HashMap<char, usize> = HashMap::new();
    
    for &grade in &grades {
        *grade_count.entry(grade).or_insert(0) += 1;
    }
    
    println!("Grades: {:?}", grades);
    println!("\nGrade distribution:");
    let mut sorted_grades: Vec<_> = grade_count.iter().collect();
    sorted_grades.sort_by_key(|(k, _)| *k);
    
    for (grade, count) in sorted_grades {
        let bar = "*".repeat(*count);
        println!("  {}: {} ({})", grade, bar, count);
    }
}

// 2. Searching Patterns
fn searching_patterns() {
    println!("\n=== Searching Patterns ===\n");
    
    println!("Example 1: Linear search");
    let numbers = vec![5, 2, 8, 1, 9, 3, 7];
    let target = 9;
    
    let mut found_index = None;
    for (i, &num) in numbers.iter().enumerate() {
        if num == target {
            found_index = Some(i);
            break;
        }
    }
    
    println!("Numbers: {:?}", numbers);
    match found_index {
        Some(i) => println!("Found {} at index {}", target, i),
        None => println!("{} not found", target),
    }
    
    println!("\nExample 2: Binary search (sorted array)");
    let mut numbers = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
    numbers.sort();
    
    println!("Sorted: {:?}", numbers);
    match numbers.binary_search(&7) {
        Ok(i) => println!("Found 7 at index {}", i),
        Err(i) => println!("7 not found, would be at index {}", i),
    }
    
    println!("\nExample 3: Find all occurrences");
    let text = "the quick brown fox jumps over the lazy dog";
    let target = "the";
    let positions: Vec<usize> = text
        .split_whitespace()
        .enumerate()
        .filter(|(_, word)| *word == target)
        .map(|(i, _)| i)
        .collect();
    
    println!("Text: '{}'", text);
    println!("'{}' found at word positions: {:?}", target, positions);
    
    println!("\nExample 4: Find first matching condition");
    let numbers = vec![1, 3, 5, 8, 9, 11, 14, 16];
    
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    let first_gt_10 = numbers.iter().find(|&&x| x > 10);
    
    println!("Numbers: {:?}", numbers);
    println!("First even: {:?}", first_even);
    println!("First > 10: {:?}", first_gt_10);
    
    println!("\nExample 5: Find in nested structure");
    let matrix = vec![
        vec![1, 2, 3, 4],
        vec![5, 6, 7, 8],
        vec![9, 10, 11, 12],
    ];
    
    let target = 7;
    let mut position = None;
    
    'search: for (i, row) in matrix.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            if value == target {
                position = Some((i, j));
                break 'search;
            }
        }
    }
    
    println!("Matrix:");
    for row in &matrix {
        println!("  {:?}", row);
    }
    
    match position {
        Some((i, j)) => println!("Found {} at position ({}, {})", target, i, j),
        None => println!("{} not found", target),
    }
    
    println!("\nExample 6: Pattern matching in strings");
    let logs = vec![
        "INFO: Application started",
        "DEBUG: Loading config",
        "ERROR: File not found",
        "INFO: Processing data",
        "ERROR: Network timeout",
    ];
    
    println!("Searching for errors:");
    for (i, log) in logs.iter().enumerate() {
        if log.contains("ERROR") {
            println!("  Line {}: {}", i + 1, log);
        }
    }
}

// 3. Filtering and Transformation
fn filtering_and_transformation() {
    println!("\n=== Filtering & Transformation Patterns ===\n");
    
    println!("Example 1: Filter and collect");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let evens: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)
        .copied()
        .collect();
    
    println!("Numbers: {:?}", numbers);
    println!("Evens:   {:?}", evens);
    
    println!("\nExample 2: Transform and collect");
    let names = vec!["alice", "bob", "charlie"];
    let capitalized: Vec<String> = names.iter()
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();
    
    println!("Original:    {:?}", names);
    println!("Capitalized: {:?}", capitalized);
    
    println!("\nExample 3: Filter-map pattern");
    let strings = vec!["1", "two", "3", "four", "5", "six", "7"];
    let numbers: Vec<i32> = strings.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    
    println!("Strings: {:?}", strings);
    println!("Numbers: {:?}", numbers);
    
    println!("\nExample 4: Partition into two groups");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers.iter()
        .copied()
        .partition(|&x| x % 2 == 0);
    
    println!("Numbers: {:?}", numbers);
    println!("Evens:   {:?}", evens);
    println!("Odds:    {:?}", odds);
    
    println!("\nExample 5: Chained transformations");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let result: Vec<i32> = numbers.iter()
        .filter(|&&x| x % 2 == 0)  // Get evens
        .map(|x| x * x)             // Square them
        .filter(|&x| x > 10)        // Keep only > 10
        .collect();
    
    println!("Numbers: {:?}", numbers);
    println!("Result:  {:?}", result);
    println!("Process: even → square → filter(>10)");
    
    println!("\nExample 6: Remove duplicates");
    use std::collections::HashSet;
    
    let numbers = vec![1, 2, 2, 3, 4, 4, 5, 1, 3];
    let unique: Vec<i32> = numbers.iter()
        .copied()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    
    println!("Original: {:?}", numbers);
    println!("Unique:   {:?}", unique);
}

// 4. Nested Iteration Patterns
fn nested_iteration_patterns() {
    println!("\n=== Nested Iteration Patterns ===\n");
    
    println!("Example 1: Multiplication table");
    println!("     1   2   3   4   5   6   7   8   9  10");
    println!("   ----------------------------------------");
    
    for i in 1..=10 {
        print!("{:2} |", i);
        for j in 1..=10 {
            print!("{:3} ", i * j);
        }
        println!();
    }
    
    println!("\nExample 2: Matrix operations");
    let matrix_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    
    let matrix_b = vec![
        vec![7, 8, 9],
        vec![10, 11, 12],
    ];
    
    println!("Matrix A:");
    for row in &matrix_a {
        println!("  {:?}", row);
    }
    
    println!("\nMatrix B:");
    for row in &matrix_b {
        println!("  {:?}", row);
    }
    
    println!("\nMatrix A + B:");
    for i in 0..matrix_a.len() {
        let row: Vec<i32> = (0..matrix_a[0].len())
            .map(|j| matrix_a[i][j] + matrix_b[i][j])
            .collect();
        println!("  {:?}", row);
    }
    
    println!("\nExample 3: Finding pairs");
    let numbers = vec![1, 2, 3, 4, 5];
    println!("All pairs:");
    
    for i in 0..numbers.len() {
        for j in (i + 1)..numbers.len() {
            println!("  ({}, {})", numbers[i], numbers[j]);
        }
    }
    
    println!("\nExample 4: Cartesian product");
    let colors = vec!["Red", "Green", "Blue"];
    let sizes = vec!["S", "M", "L"];
    
    println!("Product combinations:");
    for color in &colors {
        for size in &sizes {
            println!("  {} - {}", color, size);
        }
    }
    
    println!("\nExample 5: Triangle patterns");
    println!("Right triangle:");
    for i in 1..=5 {
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }
    
    println!("\nInverted triangle:");
    for i in (1..=5).rev() {
        for _ in 0..i {
            print!("* ");
        }
        println!();
    }
    
    println!("\nPyramid:");
    for i in 1..=5 {
        for _ in 0..(5 - i) {
            print!(" ");
        }
        for _ in 0..(2 * i - 1) {
            print!("*");
        }
        println!();
    }
}

// 5. Sliding Window
fn sliding_window() {
    println!("\n=== Sliding Window Pattern ===\n");
    
    println!("Example 1: Fixed-size window sum");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let window_size = 3;
    
    println!("Numbers: {:?}", numbers);
    println!("\nWindow sums (size {}):", window_size);
    
    for window in numbers.windows(window_size) {
        let sum: i32 = window.iter().sum();
        println!("  {:?} -> sum: {}", window, sum);
    }
    
    println!("\nExample 2: Maximum in sliding window");
    let numbers = vec![1, 3, 2, 5, 8, 1, 7, 4, 6];
    let window_size = 3;
    
    println!("Numbers: {:?}", numbers);
    println!("\nMaximum in windows (size {}):", window_size);
    
    for (i, window) in numbers.windows(window_size).enumerate() {
        let max = window.iter().max().unwrap();
        println!("  Window {} {:?} -> max: {}", i, window, max);
    }
    
    println!("\nExample 3: Average in sliding window");
    let temperatures = vec![20, 22, 21, 23, 25, 24, 22, 20, 19];
    let window_size = 3;
    
    println!("Temperatures: {:?}", temperatures);
    println!("\nMoving average (window {}):", window_size);
    
    for window in temperatures.windows(window_size) {
        let avg = window.iter().sum::<i32>() as f64 / window_size as f64;
        println!("  {:?} -> avg: {:.2}", window, avg);
    }
    
    println!("\nExample 4: Consecutive sequence detection");
    let numbers = vec![1, 2, 3, 5, 6, 7, 8, 10, 11];
    
    println!("Numbers: {:?}", numbers);
    println!("\nConsecutive sequences (length 3+):");
    
    let mut in_sequence = false;
    for window in numbers.windows(2) {
        if window[1] == window[0] + 1 {
            if !in_sequence {
                print!("  [");
                in_sequence = true;
            }
            print!("{}, ", window[0]);
        } else {
            if in_sequence {
                println!("{}]", window[0]);
                in_sequence = false;
            }
        }
    }
    
    println!("\nExample 5: Substring search");
    let text = "hello world, hello rust";
    let pattern = "llo";
    
    println!("Text: '{}'", text);
    println!("Pattern: '{}'", pattern);
    println!("Matches:");
    
    let text_bytes = text.as_bytes();
    let pattern_bytes = pattern.as_bytes();
    
    for (i, window) in text_bytes.windows(pattern_bytes.len()).enumerate() {
        if window == pattern_bytes {
            println!("  Found at position {}", i);
        }
    }
}

// 6. Two Pointer Technique
fn two_pointer_technique() {
    println!("\n=== Two Pointer Technique ===\n");
    
    println!("Example 1: Reverse array in-place");
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    println!("Original: {:?}", numbers);
    
    let mut left = 0;
    let mut right = numbers.len() - 1;
    
    while left < right {
        numbers.swap(left, right);
        left += 1;
        right -= 1;
    }
    
    println!("Reversed: {:?}", numbers);
    
    println!("\nExample 2: Find pair with target sum");
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let target = 10;
    
    println!("Numbers: {:?}", numbers);
    println!("Target sum: {}", target);
    
    let mut left = 0;
    let mut right = numbers.len() - 1;
    let mut found = false;
    
    while left < right {
        let sum = numbers[left] + numbers[right];
        if sum == target {
            println!("Found pair: {} + {} = {}", numbers[left], numbers[right], target);
            found = true;
            left += 1;
            right -= 1;
        } else if sum < target {
            left += 1;
        } else {
            right -= 1;
        }
    }
    
    if !found {
        println!("No pair found");
    }
    
    println!("\nExample 3: Remove duplicates from sorted array");
    let mut numbers = vec![1, 1, 2, 2, 2, 3, 4, 4, 5];
    println!("Original: {:?}", numbers);
    
    if !numbers.is_empty() {
        let mut write_pos = 1;
        
        for read_pos in 1..numbers.len() {
            if numbers[read_pos] != numbers[read_pos - 1] {
                numbers[write_pos] = numbers[read_pos];
                write_pos += 1;
            }
        }
        
        numbers.truncate(write_pos);
    }
    
    println!("Unique:   {:?}", numbers);
    
    println!("\nExample 4: Merge two sorted arrays");
    let arr1 = vec![1, 3, 5, 7];
    let arr2 = vec![2, 4, 6, 8];
    
    println!("Array 1: {:?}", arr1);
    println!("Array 2: {:?}", arr2);
    
    let mut merged = Vec::new();
    let mut i = 0;
    let mut j = 0;
    
    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            merged.push(arr1[i]);
            i += 1;
        } else {
            merged.push(arr2[j]);
            j += 1;
        }
    }
    
    merged.extend_from_slice(&arr1[i..]);
    merged.extend_from_slice(&arr2[j..]);
    
    println!("Merged:  {:?}", merged);
    
    println!("\nExample 5: Palindrome checker");
    let test_strings = vec!["racecar", "hello", "level", "world", "madam"];
    
    println!("Checking palindromes:");
    for s in test_strings {
        let chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;
        let mut is_palindrome = true;
        
        while left < right {
            if chars[left] != chars[right] {
                is_palindrome = false;
                break;
            }
            left += 1;
            right -= 1;
        }
        
        println!("  '{}' -> {}", s, if is_palindrome { "YES" } else { "NO" });
    }
}

// 7. Retry and Backoff
fn retry_and_backoff() {
    println!("\n=== Retry & Backoff Pattern ===\n");
    
    println!("Example 1: Simple retry logic");
    let max_attempts = 3;
    let mut attempts = 0;
    let mut success = false;
    
    while attempts < max_attempts && !success {
        attempts += 1;
        println!("Attempt {}/{}", attempts, max_attempts);
        
        // Simulate operation (succeeds on attempt 2)
        success = attempts == 2;
        
        if success {
            println!("✓ Success!");
        } else if attempts < max_attempts {
            println!("✗ Failed, retrying...");
        }
    }
    
    if !success {
        println!("✗ Failed after {} attempts", max_attempts);
    }
    
    println!("\nExample 2: Exponential backoff");
    let max_attempts = 5;
    let base_delay = 100; // milliseconds
    
    println!("Simulating exponential backoff:");
    for attempt in 1..=max_attempts {
        let delay = base_delay * 2_u32.pow(attempt - 1);
        println!("Attempt {}: wait {}ms", attempt, delay);
        
        // In real code, you would sleep here
        // std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    
    println!("\nExample 3: Retry with jitter");
    println!("Simulating retry with jitter:");
    use std::collections::hash_map::RandomState;
    use std::hash::{BuildHasher, Hash, Hasher};
    
    for attempt in 1..=5 {
        let base_delay = 100 * 2_u32.pow(attempt - 1);
        
        // Simple pseudo-random jitter
        let mut hasher = RandomState::new().build_hasher();
        attempt.hash(&mut hasher);
        let jitter = (hasher.finish() % 50) as u32;
        
        let delay = base_delay + jitter;
        println!("Attempt {}: wait {}ms ({}ms + {}ms jitter)", 
                 attempt, delay, base_delay, jitter);
    }
    
    println!("\nExample 4: Circuit breaker pattern");
    let mut failures = 0;
    let failure_threshold = 3;
    let mut circuit_open = false;
    
    let operations = vec![false, false, false, true, false, true];
    
    println!("Circuit breaker simulation:");
    for (i, &should_succeed) in operations.iter().enumerate() {
        if circuit_open {
            println!("Operation {}: ⊘ Circuit OPEN, skipping", i + 1);
            continue;
        }
        
        println!("Operation {}: Attempting...", i + 1);
        
        if should_succeed {
            println!("  ✓ Success! Resetting failure count");
            failures = 0;
        } else {
            failures += 1;
            println!("  ✗ Failed ({}/{})", failures, failure_threshold);
            
            if failures >= failure_threshold {
                circuit_open = true;
                println!("  ⚠ Circuit OPENED!");
            }
        }
    }
}

// 8. Event Loop Simulation
fn event_loop_simulation() {
    println!("\n=== Event Loop Simulation ===\n");
    
    println!("Example 1: Simple event queue");
    let mut events = vec!["click", "hover", "click", "scroll", "resize"];
    
    println!("Processing events:");
    while let Some(event) = events.pop() {
        println!("  Handling: {}", event);
        
        // Simulate event processing
        match event {
            "click" => println!("    → Button clicked"),
            "hover" => println!("    → Mouse hover detected"),
            "scroll" => println!("    → Page scrolled"),
            "resize" => println!("    → Window resized"),
            _ => println!("    → Unknown event"),
        }
    }
    
    println!("\nExample 2: Message queue with priority");
    #[derive(Debug)]
    struct Message {
        priority: u8,
        content: String,
    }
    
    let mut messages = vec![
        Message { priority: 2, content: "Normal task".to_string() },
        Message { priority: 1, content: "High priority!".to_string() },
        Message { priority: 3, content: "Low priority".to_string() },
        Message { priority: 1, content: "Another urgent".to_string() },
        Message { priority: 2, content: "Regular work".to_string() },
    ];
    
    // Sort by priority (1 = highest)
    messages.sort_by_key(|m| m.priority);
    
    println!("Processing messages by priority:");
    for (i, msg) in messages.iter().enumerate() {
        println!("  {}. [P{}] {}", i + 1, msg.priority, msg.content);
    }
    
    println!("\nExample 3: Task scheduler simulation");
    #[derive(Debug)]
    struct Task {
        id: u32,
        duration: u32,
    }
    
    let tasks = vec![
        Task { id: 1, duration: 2 },
        Task { id: 2, duration: 3 },
        Task { id: 3, duration: 1 },
        Task { id: 4, duration: 2 },
    ];
    
    println!("Executing tasks:");
    let mut time = 0;
    
    for task in tasks {
        println!("  Time {}: Starting task {} (duration: {})", 
                 time, task.id, task.duration);
        time += task.duration;
        println!("  Time {}: Completed task {}", time, task.id);
    }
    
    println!("\nTotal execution time: {}", time);
    
    println!("\nExample 4: State machine with loop");
    #[derive(Debug, PartialEq)]
    enum State {
        Idle,
        Processing,
        Complete,
        Error,
    }
    
    let mut state = State::Idle;
    let mut iterations = 0;
    let max_iterations = 10;
    
    println!("State machine simulation:");
    loop {
        println!("  State: {:?}", state);
        iterations += 1;
        
        state = match state {
            State::Idle => {
                println!("    → Starting process");
                State::Processing
            }
            State::Processing => {
                if iterations < 5 {
                    println!("    → Processing... ({}/{})", iterations, max_iterations);
                    State::Processing
                } else if iterations < 8 {
                    println!("    → Still processing...");
                    State::Processing
                } else {
                    println!("    → Processing complete");
                    State::Complete
                }
            }
            State::Complete => {
                println!("    → Done!");
                break;
            }
            State::Error => {
                println!("    → Error occurred");
                break;
            }
        };
        
        if iterations >= max_iterations {
            println!("    → Max iterations reached");
            state = State::Error;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_accumulation() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_searching() {
        let numbers = vec![1, 3, 5, 7, 9];
        let found = numbers.iter().find(|&&x| x == 5);
        assert_eq!(found, Some(&5));
    }
    
    #[test]
    fn test_filtering() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<i32> = numbers.iter()
            .filter(|&&x| x % 2 == 0)
            .copied()
            .collect();
        assert_eq!(evens, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_sliding_window() {
        let numbers = vec![1, 2, 3, 4, 5];
        let windows: Vec<Vec<i32>> = numbers.windows(3)
            .map(|w| w.to_vec())
            .collect();
        assert_eq!(windows.len(), 3);
        assert_eq!(windows[0], vec![1, 2, 3]);
    }
}
