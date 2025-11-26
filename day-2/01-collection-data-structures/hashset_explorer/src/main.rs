// HashSet Explorer
// Demonstrates HashSet<T> and set operations

use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    println!("=== HashSet Explorer ===\n");
    
    loop {
        println!("\nChoose a demonstration:");
        println!("1. Basic Set Operations");
        println!("2. Set Theory Operations");
        println!("3. Duplicate Remover");
        println!("4. Unique Visitor Tracker");
        println!("5. Tag Manager");
        println!("6. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => basic_operations(),
            2 => set_theory(),
            3 => duplicate_remover(),
            4 => visitor_tracker(),
            5 => tag_manager(),
            6 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn basic_operations() {
    println!("\n=== Basic Set Operations ===");
    let mut set: HashSet<i32> = HashSet::new();
    
    loop {
        println!("\nCurrent set: {:?}", set);
        println!("Size: {}", set.len());
        
        println!("\n1. Insert element");
        println!("2. Remove element");
        println!("3. Check membership");
        println!("4. Clear set");
        println!("5. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter number to insert: ");
                io::stdout().flush().unwrap();
                let num = read_number();
                
                if set.insert(num) {
                    println!("✅ Inserted {}", num);
                } else {
                    println!("ℹ️  {} already exists in set", num);
                }
            }
            2 => {
                print!("Enter number to remove: ");
                io::stdout().flush().unwrap();
                let num = read_number();
                
                if set.remove(&num) {
                    println!("✅ Removed {}", num);
                } else {
                    println!("❌ {} not found in set", num);
                }
            }
            3 => {
                print!("Enter number to check: ");
                io::stdout().flush().unwrap();
                let num = read_number();
                
                if set.contains(&num) {
                    println!("✅ {} is in the set", num);
                } else {
                    println!("❌ {} is NOT in the set", num);
                }
            }
            4 => {
                set.clear();
                println!("✅ Set cleared");
            }
            5 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn set_theory() {
    println!("\n=== Set Theory Operations ===");
    
    print!("\nEnter elements for Set A (comma-separated): ");
    io::stdout().flush().unwrap();
    let set_a = read_number_set();
    
    print!("Enter elements for Set B (comma-separated): ");
    io::stdout().flush().unwrap();
    let set_b = read_number_set();
    
    println!("\n📊 Sets:");
    println!("   A = {:?}", set_a);
    println!("   B = {:?}", set_b);
    
    // Union
    let union: HashSet<_> = set_a.union(&set_b).copied().collect();
    println!("\n🔗 Union (A ∪ B):");
    println!("   {:?}", union);
    println!("   All elements from both sets");
    
    // Intersection
    let intersection: HashSet<_> = set_a.intersection(&set_b).copied().collect();
    println!("\n🎯 Intersection (A ∩ B):");
    println!("   {:?}", intersection);
    println!("   Common elements");
    
    // Difference
    let diff_a_b: HashSet<_> = set_a.difference(&set_b).copied().collect();
    println!("\n➖ Difference (A - B):");
    println!("   {:?}", diff_a_b);
    println!("   Elements in A but not in B");
    
    let diff_b_a: HashSet<_> = set_b.difference(&set_a).copied().collect();
    println!("\n➖ Difference (B - A):");
    println!("   {:?}", diff_b_a);
    println!("   Elements in B but not in A");
    
    // Symmetric Difference
    let sym_diff: HashSet<_> = set_a.symmetric_difference(&set_b).copied().collect();
    println!("\n🔄 Symmetric Difference (A △ B):");
    println!("   {:?}", sym_diff);
    println!("   Elements in A or B but not both");
    
    // Set Relations
    println!("\n📐 Set Relations:");
    println!("   A is subset of B: {}", set_a.is_subset(&set_b));
    println!("   A is superset of B: {}", set_a.is_superset(&set_b));
    println!("   A and B are disjoint: {}", set_a.is_disjoint(&set_b));
    
    // Venn Diagram ASCII
    println!("\n📊 Venn Diagram (conceptual):");
    println!("   Only in A: {:?}", diff_a_b);
    println!("   In both:   {:?}", intersection);
    println!("   Only in B: {:?}", diff_b_a);
}

fn duplicate_remover() {
    println!("\n=== Duplicate Remover ===");
    
    println!("\nChoose input method:");
    println!("1. Enter numbers");
    println!("2. Enter text/words");
    print!("\nChoice: ");
    io::stdout().flush().unwrap();
    
    let choice = read_number();
    
    match choice {
        1 => {
            print!("\nEnter numbers (comma-separated): ");
            io::stdout().flush().unwrap();
            let input = read_string();
            
            let numbers: Vec<i32> = input
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            
            println!("\n📝 Original list ({} items):", numbers.len());
            println!("   {:?}", numbers);
            
            // Remove duplicates (unordered)
            let unique_set: HashSet<_> = numbers.iter().copied().collect();
            let unique_vec: Vec<_> = unique_set.into_iter().collect();
            
            println!("\n🎯 Unique elements ({} items):", unique_vec.len());
            println!("   {:?}", unique_vec);
            
            // Remove duplicates preserving order
            let mut seen = HashSet::new();
            let unique_ordered: Vec<_> = numbers
                .into_iter()
                .filter(|x| seen.insert(*x))
                .collect();
            
            println!("\n📑 Unique (order preserved):");
            println!("   {:?}", unique_ordered);
        }
        2 => {
            print!("\nEnter text: ");
            io::stdout().flush().unwrap();
            let text = read_string();
            
            let words: Vec<&str> = text.split_whitespace().collect();
            
            println!("\n📝 Original words ({} words):", words.len());
            println!("   {:?}", words);
            
            // Unique words
            let unique_words: HashSet<_> = words.iter().copied().collect();
            
            println!("\n🎯 Unique words ({} words):", unique_words.len());
            println!("   {:?}", unique_words);
            
            // Case-insensitive unique
            let mut case_insensitive: HashSet<String> = HashSet::new();
            for word in &words {
                case_insensitive.insert(word.to_lowercase());
            }
            
            println!("\n🎯 Unique (case-insensitive) ({} words):", case_insensitive.len());
            println!("   {:?}", case_insensitive);
        }
        _ => println!("Invalid choice!"),
    }
}

fn visitor_tracker() {
    println!("\n=== Unique Visitor Tracker ===");
    
    let mut visitors: HashSet<String> = HashSet::new();
    let mut visit_log: Vec<String> = Vec::new();
    
    loop {
        println!("\n📊 Statistics:");
        println!("   Total visits: {}", visit_log.len());
        println!("   Unique visitors: {}", visitors.len());
        if !visit_log.is_empty() {
            let return_rate = 100.0 * (visit_log.len() - visitors.len()) as f64 / visit_log.len() as f64;
            println!("   Return visitor rate: {:.1}%", return_rate);
        }
        
        println!("\n1. Record visit");
        println!("2. Check if visitor exists");
        println!("3. View all visitors");
        println!("4. View visit log");
        println!("5. Reset");
        println!("6. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter visitor ID/name: ");
                io::stdout().flush().unwrap();
                let visitor = read_string();
                
                visit_log.push(visitor.clone());
                
                if visitors.insert(visitor.clone()) {
                    println!("✅ New visitor: {}", visitor);
                } else {
                    println!("🔄 Return visitor: {}", visitor);
                }
            }
            2 => {
                print!("Enter visitor ID/name: ");
                io::stdout().flush().unwrap();
                let visitor = read_string();
                
                if visitors.contains(&visitor) {
                    let count = visit_log.iter().filter(|v| *v == &visitor).count();
                    println!("✅ Visitor exists ({} visits)", count);
                } else {
                    println!("❌ Visitor not found");
                }
            }
            3 => {
                if visitors.is_empty() {
                    println!("\n❌ No visitors yet");
                } else {
                    println!("\n👥 Unique Visitors ({}):", visitors.len());
                    let mut sorted: Vec<_> = visitors.iter().collect();
                    sorted.sort();
                    for visitor in sorted {
                        let count = visit_log.iter().filter(|v| *v == visitor).count();
                        println!("   {} ({} visits)", visitor, count);
                    }
                }
            }
            4 => {
                if visit_log.is_empty() {
                    println!("\n❌ No visits recorded");
                } else {
                    println!("\n📋 Visit Log ({} visits):", visit_log.len());
                    for (i, visitor) in visit_log.iter().enumerate() {
                        println!("   {}. {}", i + 1, visitor);
                    }
                }
            }
            5 => {
                visitors.clear();
                visit_log.clear();
                println!("✅ Reset complete");
            }
            6 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn tag_manager() {
    println!("\n=== Tag Manager ===");
    
    let mut item_tags: std::collections::HashMap<String, HashSet<String>> = 
        std::collections::HashMap::new();
    
    loop {
        println!("\n1. Add item");
        println!("2. Add tag to item");
        println!("3. Remove tag from item");
        println!("4. View item tags");
        println!("5. Find items with tag");
        println!("6. Find items with all tags");
        println!("7. View all tags");
        println!("8. Back");
        print!("\nChoice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => {
                print!("Enter item name: ");
                io::stdout().flush().unwrap();
                let item = read_string();
                
                item_tags.entry(item.clone()).or_insert_with(HashSet::new);
                println!("✅ Added item: {}", item);
            }
            2 => {
                print!("Enter item name: ");
                io::stdout().flush().unwrap();
                let item = read_string();
                
                print!("Enter tag: ");
                io::stdout().flush().unwrap();
                let tag = read_string();
                
                item_tags.entry(item.clone())
                    .or_insert_with(HashSet::new)
                    .insert(tag.clone());
                
                println!("✅ Added tag '{}' to '{}'", tag, item);
            }
            3 => {
                print!("Enter item name: ");
                io::stdout().flush().unwrap();
                let item = read_string();
                
                if let Some(tags) = item_tags.get_mut(&item) {
                    print!("Enter tag to remove: ");
                    io::stdout().flush().unwrap();
                    let tag = read_string();
                    
                    if tags.remove(&tag) {
                        println!("✅ Removed tag '{}' from '{}'", tag, item);
                    } else {
                        println!("❌ Tag not found");
                    }
                } else {
                    println!("❌ Item not found");
                }
            }
            4 => {
                print!("Enter item name: ");
                io::stdout().flush().unwrap();
                let item = read_string();
                
                if let Some(tags) = item_tags.get(&item) {
                    if tags.is_empty() {
                        println!("\n🏷️  '{}' has no tags", item);
                    } else {
                        println!("\n🏷️  Tags for '{}' ({}):", item, tags.len());
                        for tag in tags {
                            println!("   - {}", tag);
                        }
                    }
                } else {
                    println!("❌ Item not found");
                }
            }
            5 => {
                print!("Enter tag: ");
                io::stdout().flush().unwrap();
                let search_tag = read_string();
                
                let items: Vec<_> = item_tags.iter()
                    .filter(|(_, tags)| tags.contains(&search_tag))
                    .map(|(item, _)| item)
                    .collect();
                
                if items.is_empty() {
                    println!("\n❌ No items with tag '{}'", search_tag);
                } else {
                    println!("\n🔍 Items with tag '{}' ({}):", search_tag, items.len());
                    for item in items {
                        println!("   - {}", item);
                    }
                }
            }
            6 => {
                print!("Enter tags (comma-separated): ");
                io::stdout().flush().unwrap();
                let input = read_string();
                let search_tags: HashSet<String> = input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect();
                
                let items: Vec<_> = item_tags.iter()
                    .filter(|(_, tags)| search_tags.is_subset(tags))
                    .map(|(item, _)| item)
                    .collect();
                
                if items.is_empty() {
                    println!("\n❌ No items with all tags: {:?}", search_tags);
                } else {
                    println!("\n🔍 Items with all tags ({}):", items.len());
                    for item in items {
                        println!("   - {}", item);
                    }
                }
            }
            7 => {
                let mut all_tags = HashSet::new();
                for tags in item_tags.values() {
                    all_tags.extend(tags.iter().cloned());
                }
                
                if all_tags.is_empty() {
                    println!("\n❌ No tags in system");
                } else {
                    println!("\n🏷️  All tags ({}):", all_tags.len());
                    let mut sorted: Vec<_> = all_tags.iter().collect();
                    sorted.sort();
                    for tag in sorted {
                        let count = item_tags.values()
                            .filter(|tags| tags.contains(tag))
                            .count();
                        println!("   - {} ({} items)", tag, count);
                    }
                }
            }
            8 => break,
            _ => println!("Invalid choice!"),
        }
    }
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().unwrap_or(0)
}

fn read_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn read_number_set() -> HashSet<i32> {
    let input = read_string();
    input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_set_operations() {
        let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
        let set2: HashSet<_> = vec![3, 4, 5].into_iter().collect();
        
        let union: HashSet<_> = set1.union(&set2).copied().collect();
        assert_eq!(union.len(), 5);
        
        let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
        assert_eq!(intersection, vec![3].into_iter().collect());
    }
    
    #[test]
    fn test_duplicate_removal() {
        let nums = vec![1, 2, 2, 3, 3, 3];
        let unique: HashSet<_> = nums.into_iter().collect();
        assert_eq!(unique.len(), 3);
    }
    
    #[test]
    fn test_membership() {
        let mut set = HashSet::new();
        set.insert(1);
        assert!(set.contains(&1));
        assert!(!set.contains(&2));
    }
}
