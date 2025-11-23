# Rust Data Structures - Quick Start

## Running the Examples

### 1. Student Records System

```powershell
cd student_records
cargo run
```

**Demonstrates:**
- Vec for storing grades
- HashMap for student database
- Custom struct (Student)
- Sorting and filtering
- Statistical analysis

**Features:**
- Add students
- Record grades
- Calculate averages
- View top students
- Class statistics

### 2. Word Counter

```powershell
cd word_counter
cargo run
```

**Demonstrates:**
- HashMap for word frequency
- HashSet for unique words
- String processing
- Text analysis algorithms

**Features:**
- Word frequency counting
- Character frequency
- Unique word counting
- Longest/shortest words
- Statistical analysis

**Usage:**
```powershell
# Test with a text file
cargo run < test.txt

# Or type/paste text manually
```

### 3. Task Manager

```powershell
cd task_manager
cargo run
```

**Demonstrates:**
- BinaryHeap for priority queue
- Custom Ord implementation
- Task prioritization
- Vec for completed tasks

**Features:**
- Add tasks with priorities
- Process tasks by priority
- View pending/completed tasks
- Priority statistics

### 4. Inventory System

```powershell
cd inventory_system
cargo run
```

**Demonstrates:**
- HashMap for product storage
- Enums for categories
- Complex struct design
- Search and filter operations

**Features:**
- Product management
- Inventory tracking
- Search by name/category
- Low stock alerts
- Value calculations

## Running Tests

```powershell
# Word Counter tests
cd word_counter
cargo test

# All projects support testing
cargo test --all
```

## Key Data Structures Used

### Collections Summary

| Structure | Use Case | Example Project |
|-----------|----------|-----------------|
| `Vec<T>` | Dynamic arrays | Student Records (grades) |
| `HashMap<K, V>` | Key-value pairs | All projects |
| `HashSet<T>` | Unique values | Word Counter |
| `BinaryHeap<T>` | Priority queue | Task Manager |
| Structs | Custom types | All projects |
| Enums | Variants/States | Inventory (categories) |

### Performance Tips

```rust
// Pre-allocate capacity when size is known
let mut v = Vec::with_capacity(100);

// Use entry API for HashMaps
map.entry(key).or_insert(0) += 1;

// Iterator chains are efficient
let result: Vec<_> = data
    .iter()
    .filter(|x| x.is_valid())
    .map(|x| x.process())
    .collect();
```

## Common Patterns Demonstrated

### 1. Builder Pattern
```rust
let student = Student::new(id, name)
    .add_grade(90.0)
    .add_grade(85.0);
```

### 2. Option/Result Handling
```rust
match db.get_student(id) {
    Some(student) => student.display(),
    None => println!("Not found"),
}
```

### 3. Iterator Chains
```rust
let top_students: Vec<_> = students
    .iter()
    .filter(|s| s.average() > 90.0)
    .take(5)
    .collect();
```

### 4. Entry API
```rust
*freq_map.entry(word).or_insert(0) += 1;
```

## Troubleshooting

### "Borrowed value does not live long enough"
Ensure references don't outlive their data:
```rust
// ❌ Bad
let r;
{
    let x = 5;
    r = &x;  // x dies here
}

// ✅ Good
let x = 5;
let r = &x;
```

### "Cannot borrow as mutable"
Only one mutable reference at a time:
```rust
// ❌ Bad
let mut v = vec![1, 2, 3];
let r1 = &mut v;
let r2 = &mut v;  // Error

// ✅ Good
let mut v = vec![1, 2, 3];
{
    let r1 = &mut v;
    // use r1
} // r1 scope ends
let r2 = &mut v;  // OK
```

### HashMap ownership issues
Use `.get()` for borrowing:
```rust
// Returns Option<&V>
if let Some(value) = map.get(&key) {
    println!("{}", value);
}
```

## Next Steps

1. ✅ Run each example and experiment
2. ✅ Modify the code to add features
3. ✅ Complete the exercises in README.md
4. ✅ Try combining multiple data structures
5. ✅ Build your own application

## Additional Examples

### Creating a Phone Book
```rust
use std::collections::HashMap;

let mut phone_book: HashMap<String, String> = HashMap::new();
phone_book.insert("Alice".to_string(), "555-1234".to_string());
```

### Set Operations
```rust
use std::collections::HashSet;

let set1: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let set2: HashSet<_> = vec![3, 4, 5].into_iter().collect();

let union: HashSet<_> = set1.union(&set2).cloned().collect();
```

### Priority Queue
```rust
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(5);

while let Some(max) = heap.pop() {
    println!("{}", max);  // 5, 3, 1
}
```

Happy coding! 🦀
