# Rust Data Structures

## Table of Contents

- [Introduction](#introduction)
- [Learning Objectives](#learning-objectives)
- [Understanding Collections](#understanding-collections)
- [Vectors (Vec<T>)](#vectors-vect)
- [Strings and String Slices](#strings-and-string-slices)
- [Hash Maps (HashMap<K, V>)](#hash-maps-hashmapk-v)
- [Hash Sets (HashSet<T>)](#hash-sets-hashsett)
- [VecDeque (Double-Ended Queue)](#vecdeque-double-ended-queue)
- [BTreeMap and BTreeSet](#btreemap-and-btreeset)
- [Binary Heap](#binary-heap)
- [LinkedList](#linkedlist)
- [Tuples and Arrays (Review)](#tuples-and-arrays-review)
- [Structs (Custom Data Structures)](#structs-custom-data-structures)
- [Enums and Pattern Matching](#enums-and-pattern-matching)
- [Option and Result](#option-and-result)
- [Iterators and Iteration](#iterators-and-iteration)
- [Performance Considerations](#performance-considerations)
- [Practical Examples](#practical-examples)
- [Best Practices](#best-practices)
- [Exercises](#exercises)
- [Summary](#summary)

---

## Introduction

Data structures are fundamental to programming - they determine how we organize, store, and access data. Rust provides a rich set of built-in data structures through its standard library, each optimized for different use cases.

Understanding when and how to use each data structure is crucial for writing efficient, idiomatic Rust code. This lesson covers all major data structures in Rust, from basic collections to advanced structures.

**Key Characteristics of Rust Data Structures:**
- **Memory Safety**: Guaranteed at compile time
- **Performance**: Zero-cost abstractions
- **Ownership**: Clear ownership semantics
- **Type Safety**: Strong typing prevents errors

---

## Learning Objectives

By completing this lesson, you will be able to:

✅ Understand and use dynamic arrays with `Vec<T>`  
✅ Work with strings (`String` and `&str`)  
✅ Store key-value pairs with `HashMap<K, V>`  
✅ Use sets for unique value collections  
✅ Choose the right data structure for your use case  
✅ Work with advanced collections (VecDeque, BTreeMap, etc.)  
✅ Create custom data structures with structs  
✅ Use enums for algebraic data types  
✅ Master `Option` and `Result` for error handling  
✅ Apply iterators for efficient data processing  

---

## Understanding Collections

### What are Collections?

Collections are data structures that can hold multiple values. Unlike arrays, most collections:
- Store data on the **heap** (dynamic size)
- Can **grow or shrink** at runtime
- Are **owned** by the variable that creates them

### Common Collections in Rust

```rust
use std::collections::*;

// Vector - growable array
let vec: Vec<i32> = Vec::new();

// HashMap - key-value pairs
let map: HashMap<String, i32> = HashMap::new();

// HashSet - unique values
let set: HashSet<i32> = HashSet::new();

// VecDeque - double-ended queue
let deque: VecDeque<i32> = VecDeque::new();

// BTreeMap - sorted key-value pairs
let btree_map: BTreeMap<String, i32> = BTreeMap::new();

// BTreeSet - sorted unique values
let btree_set: BTreeSet<i32> = BTreeSet::new();

// BinaryHeap - priority queue
let heap: BinaryHeap<i32> = BinaryHeap::new();

// LinkedList - doubly-linked list
let list: LinkedList<i32> = LinkedList::new();
```

---

## Vectors (Vec<T>)

Vectors are the most common collection type - a growable array.

### Creating Vectors

```rust
// Empty vector with type annotation
let mut v1: Vec<i32> = Vec::new();

// Using vec! macro
let v2 = vec![1, 2, 3, 4, 5];

// With initial capacity
let mut v3 = Vec::with_capacity(10);

// From an iterator
let v4: Vec<i32> = (0..10).collect();
```

### Adding Elements

```rust
let mut v = Vec::new();

// Add to the end
v.push(1);
v.push(2);
v.push(3);

println!("{:?}", v);  // [1, 2, 3]

// Insert at specific position
v.insert(1, 10);
println!("{:?}", v);  // [1, 10, 2, 3]
```

### Accessing Elements

```rust
let v = vec![1, 2, 3, 4, 5];

// Using indexing (panics if out of bounds)
let third = v[2];
println!("Third element: {}", third);

// Using get method (returns Option)
match v.get(2) {
    Some(value) => println!("Third element: {}", value),
    None => println!("No third element"),
}

// Safe access
if let Some(value) = v.get(10) {
    println!("Element: {}", value);
} else {
    println!("Index out of bounds");
}
```

### Removing Elements

```rust
let mut v = vec![1, 2, 3, 4, 5];

// Remove last element
if let Some(last) = v.pop() {
    println!("Removed: {}", last);
}

// Remove at specific index
let removed = v.remove(1);
println!("Removed: {}", removed);
println!("{:?}", v);  // [1, 3, 4]

// Remove all elements
v.clear();
```

### Iterating Over Vectors

```rust
let v = vec![1, 2, 3, 4, 5];

// Immutable iteration
for value in &v {
    println!("{}", value);
}

// Mutable iteration
let mut v = vec![1, 2, 3];
for value in &mut v {
    *value *= 2;
}
println!("{:?}", v);  // [2, 4, 6]

// Consuming iteration
for value in v {
    println!("{}", value);
}
// v is no longer valid here
```

### Useful Vector Methods

```rust
let mut v = vec![1, 2, 3, 4, 5];

// Length
println!("Length: {}", v.len());

// Is empty?
println!("Is empty: {}", v.is_empty());

// Contains element?
println!("Contains 3: {}", v.contains(&3));

// First and last
if let Some(first) = v.first() {
    println!("First: {}", first);
}
if let Some(last) = v.last() {
    println!("Last: {}", last);
}

// Sorting
v.sort();
v.reverse();

// Deduplication
v.dedup();

// Slicing
let slice = &v[1..3];
println!("{:?}", slice);
```

### Vector of Different Types

```rust
// Using enums for multiple types
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("blue")),
];
```

---

## Strings and String Slices

Rust has two main string types: `String` (owned) and `&str` (borrowed slice).

### String vs &str

```rust
// String - owned, mutable, heap-allocated
let mut s1 = String::new();
let s2 = String::from("hello");
let s3 = "hello".to_string();

// &str - string slice, immutable, can point to heap or static memory
let s4: &str = "hello";  // String literal
let s5: &str = &s2[0..2];  // Slice of String
```

### Creating Strings

```rust
// Empty string
let mut s = String::new();

// From string literal
let s = String::from("Hello, world!");

// Using to_string()
let s = "Initial contents".to_string();

// From bytes
let s = String::from_utf8(vec![72, 101, 108, 108, 111]).unwrap();
```

### Updating Strings

```rust
let mut s = String::from("Hello");

// Append string slice
s.push_str(", world");

// Append single character
s.push('!');

println!("{}", s);  // Hello, world!

// Concatenation with +
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2;  // s1 is moved
// println!("{}", s1);  // Error: s1 moved

// Using format! macro
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");
let s = format!("{}-{}-{}", s1, s2, s3);  // Doesn't take ownership
```

### String Indexing

```rust
// ❌ This doesn't work in Rust!
// let s = String::from("hello");
// let h = s[0];  // Error!

// Use bytes, chars, or grapheme clusters

let s = String::from("hello");

// Bytes
for b in s.bytes() {
    println!("{}", b);
}

// Characters
for c in s.chars() {
    println!("{}", c);
}

// Slicing (be careful with UTF-8!)
let hello = "Здравствуйте";
let s = &hello[0..4];  // "Зд" (each Cyrillic char is 2 bytes)
```

### Useful String Methods

```rust
let s = String::from("  Hello, World!  ");

// Length
println!("Length: {}", s.len());

// Is empty?
println!("Is empty: {}", s.is_empty());

// Contains substring
println!("Contains 'World': {}", s.contains("World"));

// Starts with / ends with
println!("Starts with 'Hello': {}", s.starts_with("Hello"));

// Trim whitespace
let trimmed = s.trim();
println!("Trimmed: '{}'", trimmed);

// Split
for word in s.split(',') {
    println!("{}", word.trim());
}

// Replace
let replaced = s.replace("World", "Rust");
println!("{}", replaced);

// To uppercase/lowercase
println!("{}", s.to_uppercase());
println!("{}", s.to_lowercase());
```

---

## Hash Maps (HashMap<K, V>)

Hash maps store key-value pairs for fast lookups.

### Creating Hash Maps

```rust
use std::collections::HashMap;

// Empty hash map
let mut scores: HashMap<String, i32> = HashMap::new();

// Insert values
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// From vectors using zip and collect
let teams = vec![String::from("Blue"), String::from("Red")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

### Accessing Values

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Using get (returns Option<&V>)
let team_name = String::from("Blue");
if let Some(score) = scores.get(&team_name) {
    println!("Score: {}", score);
}

// With default value
let score = scores.get(&String::from("Blue")).unwrap_or(&0);
```

### Updating Hash Maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Overwriting a value
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);  // Overwrites

// Only insert if key doesn't exist
scores.entry(String::from("Red")).or_insert(50);
scores.entry(String::from("Red")).or_insert(100);  // Doesn't change

// Update based on old value
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);  // {"hello": 1, "world": 2, "wonderful": 1}
```

### Iterating Over Hash Maps

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Iterate over key-value pairs
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// Iterate over keys
for key in scores.keys() {
    println!("Key: {}", key);
}

// Iterate over values
for value in scores.values() {
    println!("Value: {}", value);
}

// Mutable iteration
for value in scores.values_mut() {
    *value += 10;
}
```

### Useful HashMap Methods

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("a", 1);
map.insert("b", 2);

// Check if key exists
if map.contains_key("a") {
    println!("Key 'a' exists");
}

// Remove entry
map.remove("a");

// Get number of entries
println!("Length: {}", map.len());

// Clear all entries
map.clear();

// Capacity
let mut map = HashMap::with_capacity(10);
```

---

## Hash Sets (HashSet<T>)

Hash sets store unique values for fast membership testing.

### Creating and Using Hash Sets

```rust
use std::collections::HashSet;

// Create empty set
let mut set: HashSet<i32> = HashSet::new();

// Insert elements
set.insert(1);
set.insert(2);
set.insert(3);
set.insert(2);  // Duplicate - not added

println!("{:?}", set);  // {1, 2, 3}

// From vector
let nums = vec![1, 2, 3, 2, 1, 4];
let set: HashSet<_> = nums.into_iter().collect();
println!("{:?}", set);  // {1, 2, 3, 4}
```

### Set Operations

```rust
use std::collections::HashSet;

let mut set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
let mut set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();

// Check membership
println!("Contains 2: {}", set1.contains(&2));

// Union
let union: HashSet<_> = set1.union(&set2).cloned().collect();
println!("Union: {:?}", union);  // {1, 2, 3, 4, 5}

// Intersection
let intersection: HashSet<_> = set1.intersection(&set2).cloned().collect();
println!("Intersection: {:?}", intersection);  // {3}

// Difference
let difference: HashSet<_> = set1.difference(&set2).cloned().collect();
println!("Difference: {:?}", difference);  // {1, 2}

// Symmetric difference
let sym_diff: HashSet<_> = set1.symmetric_difference(&set2).cloned().collect();
println!("Symmetric difference: {:?}", sym_diff);  // {1, 2, 4, 5}

// Subset check
println!("Is subset: {}", set1.is_subset(&set2));
```

### Practical Example: Removing Duplicates

```rust
use std::collections::HashSet;

fn remove_duplicates(vec: Vec<i32>) -> Vec<i32> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

let numbers = vec![1, 2, 3, 2, 1, 4, 5, 4];
let unique = remove_duplicates(numbers);
println!("{:?}", unique);
```

---

## VecDeque (Double-Ended Queue)

Efficient insertion and removal from both ends.

```rust
use std::collections::VecDeque;

// Create deque
let mut deque = VecDeque::new();

// Add to back
deque.push_back(1);
deque.push_back(2);

// Add to front
deque.push_front(0);

println!("{:?}", deque);  // [0, 1, 2]

// Remove from back
if let Some(back) = deque.pop_back() {
    println!("Removed from back: {}", back);
}

// Remove from front
if let Some(front) = deque.pop_front() {
    println!("Removed from front: {}", front);
}

// Access front and back
if let Some(f) = deque.front() {
    println!("Front: {}", f);
}
if let Some(b) = deque.back() {
    println!("Back: {}", b);
}

// Use as a queue (FIFO)
deque.push_back(3);
deque.push_back(4);
while let Some(item) = deque.pop_front() {
    println!("Processing: {}", item);
}
```

---

## BTreeMap and BTreeSet

Sorted collections using B-tree data structure.

### BTreeMap

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "c");
map.insert(1, "a");
map.insert(2, "b");

// Iteration is in sorted key order
for (key, value) in &map {
    println!("{}: {}", key, value);  // 1: a, 2: b, 3: c
}

// Range queries
for (key, value) in map.range(1..3) {
    println!("{}: {}", key, value);
}
```

### BTreeSet

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(3);
set.insert(1);
set.insert(2);

// Iteration is in sorted order
for value in &set {
    println!("{}", value);  // 1, 2, 3
}

// Range queries
for value in set.range(1..3) {
    println!("{}", value);
}
```

---

## Binary Heap

Priority queue implementation.

```rust
use std::collections::BinaryHeap;

// Max heap by default
let mut heap = BinaryHeap::new();

heap.push(3);
heap.push(1);
heap.push(5);
heap.push(2);

// Always get the largest element
while let Some(max) = heap.pop() {
    println!("{}", max);  // 5, 3, 2, 1
}

// Min heap using Reverse
use std::cmp::Reverse;
let mut min_heap = BinaryHeap::new();
min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(5));

while let Some(Reverse(min)) = min_heap.pop() {
    println!("{}", min);  // 1, 3, 5
}
```

---

## LinkedList

Doubly-linked list (rarely used in Rust).

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();

list.push_back(1);
list.push_back(2);
list.push_front(0);

println!("{:?}", list);  // [0, 1, 2]

// Iterate
for item in &list {
    println!("{}", item);
}

// Note: Vec is usually better than LinkedList in Rust!
```

---

## Tuples and Arrays (Review)

### Tuples

```rust
// Fixed-size, heterogeneous
let tuple: (i32, f64, &str) = (42, 3.14, "hello");

// Destructuring
let (x, y, z) = tuple;

// Access by index
let first = tuple.0;
let second = tuple.1;
```

### Arrays

```rust
// Fixed-size, homogeneous
let arr: [i32; 5] = [1, 2, 3, 4, 5];

// Initialize with same value
let arr = [0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

// Access
let first = arr[0];

// Slicing
let slice = &arr[1..3];

// Iteration
for element in &arr {
    println!("{}", element);
}
```

---

## Structs (Custom Data Structures)

### Classic Structs

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

let user1 = User {
    email: String::from("user@example.com"),
    username: String::from("user123"),
    active: true,
    sign_in_count: 1,
};

// Accessing fields
println!("Email: {}", user1.email);

// Update syntax
let user2 = User {
    email: String::from("another@example.com"),
    ..user1  // Rest from user1
};
```

### Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### Unit-Like Structs

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

### Methods on Structs

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

let rect = Rectangle { width: 30, height: 50 };
println!("Area: {}", rect.area());

let sq = Rectangle::square(25);
```

---

## Enums and Pattern Matching

### Defining Enums

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));
```

### Enums with Different Types

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("Color: ({}, {}, {})", r, g, b),
        }
    }
}
```

### Pattern Matching

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

---

## Option and Result

### Option<T>

```rust
// Option represents an optional value
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

// Using Option
match find_user(1) {
    Some(name) => println!("Found: {}", name),
    None => println!("Not found"),
}

// Using if let
if let Some(name) = find_user(1) {
    println!("Found: {}", name);
}

// Unwrap or default
let name = find_user(2).unwrap_or(String::from("Guest"));
```

### Result<T, E>

```rust
use std::fs::File;
use std::io::ErrorKind;

fn open_file(filename: &str) -> Result<File, std::io::Error> {
    File::open(filename)
}

// Handling Result
match open_file("hello.txt") {
    Ok(file) => println!("File opened"),
    Err(error) => match error.kind() {
        ErrorKind::NotFound => println!("File not found"),
        other_error => println!("Error: {:?}", other_error),
    },
}

// Using ? operator (propagates errors)
fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut file = File::open("username.txt")?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

---

## Iterators and Iteration

### Creating Iterators

```rust
let v = vec![1, 2, 3];

// iter() - borrows each element
for val in v.iter() {
    println!("{}", val);
}

// iter_mut() - mutable borrow
let mut v = vec![1, 2, 3];
for val in v.iter_mut() {
    *val += 1;
}

// into_iter() - takes ownership
for val in v.into_iter() {
    println!("{}", val);
}
// v is no longer valid
```

### Iterator Adapters

```rust
let v = vec![1, 2, 3, 4, 5, 6];

// map - transform elements
let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();

// filter - select elements
let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();

// Chain operations
let result: Vec<_> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();

println!("{:?}", result);  // [4, 8, 12]
```

### Consuming Adapters

```rust
let v = vec![1, 2, 3, 4, 5];

// sum
let total: i32 = v.iter().sum();

// product
let product: i32 = v.iter().product();

// count
let count = v.iter().count();

// min and max
let min = v.iter().min();
let max = v.iter().max();

// any and all
let has_even = v.iter().any(|x| x % 2 == 0);
let all_positive = v.iter().all(|x| *x > 0);

// find
let first_even = v.iter().find(|x| *x % 2 == 0);

// fold
let sum = v.iter().fold(0, |acc, x| acc + x);
```

---

## Performance Considerations

### Choosing the Right Collection

```rust
// Use Vec<T> when:
// - You need a growable array
// - You mostly append/remove from the end
// - You access elements by index

// Use HashMap<K, V> when:
// - You need key-value associations
// - Fast lookups by key are important

// Use HashSet<T> when:
// - You need unique values
// - Fast membership testing

// Use VecDeque<T> when:
// - You need to add/remove from both ends

// Use BTreeMap/BTreeSet when:
// - You need sorted data
// - You need range queries

// Use BinaryHeap when:
// - You need a priority queue
```

### Capacity Management

```rust
// Pre-allocate capacity
let mut v = Vec::with_capacity(1000);

// Check capacity
println!("Capacity: {}", v.capacity());

// Shrink to fit
v.shrink_to_fit();

// Reserve additional capacity
v.reserve(100);
```

---

## Practical Examples

See the code examples in this directory for complete implementations of:

1. **student_records** - Student management system using various collections
2. **word_counter** - Text analysis with HashMap
3. **task_manager** - Priority-based task queue
4. **inventory_system** - Product inventory with custom structs
5. **graph_implementation** - Graph data structure

---

## Best Practices

### 1. Choose the Right Collection

```rust
// ✅ Good: Vec for simple list
let numbers = vec![1, 2, 3, 4, 5];

// ❌ Bad: LinkedList (rarely needed)
let numbers = LinkedList::new();
```

### 2. Use Iterators

```rust
// ✅ Good: Functional style
let sum: i32 = numbers.iter().sum();

// ❌ Bad: Manual loop (less idiomatic)
let mut sum = 0;
for num in &numbers {
    sum += num;
}
```

### 3. Pre-allocate When Possible

```rust
// ✅ Good: Known size
let mut v = Vec::with_capacity(1000);

// ❌ Less optimal: Unknown size, many reallocations
let mut v = Vec::new();
```

### 4. Use Entry API for HashMaps

```rust
// ✅ Good: Single lookup
map.entry(key).or_insert(0) += 1;

// ❌ Bad: Multiple lookups
if !map.contains_key(&key) {
    map.insert(key, 0);
}
*map.get_mut(&key).unwrap() += 1;
```

---

## Exercises

### Exercise 1: Phone Book
Create a phone book using HashMap that can:
- Add contacts
- Search by name
- Update phone numbers
- Delete contacts
- List all contacts

### Exercise 2: Word Frequency
Write a program that:
- Reads text input
- Counts word frequency
- Displays top 10 most common words
- Shows statistics

### Exercise 3: Student Grade System
Build a system that:
- Stores student records (struct)
- Calculates average grades
- Finds top performers
- Groups students by grade level

### Exercise 4: Set Operations
Implement a program that:
- Takes two lists of numbers
- Finds union, intersection, difference
- Identifies unique elements
- Detects duplicates

### Exercise 5: Task Priority Queue
Create a task manager that:
- Uses BinaryHeap for priorities
- Adds tasks with priority levels
- Processes tasks in priority order
- Supports task cancellation

---

## Summary

Today you learned:

✅ **Vectors**: Dynamic arrays with `Vec<T>`  
✅ **Strings**: `String` and `&str` types  
✅ **Hash Maps**: Key-value storage with `HashMap<K, V>`  
✅ **Hash Sets**: Unique value collections  
✅ **Advanced Collections**: VecDeque, BTreeMap, BinaryHeap  
✅ **Structs**: Custom data structures  
✅ **Enums**: Algebraic data types  
✅ **Option/Result**: Handling optional and error values  
✅ **Iterators**: Efficient data processing  
✅ **Performance**: Choosing the right collection  

### Key Takeaways

1. **Vec is the default choice** for most list needs
2. **HashMap for key-value** associations
3. **Iterators are powerful** and idiomatic
4. **Choose based on use case** - each collection has strengths
5. **Option and Result** are essential for safe Rust

### Next Steps

Tomorrow, we'll explore:
- **Ownership and Borrowing** in depth
- **Lifetimes** and reference validity
- **Smart Pointers**: Box, Rc, RefCell

### Additional Resources

- [The Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust by Example - Collections](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [std::collections Documentation](https://doc.rust-lang.org/std/collections/)

---

Happy coding! 🦀

*Last Updated: November 23, 2025*
