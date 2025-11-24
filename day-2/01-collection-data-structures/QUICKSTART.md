# Collection Data Structures - Quick Start

## Running the Examples

### 1. Vector Manager

```powershell
cd vector_manager
cargo run
```

**Demonstrates:**
- Vec<T> operations (push, pop, insert, remove)
- Dynamic array growth
- Indexing and slicing
- Iteration methods (iter, iter_mut, into_iter)
- Sorting and searching
- Capacity management

**Features:**
- Interactive vector manipulation
- Element management
- Statistical operations
- Vector transformations
- Performance demonstrations

### 2. HashMap Operations

```powershell
cd hashmap_operations
cargo run
```

**Demonstrates:**
- HashMap<K, V> creation and usage
- Key-value pair management
- Entry API patterns
- Iteration over maps
- Frequency counting
- Data aggregation

**Features:**
- Phone book application
- Word frequency counter
- Data lookup and updates
- Collision handling
- Performance comparisons

### 3. HashSet Explorer

```powershell
cd hashset_explorer
cargo run
```

**Demonstrates:**
- HashSet<T> for unique values
- Set operations (union, intersection, difference)
- Membership testing
- Deduplication
- Set algebra

**Features:**
- Set theory operations
- Unique value collections
- Comparison operations
- Practical set applications

### 4. Advanced Collections

```powershell
cd advanced_collections
cargo run
```

**Demonstrates:**
- VecDeque for double-ended operations
- BTreeMap for sorted key-value pairs
- BTreeSet for sorted unique values
- BinaryHeap for priority queues
- LinkedList (when to avoid it)

**Features:**
- Queue and stack implementations
- Priority task management
- Sorted data handling
- Performance comparisons
- Collection selection guide

## Running Tests

```powershell
# Vector Manager tests
cd vector_manager
cargo test

# All projects support testing
cargo test --all
```

## Key Collections Covered

### Collections Summary

| Collection | Use Case | Time Complexity | Example Project |
|-----------|----------|-----------------|-----------------|
| `Vec<T>` | Dynamic arrays, indexed access | O(1) access, O(1) amortized push | Vector Manager |
| `HashMap<K, V>` | Fast key-value lookup | O(1) average | HashMap Operations |
| `HashSet<T>` | Unique values, fast membership | O(1) average | HashSet Explorer |
| `VecDeque<T>` | Double-ended queue | O(1) both ends | Advanced Collections |
| `BTreeMap<K, V>` | Sorted key-value pairs | O(log n) | Advanced Collections |
| `BTreeSet<T>` | Sorted unique values | O(log n) | Advanced Collections |
| `BinaryHeap<T>` | Priority queue | O(log n) insert/remove | Advanced Collections |
| `LinkedList<T>` | Rarely needed in Rust | O(1) splice | Advanced Collections |

### Performance Characteristics

```rust
// Vec - Best for most use cases
let mut v = Vec::new();
v.push(1);              // O(1) amortized
let x = v[0];           // O(1)
v.insert(0, 1);         // O(n)

// HashMap - Fast lookups
let mut map = HashMap::new();
map.insert(key, value); // O(1) average
let val = map.get(&key); // O(1) average

// HashSet - Fast membership
let mut set = HashSet::new();
set.insert(value);      // O(1) average
set.contains(&value);   // O(1) average

// VecDeque - Efficient both ends
let mut deque = VecDeque::new();
deque.push_front(1);    // O(1)
deque.push_back(2);     // O(1)
```

## Common Patterns Demonstrated

### 1. Vec Capacity Management
```rust
// Pre-allocate for known size
let mut v = Vec::with_capacity(1000);

// Check and reserve capacity
if v.capacity() - v.len() < 100 {
    v.reserve(100);
}

// Shrink to fit
v.shrink_to_fit();
```

### 2. HashMap Entry API
```rust
// Efficient word counting
let mut freq = HashMap::new();
for word in text.split_whitespace() {
    *freq.entry(word).or_insert(0) += 1;
}

// Insert only if absent
freq.entry("key").or_insert(default_value);

// Modify existing or insert
freq.entry("key")
    .and_modify(|v| *v += 1)
    .or_insert(1);
```

