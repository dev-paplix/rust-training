use std::io::{self, Write};
use std::collections::HashMap;

/// Demonstrates iterator methods and patterns in Rust
fn main() {
    loop {
        display_menu();
        
        let choice = get_user_choice();
        
        match choice {
            1 => demonstrate_iterator_basics(),
            2 => demonstrate_map_filter(),
            3 => demonstrate_consuming_adapters(),
            4 => demonstrate_iterator_chaining(),
            5 => demonstrate_collect_operations(),
            6 => demonstrate_advanced_iterators(),
            7 => demonstrate_custom_iteration(),
            8 => demonstrate_performance_comparison(),
            0 => {
                println!("\nThank you for exploring Rust iterators!");
                break;
            }
            _ => println!("\n❌ Invalid choice. Please try again."),
        }
        
        pause();
    }
}

fn display_menu() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║      Iterator Examples in Rust        ║");
    println!("╚════════════════════════════════════════╝");
    println!("1. Iterator Basics");
    println!("2. Map & Filter");
    println!("3. Consuming Adapters (sum, fold, reduce)");
    println!("4. Iterator Chaining");
    println!("5. Collect Operations");
    println!("6. Advanced Iterators");
    println!("7. Custom Iteration Patterns");
    println!("8. Performance Comparison");
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

// 1. Iterator Basics
fn demonstrate_iterator_basics() {
    println!("\n=== Iterator Basics ===\n");
    
    println!("Example 1: Creating iterators");
    let vec = vec![1, 2, 3, 4, 5];
    
    // iter() - borrows
    println!("Using iter() (borrows):");
    for val in vec.iter() {
        print!("{} ", val);
    }
    println!("\nVector still usable: {:?}", vec);
    
    // iter_mut() - mutable borrow
    let mut vec_mut = vec![1, 2, 3, 4, 5];
    println!("\nUsing iter_mut() (mutable borrow):");
    for val in vec_mut.iter_mut() {
        *val *= 2;
    }
    println!("Modified vector: {:?}", vec_mut);
    
    // into_iter() - takes ownership
    let vec_owned = vec![1, 2, 3, 4, 5];
    println!("\nUsing into_iter() (takes ownership):");
    for val in vec_owned.into_iter() {
        print!("{} ", val);
    }
    println!("\n(vec_owned no longer available)");
    
    println!("\nExample 2: Iterator methods");
    let numbers = vec![1, 2, 3, 4, 5];
    
    let mut iter = numbers.iter();
    println!("next(): {:?}", iter.next()); // Some(1)
    println!("next(): {:?}", iter.next()); // Some(2)
    println!("next(): {:?}", iter.next()); // Some(3)
    
    println!("\ncount(): {}", numbers.iter().count());
    println!("last(): {:?}", numbers.iter().last());
    println!("nth(2): {:?}", numbers.iter().nth(2));
    
    println!("\nExample 3: Range iterators");
    println!("Range 1..=10:");
    for i in 1..=10 {
        print!("{} ", i);
    }
    println!();
    
    println!("\nReverse range:");
    for i in (1..=10).rev() {
        print!("{} ", i);
    }
    println!();
    
    println!("\nStep by 2:");
    for i in (0..20).step_by(2) {
        print!("{} ", i);
    }
    println!();
}

