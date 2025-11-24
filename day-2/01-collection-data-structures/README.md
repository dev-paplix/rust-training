# Collection Data Structures

## Table of Contents

- [Introduction](#introduction)
- [Learning Objectives](#learning-objectives)
- [Why Collections Matter](#why-collections-matter)
- [Vector (Vec\<T\>)](#vector-vect)
- [HashMap (HashMap\<K, V\>)](#hashmap-hashmapk-v)
- [HashSet (HashSet\<T\>)](#hashset-hashsett)
- [VecDeque (VecDeque\<T\>)](#vecdeque-vecdequet)
- [BTreeMap and BTreeSet](#btreemap-and-btreeset)
- [BinaryHeap (BinaryHeap\<T\>)](#binaryheap-binaryheapt)
- [LinkedList (LinkedList\<T\>)](#linkedlist-linkedlistt)
- [Choosing the Right Collection](#choosing-the-right-collection)
- [Common Collection Patterns](#common-collection-patterns)
- [Iterator Methods](#iterator-methods)
- [Performance Considerations](#performance-considerations)
- [Memory Layout](#memory-layout)
- [Best Practices](#best-practices)
- [Practical Examples](#practical-examples)
- [Exercises](#exercises)
- [Summary](#summary)

---

## Introduction

Collections are fundamental data structures that store multiple values. Unlike arrays with fixed sizes, Rust's collections are stored on the heap and can grow or shrink dynamically at runtime. Understanding when and how to use each collection type is crucial for writing efficient Rust programs.

This lesson provides an in-depth exploration of Rust's standard library collections, their performance characteristics, use cases, and best practices.

**Core Principles:**
- **Ownership**: Collections own their data
- **Heap Allocation**: Dynamic sizing capability
- **Type Safety**: Generic with compile-time checking
- **Zero-Cost Abstractions**: High-level with low-level performance

---

## Learning Objectives

By completing this lesson, you will be able to:

✅ Choose the appropriate collection for any task  
✅ Master Vec\<T\> operations and capacity management  
✅ Use HashMap\<K, V\> efficiently with the Entry API  
✅ Apply HashSet\<T\> for uniqueness and set operations  
✅ Work with specialized collections (VecDeque, BTreeMap, BinaryHeap)  
✅ Understand time and space complexity of operations  
✅ Use iterators effectively with collections  
✅ Apply ownership rules to collection operations  
✅ Optimize collection performance  
✅ Debug common collection-related errors  

---

## Why Collections Matter

### Real-World Applications

```rust
// User management system
let mut users: HashMap<u64, User> = HashMap::new();

// Task priority queue
let mut tasks: BinaryHeap<Task> = BinaryHeap::new();

// Shopping cart
let mut cart: Vec<Product> = Vec::new();

// Unique visitor tracking
let mut visitors: HashSet<IpAddr> = HashSet::new();

// Order processing queue
let mut orders: VecDeque<Order> = VecDeque::new();
```

### Performance Impact

Choosing the right collection can make the difference between O(1) and O(n) operations:

```rust
// Finding an item
// Vec: O(n) - must search linearly
// HashMap: O(1) - direct lookup
// BTreeMap: O(log n) - binary search

// Inserting at start
// Vec: O(n) - must shift all elements
// VecDeque: O(1) - optimized for both ends
```

---

## Vector (Vec\<T\>)

The most versatile and commonly used collection. A contiguous, growable array.

### Creating Vectors

```rust
// Empty vector with type annotation
let v1: Vec<i32> = Vec::new();

// Using vec! macro
let v2 = vec![1, 2, 3, 4, 5];

// With initial capacity
let v3 = Vec::with_capacity(100);

// From an iterator
let v4: Vec<i32> = (0..10).collect();

// From array
let v5 = Vec::from([1, 2, 3]);

// Filled with default values
let v6 = vec![0; 10];  // [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
```

### Adding Elements

```rust
let mut v = Vec::new();

// Add to end (amortized O(1))
v.push(1);
v.push(2);
v.push(3);

// Extend with another collection
v.extend([4, 5, 6]);
v.extend_from_slice(&[7, 8, 9]);

// Insert at specific position (O(n))
v.insert(0, 0);  // Insert at beginning

// Append another vector
let mut v2 = vec![10, 11, 12];
v.append(&mut v2);  // v2 is now empty
```

### Removing Elements

```rust
let mut v = vec![1, 2, 3, 4, 5];

// Remove from end (O(1))
if let Some(last) = v.pop() {
    println!("Removed: {}", last);
}

// Remove at index (O(n))
let removed = v.remove(1);  // Removes index 1, shifts rest

// Swap and remove (O(1) but changes order)
let removed = v.swap_remove(1);  // Swaps with last, then removes

// Remove matching elements
v.retain(|&x| x > 2);  // Keeps only elements > 2

// Clear all elements
v.clear();

// Truncate to length
v.truncate(3);  // Keep only first 3 elements
```

### Accessing Elements

```rust
let v = vec![1, 2, 3, 4, 5];

// Indexing (panics if out of bounds)
let third = v[2];

// Safe access with get (returns Option)
match v.get(2) {
    Some(value) => println!("Third element: {}", value),
    None => println!("No element at index 2"),
}

// First and last
let first = v.first();   // Option<&i32>
let last = v.last();     // Option<&i32>

// Mutable access
let mut v = vec![1, 2, 3];
if let Some(first) = v.first_mut() {
    *first = 10;
}

// Slicing
let slice = &v[1..3];  // [2, 3]
let slice = &v[..2];   // [1, 2]
let slice = &v[2..];   // [3, 4, 5]
```

### Iteration

```rust
let v = vec![1, 2, 3, 4, 5];

// Immutable iteration
for &value in &v {
    println!("{}", value);
}

// Mutable iteration
let mut v = vec![1, 2, 3];
for value in &mut v {
    *value *= 2;
}

// Consuming iteration (takes ownership)
for value in v {
    println!("{}", value);
}
// v is no longer valid here

// Enumerate with index
for (i, &value) in v.iter().enumerate() {
    println!("Index {}: {}", i, value);
}
```

### Capacity Management

```rust
let mut v = Vec::with_capacity(10);
println!("Capacity: {}", v.capacity());  // 10
println!("Length: {}", v.len());         // 0

// Add elements
for i in 0..5 {
    v.push(i);
}
println!("Capacity: {}", v.capacity());  // Still 10
println!("Length: {}", v.len());         // 5

// Reserve additional capacity
v.reserve(20);  // Ensure at least 20 more slots

// Reserve exact capacity
v.reserve_exact(30);

// Shrink to fit
v.shrink_to_fit();
println!("Capacity: {}", v.capacity());  // Likely 5
```

### Sorting and Searching

```rust
let mut v = vec![3, 1, 4, 1, 5, 9, 2, 6];

// Sort (ascending)
v.sort();  // [1, 1, 2, 3, 4, 5, 6, 9]

// Sort descending
v.sort_by(|a, b| b.cmp(a));

// Sort by key
let mut people = vec![
    Person { name: "Alice", age: 30 },
    Person { name: "Bob", age: 25 },
];
people.sort_by_key(|p| p.age);

// Reverse
v.reverse();

// Binary search (requires sorted vector)
v.sort();
match v.binary_search(&5) {
    Ok(pos) => println!("Found at position {}", pos),
    Err(pos) => println!("Not found, would insert at {}", pos),
}

// Check if contains
if v.contains(&5) {
    println!("Vector contains 5");
}

// Deduplication (requires sorted)
v.dedup();
```

### Useful Methods

```rust
let v = vec![1, 2, 3, 4, 5];

// Length
println!("Length: {}", v.len());

// Is empty?
if v.is_empty() {
    println!("Vector is empty");
}

// Split into chunks
for chunk in v.chunks(2) {
    println!("{:?}", chunk);
}

// Windows (sliding)
for window in v.windows(3) {
    println!("{:?}", window);
}

// Split at index
let (left, right) = v.split_at(2);

// Drain elements
let mut v = vec![1, 2, 3, 4, 5];
let drained: Vec<_> = v.drain(1..3).collect();  // Removes and returns [2, 3]
```

---

## HashMap (HashMap\<K, V\>)

Hash table for fast key-value lookups with O(1) average complexity.

### Creating HashMaps

```rust
use std::collections::HashMap;

// Empty HashMap
let mut map: HashMap<String, i32> = HashMap::new();

// With capacity
let mut map = HashMap::with_capacity(100);

// From iterator of tuples
let map: HashMap<_, _> = vec![
    ("a", 1),
    ("b", 2),
    ("c", 3),
].into_iter().collect();

// Using zip
let keys = vec!["a", "b", "c"];
let values = vec![1, 2, 3];
let map: HashMap<_, _> = keys.into_iter()
    .zip(values.into_iter())
    .collect();
```

### Basic Operations

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Insert
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Red"), 50);

// Access
let team = String::from("Blue");
if let Some(score) = scores.get(&team) {
    println!("Score: {}", score);
}

// Get with default
let score = scores.get(&team).unwrap_or(&0);
let score = scores.get(&team).copied().unwrap_or(0);

// Check existence
if scores.contains_key(&team) {
    println!("Team exists");
}

// Remove
if let Some(score) = scores.remove(&team) {
    println!("Removed score: {}", score);
}

// Update (overwrites)
scores.insert(String::from("Blue"), 25);
```

### Entry API (Most Efficient)

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

// Insert only if absent
scores.entry(String::from("Yellow"))
    .or_insert(50);

// Insert or modify
scores.entry(String::from("Blue"))
    .and_modify(|v| *v += 10)
    .or_insert(0);

// Word frequency counter
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
// {("hello", 1), ("world", 2), ("wonderful", 1)}

// Complex default value
map.entry(key)
    .or_insert_with(|| expensive_computation());
```

### Iteration

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
for (key, value) in &mut scores {
    *value += 10;
}

// Consuming iteration
for (key, value) in scores {
    println!("{}: {}", key, value);
}
// scores is no longer valid
```

### Practical Examples

```rust
use std::collections::HashMap;

// Grouping data
let mut groups: HashMap<char, Vec<String>> = HashMap::new();
for word in words {
    let first_char = word.chars().next().unwrap();
    groups.entry(first_char)
        .or_insert_with(Vec::new)
        .push(word);
}

// Caching
let mut cache: HashMap<String, ExpensiveResult> = HashMap::new();
fn get_or_compute(key: &str, cache: &mut HashMap<String, ExpensiveResult>) -> &ExpensiveResult {
    cache.entry(key.to_string())
        .or_insert_with(|| expensive_computation(key))
}

// Counting occurrences
fn count_chars(s: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    counts
}
```

---

## HashSet (HashSet\<T\>)

Collection of unique values with O(1) average lookup.

### Creating HashSets

```rust
use std::collections::HashSet;

// Empty set
let mut set: HashSet<i32> = HashSet::new();

// From vector
let set: HashSet<_> = vec![1, 2, 3, 2, 1].into_iter().collect();
// {1, 2, 3} - duplicates removed

// From array
let set: HashSet<_> = [1, 2, 3].into_iter().collect();
```

### Basic Operations

```rust
use std::collections::HashSet;

let mut set = HashSet::new();

// Insert (returns true if inserted, false if already present)
set.insert(1);
set.insert(2);
set.insert(2);  // Returns false, already exists

// Contains
if set.contains(&1) {
    println!("Set contains 1");
}

// Remove
set.remove(&1);

// Length
println!("Size: {}", set.len());

// Clear
set.clear();
```

### Set Operations

```rust
use std::collections::HashSet;

let set1: HashSet<_> = vec![1, 2, 3, 4].into_iter().collect();
let set2: HashSet<_> = vec![3, 4, 5, 6].into_iter().collect();

// Union (all elements from both sets)
let union: HashSet<_> = set1.union(&set2).copied().collect();
// {1, 2, 3, 4, 5, 6}

// Intersection (common elements)
let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();
// {3, 4}

// Difference (in set1 but not set2)
let diff: HashSet<_> = set1.difference(&set2).copied().collect();
// {1, 2}

// Symmetric difference (in either but not both)
let sym_diff: HashSet<_> = set1.symmetric_difference(&set2).copied().collect();
// {1, 2, 5, 6}

// Subset check
if set1.is_subset(&set2) {
    println!("set1 is subset of set2");
}

// Disjoint check
if set1.is_disjoint(&set2) {
    println!("Sets have no common elements");
}
```

### Practical Uses

```rust
use std::collections::HashSet;

// Remove duplicates
fn remove_duplicates<T: Eq + std::hash::Hash>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

// Remove duplicates preserving order
fn remove_duplicates_ordered<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}

// Find unique elements
let list1 = vec![1, 2, 3, 4];
let list2 = vec![3, 4, 5, 6];
let set1: HashSet<_> = list1.into_iter().collect();
let set2: HashSet<_> = list2.into_iter().collect();
let unique_to_list1: Vec<_> = set1.difference(&set2).collect();
```

---

## VecDeque (VecDeque\<T\>)

Double-ended queue with efficient operations at both ends.

### Creating and Using VecDeque

```rust
use std::collections::VecDeque;

// Create empty deque
let mut deque = VecDeque::new();

// With capacity
let mut deque = VecDeque::with_capacity(10);

// From vector
let deque: VecDeque<_> = vec![1, 2, 3].into();

// Add to front and back
deque.push_front(0);   // [0, 1, 2, 3]
deque.push_back(4);    // [0, 1, 2, 3, 4]

// Remove from front and back
let front = deque.pop_front();  // Some(0), deque = [1, 2, 3, 4]
let back = deque.pop_back();    // Some(4), deque = [1, 2, 3]

// Access front and back
if let Some(front) = deque.front() {
    println!("Front: {}", front);
}
if let Some(back) = deque.back() {
    println!("Back: {}", back);
}

// Mutable access
if let Some(front) = deque.front_mut() {
    *front += 10;
}
```

### Use Cases

```rust
use std::collections::VecDeque;

// Queue (FIFO)
let mut queue = VecDeque::new();
queue.push_back(1);  // Enqueue
queue.push_back(2);
queue.push_back(3);

while let Some(item) = queue.pop_front() {  // Dequeue
    println!("{}", item);  // 1, 2, 3
}

// Stack (LIFO)
let mut stack = VecDeque::new();
stack.push_back(1);  // Push
stack.push_back(2);
stack.push_back(3);

while let Some(item) = stack.pop_back() {  // Pop
    println!("{}", item);  // 3, 2, 1
}

// Sliding window
fn sliding_window_max(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut deque = VecDeque::new();
    let mut result = Vec::new();
    
    for (i, &num) in nums.iter().enumerate() {
        // Remove elements outside window
        while !deque.is_empty() && deque[0] < i - k + 1 {
            deque.pop_front();
        }
        
        // Remove smaller elements (won't be max)
        while !deque.is_empty() && nums[*deque.back().unwrap()] < num {
            deque.pop_back();
        }
        
        deque.push_back(i);
        
        if i >= k - 1 {
            result.push(nums[deque[0]]);
        }
    }
    
    result
}
```

---

## BTreeMap and BTreeSet

Sorted collections using a B-tree structure with O(log n) operations.

### BTreeMap

```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();

// Insert elements
map.insert(3, "three");
map.insert(1, "one");
map.insert(2, "two");

// Iteration is in sorted key order
for (key, value) in &map {
    println!("{}: {}", key, value);  // 1: one, 2: two, 3: three
}

// Range queries
for (key, value) in map.range(1..3) {
    println!("{}: {}", key, value);  // 1: one, 2: two
}

// Get first and last
if let Some((first_key, first_val)) = map.first_key_value() {
    println!("First: {} -> {}", first_key, first_val);
}

if let Some((last_key, last_val)) = map.last_key_value() {
    println!("Last: {} -> {}", last_key, last_val);
}
```

### BTreeSet

```rust
use std::collections::BTreeSet;

let mut set = BTreeSet::new();

// Insert elements
set.insert(3);
set.insert(1);
set.insert(2);

// Iteration is in sorted order
for value in &set {
    println!("{}", value);  // 1, 2, 3
}

// Range queries
for value in set.range(1..3) {
    println!("{}", value);  // 1, 2
}

// Get first and last
if let Some(&first) = set.first() {
    println!("First: {}", first);
}

if let Some(&last) = set.last() {
    println!("Last: {}", last);
}
```

### When to Use BTree Collections

- When you need sorted iteration
- When you need range queries
- When you need to efficiently find min/max
- When you need consistent O(log n) performance (no hash collision issues)

---

## BinaryHeap (BinaryHeap\<T\>)

Priority queue implemented as a max-heap.

### Basic Usage

```rust
use std::collections::BinaryHeap;

// Create max heap
let mut heap = BinaryHeap::new();

// Add elements
heap.push(3);
heap.push(1);
heap.push(5);
heap.push(2);

// Always get the largest element
while let Some(max) = heap.pop() {
    println!("{}", max);  // 5, 3, 2, 1
}

// Peek without removing
if let Some(&max) = heap.peek() {
    println!("Max: {}", max);
}
```

### Min Heap

```rust
use std::collections::BinaryHeap;
use std::cmp::Reverse;

// Create min heap using Reverse
let mut min_heap = BinaryHeap::new();

min_heap.push(Reverse(3));
min_heap.push(Reverse(1));
min_heap.push(Reverse(5));

while let Some(Reverse(min)) = min_heap.pop() {
    println!("{}", min);  // 1, 3, 5
}
```

### Custom Ordering

```rust
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
struct Task {
    priority: u8,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

let mut tasks = BinaryHeap::new();
tasks.push(Task { priority: 3, name: "Low".to_string() });
tasks.push(Task { priority: 1, name: "High".to_string() });
tasks.push(Task { priority: 2, name: "Medium".to_string() });

while let Some(task) = tasks.pop() {
    println!("{} (priority {})", task.name, task.priority);
}
```

---

## LinkedList (LinkedList\<T\>)

Doubly-linked list. **Rarely used in Rust** - Vec or VecDeque is usually better!

### When to Consider LinkedList

- When you need to split or append lists in O(1)
- When you need specific list algorithms
- **Usually Vec or VecDeque is better due to cache locality**

```rust
use std::collections::LinkedList;

let mut list = LinkedList::new();

// Add elements
list.push_back(1);
list.push_back(2);
list.push_front(0);

// Split at position
let mut split = list.split_off(1);  // O(n)

// Append another list
list.append(&mut split);  // O(1)

// Most operations are O(n) or O(1) but slower than Vec
```

---

## Choosing the Right Collection

### Decision Tree

```
Need to store multiple values?
├─ Need indexed access?
│  ├─ Fixed size? → Array [T; N]
│  └─ Dynamic size? → Vec<T>
│
├─ Need key-value pairs?
│  ├─ Need sorted keys? → BTreeMap<K, V>
│  └─ Fast lookup? → HashMap<K, V>
│
├─ Need unique values?
│  ├─ Need sorted values? → BTreeSet<T>
│  └─ Fast membership test? → HashSet<T>
│
├─ Need both-end operations? → VecDeque<T>
│
├─ Need priority queue? → BinaryHeap<T>
│
└─ Need to splice lists? → LinkedList<T> (rare)
```

### Performance Comparison

| Operation | Vec | VecDeque | HashMap | HashSet | BTreeMap | BinaryHeap |
|-----------|-----|----------|---------|---------|----------|------------|
| Push end | O(1)* | O(1)* | - | - | - | O(log n) |
| Push front | O(n) | O(1)* | - | - | - | - |
| Pop end | O(1) | O(1)* | - | - | - | O(log n) |
| Pop front | O(n) | O(1)* | - | - | - | - |
| Index access | O(1) | O(1) | O(1)* | - | O(log n) | - |
| Insert | - | - | O(1)* | O(1)* | O(log n) | - |
| Search | O(n) | O(n) | O(1)* | O(1)* | O(log n) | - |
| Iteration | Fast | Fast | Fast | Fast | Sorted | - |

\* = amortized or average case

---

## Common Collection Patterns

### Pattern 1: Frequency Counting

```rust
use std::collections::HashMap;

fn count_frequencies<T: Eq + std::hash::Hash>(items: Vec<T>) -> HashMap<T, usize> {
    let mut freq = HashMap::new();
    for item in items {
        *freq.entry(item).or_insert(0) += 1;
    }
    freq
}
```

### Pattern 2: Grouping

```rust
use std::collections::HashMap;

fn group_by<K, V, F>(items: Vec<V>, key_fn: F) -> HashMap<K, Vec<V>>
where
    K: Eq + std::hash::Hash,
    F: Fn(&V) -> K,
{
    let mut groups = HashMap::new();
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    groups
}
```

### Pattern 3: Caching

```rust
use std::collections::HashMap;

struct Cache<K, V> {
    store: HashMap<K, V>,
}

impl<K: Eq + std::hash::Hash, V> Cache<K, V> {
    fn new() -> Self {
        Cache { store: HashMap::new() }
    }
    
    fn get_or_insert_with<F>(&mut self, key: K, f: F) -> &V
    where
        F: FnOnce() -> V,
    {
        self.store.entry(key).or_insert_with(f)
    }
}
```

### Pattern 4: Deduplication

```rust
use std::collections::HashSet;

// Remove duplicates (unordered)
fn dedupe<T: Eq + std::hash::Hash>(vec: Vec<T>) -> Vec<T> {
    let set: HashSet<_> = vec.into_iter().collect();
    set.into_iter().collect()
}

// Remove duplicates (preserve order)
fn dedupe_ordered<T: Eq + std::hash::Hash + Clone>(vec: Vec<T>) -> Vec<T> {
    let mut seen = HashSet::new();
    vec.into_iter()
        .filter(|item| seen.insert(item.clone()))
        .collect()
}
```

---

## Iterator Methods

### Common Iterator Operations

```rust
let numbers = vec![1, 2, 3, 4, 5, 6];

// Map - transform elements
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();

// Filter - select elements
let evens: Vec<_> = numbers.iter().filter(|&&x| x % 2 == 0).collect();

// Fold - reduce to single value
let sum: i32 = numbers.iter().fold(0, |acc, x| acc + x);

// Chain operations
let result: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|x| x * 2)
    .collect();

// Find
let first_even = numbers.iter().find(|&&x| x % 2 == 0);

// Any/All
let has_even = numbers.iter().any(|&x| x % 2 == 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// Take/Skip
let first_three: Vec<_> = numbers.iter().take(3).collect();
let skip_two: Vec<_> = numbers.iter().skip(2).collect();

// Enumerate
for (i, &num) in numbers.iter().enumerate() {
    println!("Index {}: {}", i, num);
}
```

---

## Performance Considerations

### Memory Allocation

```rust
// Pre-allocate when size is known
let mut v = Vec::with_capacity(1000);
for i in 0..1000 {
    v.push(i);  // No reallocation needed
}

// HashMap with capacity
let mut map = HashMap::with_capacity(100);
```

### Avoiding Unnecessary Copies

```rust
// ❌ Bad: Unnecessary cloning
let v = vec![String::from("hello")];
for s in &v {
    let owned = s.clone();  // Expensive!
    process(owned);
}

// ✅ Good: Use references
for s in &v {
    process(s);  // Borrow
}
```

### Iterator vs Loop

```rust
// Iterator: Often faster due to optimization
let sum: i32 = numbers.iter().sum();

// Manual loop: More explicit, similar performance
let mut sum = 0;
for &num in &numbers {
    sum += num;
}
```

---

## Memory Layout

### Vec Memory Layout

```
Stack:              Heap:
┌─────────┐        ┌───┬───┬───┬───┬───┐
│ ptr     │───────>│ 1 │ 2 │ 3 │ 4 │ 5 │
├─────────┤        └───┴───┴───┴───┴───┘
│ len: 5  │
├─────────┤
│ cap: 10 │
└─────────┘
```

### HashMap Memory Layout

```
Conceptual (actual implementation more complex):
┌────────────────────────────┐
│  Hash Table (buckets)      │
│  ┌─────┬─────┬─────┬─────┐│
│  │Entry│Entry│Empty│Entry││
│  └─────┴─────┴─────┴─────┘│
│         ↓                  │
│    ┌─────────┐             │
│    │ Key: K  │             │
│    │ Val: V  │             │
│    └─────────┘             │
└────────────────────────────┘
```

---

## Best Practices

### 1. Choose Vec by Default

```rust
// ✅ Vec for most list needs
let items = vec![1, 2, 3];

// ❌ LinkedList rarely needed
let items = LinkedList::new();
```

### 2. Use Entry API for HashMap

```rust
// ✅ Efficient: Single lookup
map.entry(key).or_insert(value);

// ❌ Inefficient: Multiple lookups
if !map.contains_key(&key) {
    map.insert(key, value);
}
```

### 3. Pre-allocate When Possible

```rust
// ✅ Good: Known size
let mut v = Vec::with_capacity(size);

// ❌ Less optimal: Unknown size, many reallocations
let mut v = Vec::new();
```

### 4. Use Iterators

```rust
// ✅ Idiomatic and often faster
let sum: i32 = numbers.iter().sum();

// ❌ Verbose
let mut sum = 0;
for &n in &numbers {
    sum += n;
}
```

### 5. Avoid Unnecessary Cloning

```rust
// ✅ Good: Borrow when possible
for item in &collection {
    process(item);
}

// ❌ Bad: Unnecessary ownership
for item in collection.clone() {
    process(item);
}
```

---

## Practical Examples

See the code examples in this directory:

1. **vector_manager** - Comprehensive Vec operations
2. **hashmap_operations** - HashMap and Entry API patterns
3. **hashset_explorer** - Set operations and deduplication
4. **advanced_collections** - VecDeque, BTreeMap, BinaryHeap

---

## Exercises

### Exercise 1: Student Database
Build a student management system using:
- HashMap to store students by ID
- Vec to store grades
- Calculate statistics (average, top students)

### Exercise 2: Word Analyzer
Create a text analyzer that:
- Counts word frequency (HashMap)
- Finds unique words (HashSet)
- Maintains word order (Vec with dedup)

### Exercise 3: Task Scheduler
Implement a priority-based task scheduler:
- Use BinaryHeap for priority queue
- Process tasks by priority
- Support task cancellation

### Exercise 4: Set Calculator
Build a set calculator that:
- Performs union, intersection, difference
- Handles multiple sets
- Shows Venn diagram results

### Exercise 5: Cache Implementation
Create an LRU cache using:
- HashMap for fast lookup
- VecDeque or custom structure for LRU tracking
- Size limits and eviction

---

## Summary

Today you learned:

✅ **Vec\<T\>**: The default choice for lists  
✅ **HashMap\<K, V\>**: Fast key-value lookups  
✅ **HashSet\<T\>**: Unique values and set operations  
✅ **VecDeque\<T\>**: Efficient both-end operations  
✅ **BTreeMap/BTreeSet**: Sorted collections  
✅ **BinaryHeap\<T\>**: Priority queues  
✅ **Performance**: Time/space complexity  
✅ **Patterns**: Common collection use cases  
✅ **Iterators**: Powerful data processing  
✅ **Best Practices**: Choosing and using collections efficiently  

### Key Takeaways

1. **Vec is the default** - Use it unless you have specific needs
2. **HashMap for lookups** - O(1) average performance
3. **Pre-allocate when possible** - Reduces allocations
4. **Use Entry API** - Most efficient HashMap updates
5. **Iterators are powerful** - Composable and often optimized

### Next Steps

Next, we'll explore:
- **Control Flow**: if, match, loops
- **Error Handling**: Result and Option patterns
- **Pattern Matching**: Advanced matching techniques

### Additional Resources

- [The Rust Book - Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [std::collections Documentation](https://doc.rust-lang.org/std/collections/)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)

---

Happy coding! 🦀

*Last Updated: November 24, 2025*
