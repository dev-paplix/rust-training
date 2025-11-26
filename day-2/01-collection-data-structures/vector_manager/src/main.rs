// Vector Manager
// Demonstrates comprehensive Vec<T> operations

use std::io::{self, Write};

fn main() {
    println!("=== Vector Manager ===\n");
    
    let mut numbers: Vec<i32> = Vec::new();
    
    loop {
        display_vector(&numbers);
        
        println!("\nOperations:");
        println!("1.  Add element (push)");
        println!("2.  Remove last element (pop)");
        println!("3.  Insert at position");
        println!("4.  Remove at position");
        println!("5.  Access element");
        println!("6.  Sort vector");
        println!("7.  Reverse vector");
        println!("8.  Search element");
        println!("9.  Filter elements");
        println!("10. Map (transform) elements");
        println!("11. Statistics");
        println!("12. Capacity info");
        println!("13. Clear all");
        println!("14. Fill with range");
        println!("15. Exit");
        print!("\nYour choice: ");
        io::stdout().flush().unwrap();
        
        let choice = read_number();
        
        match choice {
            1 => add_element(&mut numbers),
            2 => remove_last(&mut numbers),
            3 => insert_at_position(&mut numbers),
            4 => remove_at_position(&mut numbers),
            5 => access_element(&numbers),
            6 => sort_vector(&mut numbers),
            7 => reverse_vector(&mut numbers),
            8 => search_element(&numbers),
            9 => filter_elements(&mut numbers),
            10 => map_elements(&mut numbers),
            11 => show_statistics(&numbers),
            12 => show_capacity(&numbers),
            13 => clear_vector(&mut numbers),
            14 => fill_with_range(&mut numbers),
            15 => {
                println!("\nGoodbye!");
                break;
            }
            _ => println!("Invalid choice!"),
        }
    }
}

fn display_vector(v: &Vec<i32>) {
    println!("\n┌─────────────────────────────────────┐");
    println!("│ Current Vector:                     │");
    println!("├─────────────────────────────────────┤");
    if v.is_empty() {
        println!("│ [Empty]                             │");
    } else {
        print!("│ ");
        for (i, &num) in v.iter().enumerate() {
            print!("{}", num);
            if i < v.len() - 1 {
                print!(", ");
            }
        }
        println!();
        println!("│ Length: {}, Capacity: {}            ", v.len(), v.capacity());
    }
    println!("└─────────────────────────────────────┘");
}

fn add_element(v: &mut Vec<i32>) {
    print!("\nEnter number to add: ");
    io::stdout().flush().unwrap();
    let num = read_number();
    v.push(num);
    println!("✅ Added {} to the end", num);
}

fn remove_last(v: &mut Vec<i32>) {
    if let Some(num) = v.pop() {
        println!("✅ Removed {} from the end", num);
    } else {
        println!("❌ Vector is empty!");
    }
}

fn insert_at_position(v: &mut Vec<i32>) {
    print!("\nEnter position (0 to {}): ", v.len());
    io::stdout().flush().unwrap();
    let pos = read_number() as usize;
    
    if pos > v.len() {
        println!("❌ Invalid position!");
        return;
    }
    
    print!("Enter number to insert: ");
    io::stdout().flush().unwrap();
    let num = read_number();
    
    v.insert(pos, num);
    println!("✅ Inserted {} at position {}", num, pos);
}

fn remove_at_position(v: &mut Vec<i32>) {
    if v.is_empty() {
        println!("❌ Vector is empty!");
        return;
    }
    
    print!("\nEnter position (0 to {}): ", v.len() - 1);
    io::stdout().flush().unwrap();
    let pos = read_number() as usize;
    
    if pos >= v.len() {
        println!("❌ Invalid position!");
        return;
    }
    
    let removed = v.remove(pos);
    println!("✅ Removed {} from position {}", removed, pos);
}