// 2. Map & Filter
fn demonstrate_map_filter() {
    println!("\n=== Map & Filter ===\n");
    
    println!("Example 1: map - Transform elements");
    let numbers = vec![1, 2, 3, 4, 5];
    
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Original: {:?}", numbers);
    println!("Doubled:  {:?}", doubled);
    
    let squared: Vec<i32> = numbers.iter().map(|x| x * x).collect();
    println!("Squared:  {:?}", squared);
    
    println!("\nExample 2: map with different types");
    let words = vec!["hello", "world", "rust"];
    let lengths: Vec<usize> = words.iter().map(|s| s.len()).collect();
    println!("Words:   {:?}", words);
    println!("Lengths: {:?}", lengths);
    
    let uppercase: Vec<String> = words.iter().map(|s| s.to_uppercase()).collect();
    println!("Upper:   {:?}", uppercase);
    
    println!("\nExample 3: filter - Select elements");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("All numbers: {:?}", numbers);
    println!("Evens:       {:?}", evens);
    
    let odds: Vec<i32> = numbers.iter().filter(|&&x| x % 2 != 0).copied().collect();
    println!("Odds:        {:?}", odds);
    
    let greater_than_5: Vec<i32> = numbers.iter().filter(|&&x| x > 5).copied().collect();
    println!("Greater > 5: {:?}", greater_than_5);
    
    println!("\nExample 4: Combining map and filter");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Double even numbers
    let doubled_evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * 2)
        .collect();
    println!("Doubled evens: {:?}", doubled_evens);
    
    // Square numbers > 5
    let squared_large: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x > 5)
        .map(|x| x * x)
        .collect();
    println!("Squared (>5):  {:?}", squared_large);
    
    println!("\nExample 5: filter_map - Combine filter and map");
    let strings = vec!["1", "two", "3", "four", "5"];
    let parsed: Vec<i32> = strings
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    println!("Strings:      {:?}", strings);
    println!("Parsed nums:  {:?}", parsed);
}

// 3. Consuming Adapters
fn demonstrate_consuming_adapters() {
    println!("\n=== Consuming Adapters ===\n");
    
    println!("Example 1: sum and product");
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum: i32 = numbers.iter().sum();
    println!("Numbers: {:?}", numbers);
    println!("Sum:     {}", sum);
    
    let product: i32 = numbers.iter().product();
    println!("Product: {}", product);
    
    println!("\nExample 2: fold - General accumulation");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // Sum using fold
    let sum = numbers.iter().fold(0, |acc, x| acc + x);
    println!("Sum (fold):     {}", sum);
    
    // Product using fold
    let product = numbers.iter().fold(1, |acc, x| acc * x);
    println!("Product (fold): {}", product);
    
    // Concatenate strings
    let words = vec!["Hello", " ", "World", "!"];
    let sentence = words.iter().fold(String::new(), |acc, s| acc + s);
    println!("Sentence: '{}'", sentence);
    
    println!("\nExample 3: reduce - Similar to fold");
    let numbers = vec![1, 2, 3, 4, 5];
    
    let sum = numbers.iter().reduce(|acc, x| acc + x);
    println!("Sum (reduce): {:?}", sum);
    
    let max = numbers.iter().reduce(|acc, x| if x > acc { x } else { acc });
    println!("Max (reduce): {:?}", max);
    
    let empty: Vec<i32> = vec![];
    let sum_empty = empty.iter().reduce(|acc, x| acc + x);
    println!("Empty reduce: {:?}", sum_empty);
    
    println!("\nExample 4: any and all");
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("Has even? {}", numbers.iter().any(|&x| x % 2 == 0));
    println!("All positive? {}", numbers.iter().all(|&x| x > 0));
    println!("All even? {}", numbers.iter().all(|&x| x % 2 == 0));
    println!("Has > 10? {}", numbers.iter().any(|&x| x > 10));
    
    println!("\nExample 5: find and position");
    let numbers = vec![1, 3, 5, 8, 9, 11];
    
    let first_even = numbers.iter().find(|&&x| x % 2 == 0);
    println!("First even: {:?}", first_even);
    
    let pos = numbers.iter().position(|&x| x % 2 == 0);
    println!("Position of first even: {:?}", pos);
    
    let not_found = numbers.iter().find(|&&x| x > 100);
    println!("Number > 100: {:?}", not_found);
    
    println!("\nExample 6: max, min, max_by, min_by");
    let numbers = vec![5, 2, 8, 1, 9, 3];
    
    println!("Numbers: {:?}", numbers);
    println!("Max: {:?}", numbers.iter().max());
    println!("Min: {:?}", numbers.iter().min());
    
    let words = vec!["apple", "banana", "kiwi", "watermelon"];
    let longest = words.iter().max_by_key(|s| s.len());
    let shortest = words.iter().min_by_key(|s| s.len());
    
    println!("\nWords: {:?}", words);
    println!("Longest: {:?}", longest);
    println!("Shortest: {:?}", shortest);
}