### 3. Set Operations
```rust
let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let set2: HashSet<_> = vec![3, 4, 5].into_iter().collect();

// Union
let union: HashSet<_> = set1.union(&set2).copied().collect();

// Intersection
let intersection: HashSet<_> = set1.intersection(&set2).copied().collect();

// Difference
let diff: HashSet<_> = set1.difference(&set2).copied().collect();
```

### 4. Iterator Chains
```rust
let result: Vec<_> = collection
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect();
```

## Troubleshooting

### "Cannot borrow as mutable more than once"
```rust
// ❌ Bad: Multiple mutable borrows
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // Error!

// ✅ Good: One mutable borrow at a time
let mut v = vec![1, 2, 3];
{
    let r1 = &mut v;
    // use r1
} // r1 goes out of scope
let r2 = &mut v;  // OK
```

### "Cannot move out of indexed content"
```rust
// ❌ Bad: Moving out of Vec
let v = vec![String::from("hello")];
let s = v[0];  // Error: cannot move

// ✅ Good: Clone or use references
let s = v[0].clone();  // Clone the value
let s = &v[0];         // Borrow instead
```

### HashMap Type Inference Issues
```rust
// ❌ Ambiguous types
let mut map = HashMap::new();
// Compiler doesn't know K and V types yet

// ✅ Explicit types
let mut map: HashMap<String, i32> = HashMap::new();

// ✅ Or let compiler infer from usage
let mut map = HashMap::new();
map.insert(String::from("key"), 42);  // Now compiler knows
```

### Index Out of Bounds
```rust
// ❌ Panics if index invalid
let v = vec![1, 2, 3];
let x = v[10];  // Panic!

// ✅ Safe access with get()
if let Some(x) = v.get(10) {
    println!("{}", x);
} else {
    println!("Index out of bounds");
}
```

## Next Steps

1. ✅ Run each example and experiment
2. ✅ Modify the code to add features
3. ✅ Complete the exercises in README.md
4. ✅ Benchmark different collections
5. ✅ Build your own collection-based application

## Additional Examples

### Building a Word Index
```rust
use std::collections::HashMap;

let mut index: HashMap<String, Vec<usize>> = HashMap::new();
for (i, word) in text.split_whitespace().enumerate() {
    index.entry(word.to_string())
        .or_insert_with(Vec::new)
        .push(i);
}
```

### Removing Duplicates While Preserving Order
```rust
use std::collections::HashSet;

let mut seen = HashSet::new();
let unique: Vec<_> = items
    .into_iter()
    .filter(|x| seen.insert(x.clone()))
    .collect();
```

### Priority Queue with BinaryHeap
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(5);
heap.push(1);
heap.push(3);

while let Some(max) = heap.pop() {
    println!("{}", max);  // 5, 3, 1
}
```

### VecDeque as Queue (FIFO)
```rust
use std::collections::VecDeque;

let mut queue = VecDeque::new();
queue.push_back(1);  // Enqueue
queue.push_back(2);
queue.push_back(3);

while let Some(item) = queue.pop_front() {  // Dequeue
    println!("{}", item);  // 1, 2, 3
}
```

### BTreeMap for Sorted Iteration
```rust
use std::collections::BTreeMap;

let mut map = BTreeMap::new();
map.insert(3, "three");
map.insert(1, "one");
map.insert(2, "two");

for (k, v) in &map {
    println!("{}: {}", k, v);  // 1: one, 2: two, 3: three
}
```

## Collection Selection Guide

**Choose Vec when:**
- You need indexed access
- You're mostly adding/removing from the end
- You need to sort the data
- Memory locality is important

**Choose HashMap when:**
- You need fast key-value lookups
- Keys are unique
- Order doesn't matter
- You need to count occurrences

**Choose HashSet when:**
- You need unique values
- Fast membership testing is important
- You need set operations (union, intersection)
- Order doesn't matter

**Choose VecDeque when:**
- You need to add/remove from both ends
- Implementing a queue or deque
- Ring buffer applications

**Choose BTreeMap/BTreeSet when:**
- You need sorted iteration
- You need range queries
- You need to find min/max efficiently

**Choose BinaryHeap when:**
- You need a priority queue
- You always need the maximum element
- You're implementing algorithms like Dijkstra's

**Avoid LinkedList unless:**
- You need to splice collections together
- You need specific split/append operations
- (Usually Vec or VecDeque is better!)

Happy coding! 🦀