fn access_element(v: &Vec<i32>) {
    if v.is_empty() {
        println!("❌ Vector is empty!");
        return;
    }
    
    print!("\nEnter index (0 to {}): ", v.len() - 1);
    io::stdout().flush().unwrap();
    let index = read_number() as usize;
    
    match v.get(index) {
        Some(value) => println!("📍 Element at index {}: {}", index, value),
        None => println!("❌ Index out of bounds!"),
    }
}

fn sort_vector(v: &mut Vec<i32>) {
    println!("\n1. Sort ascending");
    println!("2. Sort descending");
    print!("Choice: ");
    io::stdout().flush().unwrap();
    let choice = read_number();
    
    match choice {
        1 => {
            v.sort();
            println!("✅ Sorted in ascending order");
        }
        2 => {
            v.sort_by(|a, b| b.cmp(a));
            println!("✅ Sorted in descending order");
        }
        _ => println!("Invalid choice!"),
    }
}

fn reverse_vector(v: &mut Vec<i32>) {
    v.reverse();
    println!("✅ Vector reversed");
}

fn search_element(v: &Vec<i32>) {
    print!("\nEnter number to search: ");
    io::stdout().flush().unwrap();
    let target = read_number();
    
    // Linear search
    if let Some(pos) = v.iter().position(|&x| x == target) {
        println!("✅ Found {} at index {}", target, pos);
        
        // Count occurrences
        let count = v.iter().filter(|&&x| x == target).count();
        println!("   Appears {} time(s)", count);
    } else {
        println!("❌ {} not found in vector", target);
    }
    
    // Binary search (if sorted)
    let mut sorted = v.clone();
    sorted.sort();
    if sorted == *v {
        match v.binary_search(&target) {
            Ok(pos) => println!("   Binary search: found at position {}", pos),
            Err(pos) => println!("   Binary search: would insert at position {}", pos),
        }
    }
}

fn filter_elements(v: &mut Vec<i32>) {
    println!("\n1. Keep only even numbers");
    println!("2. Keep only odd numbers");
    println!("3. Keep only positive numbers");
    println!("4. Keep only negative numbers");
    println!("5. Keep numbers greater than X");
    println!("6. Keep numbers less than X");
    print!("Choice: ");
    io::stdout().flush().unwrap();
    let choice = read_number();
    
    let original_len = v.len();
    
    match choice {
        1 => {
            v.retain(|&x| x % 2 == 0);
            println!("✅ Kept only even numbers");
        }
        2 => {
            v.retain(|&x| x % 2 != 0);
            println!("✅ Kept only odd numbers");
        }
        3 => {
            v.retain(|&x| x > 0);
            println!("✅ Kept only positive numbers");
        }
        4 => {
            v.retain(|&x| x < 0);
            println!("✅ Kept only negative numbers");
        }
        5 => {
            print!("Enter threshold: ");
            io::stdout().flush().unwrap();
            let threshold = read_number();
            v.retain(|&x| x > threshold);
            println!("✅ Kept numbers > {}", threshold);
        }
        6 => {
            print!("Enter threshold: ");
            io::stdout().flush().unwrap();
            let threshold = read_number();
            v.retain(|&x| x < threshold);
            println!("✅ Kept numbers < {}", threshold);
        }
        _ => {
            println!("Invalid choice!");
            return;
        }
    }
    
    println!("   Removed {} elements", original_len - v.len());
}

fn map_elements(v: &mut Vec<i32>) {
    println!("\n1. Multiply all by 2");
    println!("2. Add 10 to all");
    println!("3. Square all");
    println!("4. Absolute value");
    println!("5. Negate all");
    print!("Choice: ");
    io::stdout().flush().unwrap();
    let choice = read_number();
    
    match choice {
        1 => {
            for x in v.iter_mut() {
                *x *= 2;
            }
            println!("✅ Multiplied all by 2");
        }
        2 => {
            for x in v.iter_mut() {
                *x += 10;
            }
            println!("✅ Added 10 to all");
        }
        3 => {
            for x in v.iter_mut() {
                *x = (*x) * (*x);
            }
            println!("✅ Squared all elements");
        }
        4 => {
            for x in v.iter_mut() {
                *x = x.abs();
            }
            println!("✅ Applied absolute value");
        }
        5 => {
            for x in v.iter_mut() {
                *x = -*x;
            }
            println!("✅ Negated all elements");
        }
        _ => println!("Invalid choice!"),
    }
}