// 4. Iterator Chaining
fn demonstrate_iterator_chaining() {
    println!("\n=== Iterator Chaining ===\n");
    
    println!("Example 1: chain - Combine iterators");
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5, 6];
    let v3 = vec![7, 8, 9];
    
    let combined: Vec<i32> = v1.iter()
        .chain(v2.iter())
        .chain(v3.iter())
        .copied()
        .collect();
    
    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("v3: {:?}", v3);
    println!("Combined: {:?}", combined);
    
    println!("\nExample 2: zip - Pair elements");
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let cities = vec!["NYC", "LA", "Chicago"];
    
    println!("Names: {:?}", names);
    println!("Ages:  {:?}", ages);
    
    let people: Vec<_> = names.iter().zip(ages.iter()).collect();
    println!("Zipped: {:?}", people);
    
    for (name, age) in names.iter().zip(ages.iter()) {
        println!("{} is {} years old", name, age);
    }
    
    println!("\nTriple zip:");
    for ((name, age), city) in names.iter().zip(ages.iter()).zip(cities.iter()) {
        println!("{}, {} from {}", name, age, city);
    }
    
    println!("\nExample 3: enumerate - Add indices");
    let fruits = vec!["Apple", "Banana", "Orange"];
    
    for (i, fruit) in fruits.iter().enumerate() {
        println!("{}. {}", i + 1, fruit);
    }
    
    let indexed: Vec<_> = fruits.iter().enumerate().collect();
    println!("\nIndexed: {:?}", indexed);
    
    println!("\nExample 4: take and skip");
    let numbers: Vec<i32> = (1..=20).collect();
    
    let first_5: Vec<i32> = numbers.iter().take(5).copied().collect();
    println!("First 5: {:?}", first_5);
    
    let skip_10: Vec<i32> = numbers.iter().skip(10).copied().collect();
    println!("Skip 10: {:?}", skip_10);
    
    let middle: Vec<i32> = numbers.iter().skip(5).take(10).copied().collect();
    println!("Middle (skip 5, take 10): {:?}", middle);
    
    println!("\nExample 5: take_while and skip_while");
    let numbers = vec![1, 2, 3, 4, 5, 1, 2, 3];
    
    let take_result: Vec<i32> = numbers.iter().take_while(|&&x| x < 4).copied().collect();
    println!("Take while < 4: {:?}", take_result);
    
    let skip_result: Vec<i32> = numbers.iter().skip_while(|&&x| x < 4).copied().collect();
    println!("Skip while < 4: {:?}", skip_result);
    
    println!("\nExample 6: flat_map - Flatten and map");
    let words = vec!["hello", "world"];
    let chars: Vec<char> = words.iter().flat_map(|s| s.chars()).collect();
    println!("Words: {:?}", words);
    println!("Chars: {:?}", chars);
    
    let numbers = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
    let flattened: Vec<i32> = numbers.iter().flat_map(|v| v.iter()).copied().collect();
    println!("\nNested: {:?}", numbers);
    println!("Flat:   {:?}", flattened);
}

// 5. Collect Operations
fn demonstrate_collect_operations() {
    println!("\n=== Collect Operations ===\n");
    
    println!("Example 1: Collect to Vec");
    let squares: Vec<i32> = (1..=10).map(|x| x * x).collect();
    println!("Squares: {:?}", squares);
    
    println!("\nExample 2: Collect to String");
    let chars = vec!['H', 'e', 'l', 'l', 'o'];
    let word: String = chars.iter().collect();
    println!("Chars: {:?}", chars);
    println!("String: '{}'", word);
    
    let words = vec!["Hello", "World", "Rust"];
    let sentence = words.join(" ");
    println!("\nWords: {:?}", words);
    println!("Joined: '{}'", sentence);
    
    println!("\nExample 3: Collect to HashMap");
    let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
    let map: HashMap<&str, i32> = pairs.into_iter().collect();
    println!("HashMap: {:?}", map);
    
    let names = vec!["Alice", "Bob", "Charlie"];
    let ages = vec![25, 30, 35];
    let people: HashMap<_, _> = names.iter().zip(ages.iter()).collect();
    println!("\nPeople: {:?}", people);
    
    println!("\nExample 4: Collect to different types");
    use std::collections::{HashSet, BTreeSet};
    
    let numbers = vec![1, 2, 3, 2, 1, 4, 5, 4];
    println!("Original: {:?}", numbers);
    
    let vec: Vec<i32> = numbers.iter().copied().collect();
    println!("Vec:      {:?}", vec);
    
    let set: HashSet<i32> = numbers.iter().copied().collect();
    println!("HashSet:  {:?}", set);
    
    let btree: BTreeSet<i32> = numbers.iter().copied().collect();
    println!("BTreeSet: {:?}", btree);
    
    println!("\nExample 5: Partition - Split into two collections");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .iter()
        .copied()
        .partition(|&x| x % 2 == 0);
    
    println!("Numbers: {:?}", numbers);
    println!("Evens:   {:?}", evens);
    println!("Odds:    {:?}", odds);
    
    println!("\nExample 6: Group by using fold");
    let words = vec!["apple", "banana", "apricot", "blueberry", "cherry"];
    let grouped = words.iter().fold(HashMap::new(), |mut acc, word| {
        acc.entry(word.chars().next().unwrap())
            .or_insert(Vec::new())
            .push(*word);
        acc
    });
    
    println!("Words: {:?}", words);
    println!("\nGrouped by first letter:");
    for (letter, words) in grouped {
        println!("  {}: {:?}", letter, words);
    }
}

