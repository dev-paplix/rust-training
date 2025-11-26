# Python Extension Demo

This project demonstrates how to create a Python extension module using PyO3, allowing you to write high-performance Python libraries in Rust.

## Overview

The `mathpy` module provides:

- **Basic functions**: `add()`, `multiply()`, `divide()`, `sum_list()`, `max_list()`
- **String functions**: `greet()`, `to_uppercase()`, `word_frequency()`
- **Counter class**: A mutable counter with increment/decrement operations
- **Point class**: 2D point with distance and midpoint calculations
- **Calculator class**: Stateful calculator with chained operations
- **Person class**: Example of a class with validation and properties

## Prerequisites

- Rust toolchain (rustc, cargo)
- Python 3.8 or later
- pip (Python package manager)

## Installation

### Method 1: Using maturin (Recommended)

[Maturin](https://github.com/PyO3/maturin) is the recommended tool for building and publishing PyO3-based Python packages.

```bash
# Install maturin
pip install maturin

# Development installation (editable mode)
maturin develop

# Or build a release version
maturin develop --release
```

### Method 2: Using pip

```bash
pip install .
```

### Method 3: Building a wheel

```bash
# Install maturin
pip install maturin

# Build a wheel file
maturin build --release

# Install the wheel
pip install target/wheels/mathpy-0.1.0-*.whl
```

## Usage

### Basic Functions

```python
import mathpy

# Arithmetic
result = mathpy.add(5, 3)          # 8
result = mathpy.multiply(6, 7)     # 42
result = mathpy.divide(10.0, 2.0)  # 5.0

# List operations
numbers = [1, 2, 3, 4, 5]
total = mathpy.sum_list(numbers)   # 15
max_val = mathpy.max_list(numbers) # 5

# String operations
greeting = mathpy.greet("Alice")   # "Hello, Alice! Welcome from Rust."
upper = mathpy.to_uppercase("hello") # "HELLO"

# Word frequency
text = "the quick brown fox jumps over the lazy dog"
freq = mathpy.word_frequency(text)
# {'the': 2, 'quick': 1, 'brown': 1, ...}
```

### Counter Class

```python
import mathpy

counter = mathpy.Counter()
counter.increment()           # value = 1
counter.increment_by(5)       # value = 6
print(counter.get_value())    # 6
counter.decrement()           # value = 5
counter.reset()               # value = 0

# Custom starting value
counter2 = mathpy.Counter(start=10)
```

### Point Class

```python
import mathpy

p1 = mathpy.Point(0.0, 0.0)
p2 = mathpy.Point(3.0, 4.0)

# Calculate distance
distance = p1.distance_to(p2)  # 5.0

# Find midpoint
mid = p1.midpoint(p2)          # Point(x=1.5, y=2.0)

# Translate (move) a point
p1.translate(10.0, 20.0)       # p1 is now at (10.0, 20.0)

# Access and modify properties
print(p1.x, p1.y)              # 10.0 20.0
p1.x = 100.0
p1.y = 200.0
```

### Calculator Class

```python
import mathpy

calc = mathpy.Calculator()
calc.add(10.0)
calc.multiply(2.0)
calc.subtract(5.0)
calc.divide(3.0)
result = calc.get_result()     # 5.0
calc.reset()                   # back to 0.0
```

### Person Class

```python
import mathpy

person = mathpy.Person("Alice", 30)
greeting = person.greet()      # "Hello, my name is Alice and I am 30 years old."
is_adult = person.is_adult()   # True

person.have_birthday()
print(person.age)              # 31

# Modify properties
person.name = "Bob"
person.age = 25
```

## Error Handling

The library uses Python exceptions for error handling:

```python
import mathpy

# Division by zero
try:
    result = mathpy.divide(10.0, 0.0)
except ValueError as e:
    print(f"Error: {e}")  # "Cannot divide by zero"

# Empty list
try:
    max_val = mathpy.max_list([])
except ValueError as e:
    print(f"Error: {e}")  # "Cannot find max of empty list"

# Invalid age
try:
    person = mathpy.Person("Charlie", -5)
except ValueError as e:
    print(f"Error: {e}")  # "Age cannot be negative"
```

## Running Tests

### Rust Tests

```bash
cargo test
```

### Python Tests

```bash
# Install the module first
maturin develop

# Run the test script
python examples/test.py
```

## Building for Production

### Create an optimized build

```bash
maturin build --release
```

This creates a wheel file in `target/wheels/` that you can distribute.

### Publishing to PyPI

```bash
# Build the wheel
maturin build --release

# Upload to PyPI (requires PyPI account)
maturin publish
```

## Performance

Rust extensions can provide significant performance improvements for computationally intensive tasks. Here's a simple benchmark comparing Python and Rust implementations:

```python
import time
import mathpy

# Python implementation
def sum_python(numbers):
    return sum(numbers)

# Rust implementation via mathpy
def sum_rust(numbers):
    return mathpy.sum_list(numbers)

# Benchmark
numbers = list(range(1_000_000))

start = time.time()
result_python = sum_python(numbers)
python_time = time.time() - start

start = time.time()
result_rust = sum_rust(numbers)
rust_time = time.time() - start

print(f"Python: {python_time:.6f}s")
print(f"Rust:   {rust_time:.6f}s")
print(f"Speedup: {python_time / rust_time:.2f}x")
```

## Project Structure

```
python_extension_demo/
├── Cargo.toml          # Rust dependencies
├── pyproject.toml      # Python project configuration
├── src/
│   └── lib.rs          # Rust implementation
└── examples/
    └── test.py         # Python test script
```

## Development Tips

### Type Hints

While PyO3 doesn't automatically generate type stubs, you can create them manually for better IDE support:

```python
# mathpy.pyi (type stub file)
def add(a: int, b: int) -> int: ...
def greet(name: str) -> str: ...

class Point:
    x: float
    y: float
    def __init__(self, x: float, y: float) -> None: ...
    def distance_to(self, other: Point) -> float: ...
```

### Debugging

To enable debug symbols:

```bash
maturin develop  # Debug build by default
```

### Reloading During Development

After making changes to the Rust code:

```bash
maturin develop
```

Then restart your Python interpreter or use:

```python
import importlib
import mathpy
importlib.reload(mathpy)
```

## Common Issues

### ImportError: DLL load failed

**Windows**: Make sure you have the Visual C++ Redistributable installed.

### Symbol not found

**macOS**: Ensure you're using the correct Python version that matches your build.

### Module not found

Make sure you've installed the module:
```bash
maturin develop
```

## Learning Resources

- [PyO3 Guide](https://pyo3.rs/)
- [Maturin Documentation](https://maturin.rs/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Python/Rust Interoperability](https://www.py-o3.rs/latest/python_from_rust.html)

## License

This example is provided for educational purposes.