fn show_statistics(v: &Vec<i32>) {
    if v.is_empty() {
        println!("\n❌ Vector is empty!");
        return;
    }
    
    let sum: i32 = v.iter().sum();
    let count = v.len();
    let avg = sum as f64 / count as f64;
    let min = v.iter().min().unwrap();
    let max = v.iter().max().unwrap();
    
    let even_count = v.iter().filter(|&&x| x % 2 == 0).count();
    let odd_count = count - even_count;
    let positive_count = v.iter().filter(|&&x| x > 0).count();
    let negative_count = v.iter().filter(|&&x| x < 0).count();
    let zero_count = v.iter().filter(|&&x| x == 0).count();
    
    println!("\n📊 Statistics:");
    println!("   Count:    {}", count);
    println!("   Sum:      {}", sum);
    println!("   Average:  {:.2}", avg);
    println!("   Min:      {}", min);
    println!("   Max:      {}", max);
    println!("   Range:    {}", max - min);
    println!("\n📈 Distribution:");
    println!("   Even:     {}", even_count);
    println!("   Odd:      {}", odd_count);
    println!("   Positive: {}", positive_count);
    println!("   Negative: {}", negative_count);
    println!("   Zero:     {}", zero_count);
}

fn show_capacity(v: &Vec<i32>) {
    println!("\n💾 Capacity Information:");
    println!("   Length:   {}", v.len());
    println!("   Capacity: {}", v.capacity());
    println!("   Unused:   {}", v.capacity() - v.len());
    println!("   Bytes:    ~{} (approx)", v.capacity() * std::mem::size_of::<i32>());
    
    if v.capacity() > v.len() {
        print!("\n   Shrink to fit? (y/n): ");
        io::stdout().flush().unwrap();
        if read_string().to_lowercase() == "y" {
            let old_cap = v.capacity();
            let mut temp = v.clone();
            temp.shrink_to_fit();
            *v = temp;
            println!("   ✅ Reduced capacity from {} to {}", old_cap, v.capacity());
        }
    }
}

fn clear_vector(v: &mut Vec<i32>) {
    if v.is_empty() {
        println!("\n❌ Vector is already empty!");
        return;
    }
    
    print!("\nAre you sure? (y/n): ");
    io::stdout().flush().unwrap();
    if read_string().to_lowercase() == "y" {
        let count = v.len();
        v.clear();
        println!("✅ Cleared {} elements", count);
    }
}

fn fill_with_range(v: &mut Vec<i32>) {
    print!("\nEnter start value: ");
    io::stdout().flush().unwrap();
    let start = read_number();
    
    print!("Enter end value (exclusive): ");
    io::stdout().flush().unwrap();
    let end = read_number();
    
    if end <= start {
        println!("❌ End must be greater than start!");
        return;
    }
    
    if end - start > 1000 {
        println!("❌ Range too large (max 1000)!");
        return;
    }
    
    v.clear();
    v.extend(start..end);
    println!("✅ Filled vector with range {}..{}", start, end);
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_vec_operations() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        
        let last = v.pop();
        assert_eq!(last, Some(4));
        assert_eq!(v.len(), 3);
    }
    
    #[test]
    fn test_vec_sorting() {
        let mut v = vec![3, 1, 4, 1, 5];
        v.sort();
        assert_eq!(v, vec![1, 1, 3, 4, 5]);
    }
    
    #[test]
    fn test_vec_statistics() {
        let v = vec![1, 2, 3, 4, 5];
        let sum: i32 = v.iter().sum();
        assert_eq!(sum, 15);
        
        let max = v.iter().max();
        assert_eq!(max, Some(&5));
    }
}