// 6. Advanced Iterators
fn demonstrate_advanced_iterators() {
    println!("\n=== Advanced Iterators ===\n");
    
    println!("Example 1: cycle - Repeat iterator");
    let colors = vec!["red", "green", "blue"];
    let cycled: Vec<&str> = colors.iter().cycle().take(10).copied().collect();
    println!("Colors: {:?}", colors);
    println!("Cycled: {:?}", cycled);
    
    println!("\nExample 2: repeat - Repeat single value");
    let repeated: Vec<i32> = std::iter::repeat(42).take(5).collect();
    println!("Repeated: {:?}", repeated);
    
    println!("\nExample 3: once - Single element iterator");
    let single: Vec<i32> = std::iter::once(100).collect();
    println!("Single: {:?}", single);
    
    let combined: Vec<i32> = vec![1, 2, 3].into_iter()
        .chain(std::iter::once(999))
        .collect();
    println!("Combined: {:?}", combined);
    
    println!("\nExample 4: scan - Stateful mapping");
    let numbers = vec![1, 2, 3, 4, 5];
    let running_sum: Vec<i32> = numbers
        .iter()
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .collect();
    
    println!("Numbers:     {:?}", numbers);
    println!("Running sum: {:?}", running_sum);
    
    println!("\nExample 5: inspect - Debug iterator chain");
    let result: Vec<i32> = (1..=5)
        .inspect(|x| println!("  Start: {}", x))
        .map(|x| x * 2)
        .inspect(|x| println!("  After map: {}", x))
        .filter(|&x| x > 5)
        .inspect(|x| println!("  After filter: {}", x))
        .collect();
    
    println!("Final result: {:?}", result);
    
    println!("\nExample 6: peekable - Look ahead");
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.iter().peekable();
    
    while let Some(&current) = iter.next() {
        match iter.peek() {
            Some(&&next) => println!("{} -> {}", current, next),
            None => println!("{} -> (end)", current),
        }
    }
}

// 7. Custom Iteration Patterns
fn demonstrate_custom_iteration() {
    println!("\n=== Custom Iteration Patterns ===\n");
    
    println!("Example 1: Fibonacci sequence");
    struct Fibonacci {
        curr: u64,
        next: u64,
    }
    
    impl Iterator for Fibonacci {
        type Item = u64;
        
        fn next(&mut self) -> Option<u64> {
            let new_next = self.curr + self.next;
            self.curr = self.next;
            self.next = new_next;
            Some(self.curr)
        }
    }
    
    fn fibonacci() -> Fibonacci {
        Fibonacci { curr: 0, next: 1 }
    }
    
    let fib: Vec<u64> = fibonacci().take(10).collect();
    println!("First 10 Fibonacci: {:?}", fib);
    
    println!("\nExample 2: Range with step");
    struct StepRange {
        current: i32,
        end: i32,
        step: i32,
    }
    
    impl Iterator for StepRange {
        type Item = i32;
        
        fn next(&mut self) -> Option<i32> {
            if self.current >= self.end {
                None
            } else {
                let result = self.current;
                self.current += self.step;
                Some(result)
            }
        }
    }
    
    let stepped: Vec<i32> = StepRange {
        current: 0,
        end: 20,
        step: 3,
    }.collect();
    
    println!("Step by 3 (0-20): {:?}", stepped);
    
    println!("\nExample 3: Window iterator");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7];
    println!("Numbers: {:?}", numbers);
    println!("\nWindows of size 3:");
    for window in numbers.windows(3) {
        println!("  {:?} -> sum: {}", window, window.iter().sum::<i32>());
    }
    
    println!("\nExample 4: Chunks iterator");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("Numbers: {:?}", numbers);
    println!("\nChunks of size 3:");
    for chunk in numbers.chunks(3) {
        println!("  {:?}", chunk);
    }
    
    println!("\nExample 5: Sliding window with fold");
    let numbers = vec![1, 2, 3, 4, 5];
    let pairs: Vec<(i32, i32)> = numbers
        .windows(2)
        .map(|w| (w[0], w[1]))
        .collect();
    
    println!("Numbers: {:?}", numbers);
    println!("Pairs:   {:?}", pairs);
}

// 8. Performance Comparison
fn demonstrate_performance_comparison() {
    println!("\n=== Performance Comparison ===\n");
    
    println!("Example 1: Iterator vs Index-based");
    let numbers: Vec<i32> = (0..1000).collect();
    
    // Using iterators (preferred)
    let sum1: i32 = numbers.iter().sum();
    println!("Sum using iterator: {}", sum1);
    
    // Using index (less idiomatic)
    let mut sum2 = 0;
    for i in 0..numbers.len() {
        sum2 += numbers[i];
    }
    println!("Sum using index:    {}", sum2);
    
    println!("\nIterators are often faster and always safer!");
    
    println!("\nExample 2: Lazy evaluation");
    let numbers = vec![1, 2, 3, 4, 5];
    
    println!("Creating lazy iterator chain:");
    let _iter = numbers.iter()
        .map(|x| {
            println!("  Mapping {}", x);
            x * 2
        })
        .filter(|x| {
            println!("  Filtering {}", x);
            x > &5
        });
    
    println!("Iterator created but not executed yet!");
    println!("\nNow collecting...");
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|x| x > &5)
        .collect();
    
    println!("Result: {:?}", result);
    
    println!("\nExample 3: Avoiding unnecessary allocations");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // Inefficient - multiple intermediate collections
    println!("Inefficient approach (avoid this):");
    let step1: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    let step2: Vec<i32> = step1.iter().filter(|&x| x > &10).copied().collect();
    let step3: Vec<i32> = step2.iter().map(|x| x + 1).collect();
    println!("Result: {:?}", step3);
    
    // Efficient - single chain
    println!("\nEfficient approach (use this):");
    let result: Vec<i32> = numbers.iter()
        .map(|x| x * 2)
        .filter(|&x| x > 10)
        .map(|x| x + 1)
        .collect();
    println!("Result: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_map() {
        let numbers = vec![1, 2, 3, 4, 5];
        let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6, 8, 10]);
    }
    
    #[test]
    fn test_filter() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let evens: Vec<i32> = numbers.iter().filter(|&&x| x % 2 == 0).copied().collect();
        assert_eq!(evens, vec![2, 4, 6]);
    }
    
    #[test]
    fn test_sum() {
        let numbers = vec![1, 2, 3, 4, 5];
        let sum: i32 = numbers.iter().sum();
        assert_eq!(sum, 15);
    }
    
    #[test]
    fn test_fold() {
        let numbers = vec![1, 2, 3, 4, 5];
        let product = numbers.iter().fold(1, |acc, x| acc * x);
        assert_eq!(product, 120);
    }
    
    #[test]
    fn test_chain() {
        let v1 = vec![1, 2, 3];
        let v2 = vec![4, 5, 6];
        let combined: Vec<i32> = v1.iter().chain(v2.iter()).copied().collect();
        assert_eq!(combined, vec![1, 2, 3, 4, 5, 6]);
    }
    
    #[test]
    fn test_any_all() {
        let numbers = vec![2, 4, 6, 8];
        assert!(numbers.iter().all(|&x| x % 2 == 0));
        assert!(numbers.iter().any(|&x| x > 5));
        assert!(!numbers.iter().any(|&x| x % 2 != 0));
    }
}
