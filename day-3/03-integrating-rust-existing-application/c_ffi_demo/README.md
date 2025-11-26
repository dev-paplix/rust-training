# C FFI Demo

This project demonstrates how to create a Rust library that can be called from C code using Foreign Function Interface (FFI).

## Overview

The library provides various functions demonstrating different FFI patterns:

- **Basic numeric operations**: `add()`, `multiply()`, `divide()`
- **String handling**: `greet()`, `to_uppercase()`, `string_length()`
- **Array processing**: `sum_array()`, `max_array()`, `sort_array()`
- **Struct operations**: `Point` type with distance, midpoint, and translate functions
- **Error handling**: Functions returning error codes and results

## Building

### Prerequisites

- Rust toolchain (rustc, cargo)
- C compiler:
  - **Windows**: MSVC (Visual Studio Build Tools) or MinGW
  - **Linux**: GCC
  - **macOS**: Clang (Xcode Command Line Tools)

### Build the Library

```bash
cargo build --release
```

This creates:
- **Windows**: `target/release/mathlib.dll` and `mathlib.lib`
- **Linux**: `target/release/libmathlib.so`
- **macOS**: `target/release/libmathlib.dylib`

### Run Tests

```bash
cargo test
```

## Using from C

### Windows (MSVC)

1. Build the library:
   ```powershell
   cargo build --release
   ```

2. Compile your C program:
   ```cmd
   cl your_program.c /I. /link target\release\mathlib.lib
   ```

3. Make sure `mathlib.dll` is in the same directory as your executable or in your PATH.

### Windows (PowerShell Script)

Use the provided build script:
```powershell
.\build_and_test.ps1
```

### Linux/macOS (Bash Script)

Use the provided build script:
```bash
chmod +x build_and_test.sh
./build_and_test.sh
```

### Manual Compilation (Linux/macOS)

```bash
# Build the Rust library
cargo build --release

# Compile the C program
gcc examples/test.c -L target/release -lmathlib -lm -o test

# Run (Linux)
export LD_LIBRARY_PATH=target/release:$LD_LIBRARY_PATH
./test

# Run (macOS)
export DYLD_LIBRARY_PATH=target/release:$DYLD_LIBRARY_PATH
./test
```

## Example Usage

See `examples/test.c` for a complete example. Here's a quick snippet:

```c
#include <stdio.h>

// Function declarations
extern int add(int a, int b);
extern char* greet(const char* name);
extern void free_rust_string(char* s);

int main() {
    // Call Rust function
    int result = add(5, 3);
    printf("5 + 3 = %d\n", result);
    
    // String handling
    char* greeting = greet("Alice");
    printf("%s\n", greeting);
    free_rust_string(greeting);  // Important: free memory!
    
    return 0;
}
```

## API Reference

### Basic Functions

```c
int add(int a, int b);
int multiply(int a, int b);
OperationResult divide(double a, double b);
```

### String Functions

```c
char* greet(const char* name);
char* to_uppercase(const char* input);
int string_length(const char* input);
void free_rust_string(char* s);
```

**Important**: Strings returned by Rust functions must be freed using `free_rust_string()`.

### Array Functions

```c
int sum_array(const int* arr, size_t len);
int max_array(const int* arr, size_t len);
void sort_array(int* arr, size_t len);
```

### Point Functions

```c
typedef struct {
    double x;
    double y;
} Point;

Point point_new(double x, double y);
double point_distance(Point p1, Point p2);
Point point_midpoint(Point p1, Point p2);
void point_translate(Point* point, double dx, double dy);
```

### Error Handling

```c
typedef enum {
    Success = 0,
    NullPointer = -1,
    InvalidUtf8 = -2,
    BufferTooSmall = -3,
} ErrorCode;

ErrorCode parse_int(const char* input, int* output);
ErrorCode copy_string(const char* src, char* dest, size_t dest_len);
```

## Generating C Headers

To generate a C header file automatically using cbindgen:

```bash
# Install cbindgen
cargo install cbindgen

# Generate header
cbindgen --config cbindgen.toml --crate c_ffi_demo --output mathlib.h
```

## Memory Management

### Key Rules

1. **Strings from Rust**: Must be freed with `free_rust_string()`
2. **Strings to Rust**: Must be null-terminated and valid UTF-8
3. **Arrays**: Must pass correct length; Rust doesn't check bounds
4. **Structs**: Use `#[repr(C)]` for C compatibility

### Safety Considerations

- Always check for null pointers before calling Rust functions
- Ensure strings are null-terminated
- Don't use pointers after freeing them
- Pass accurate array lengths
- Handle error codes appropriately

## Common Issues

### Windows: Missing DLL

**Problem**: "The program can't start because mathlib.dll is missing"

**Solutions**:
- Copy `mathlib.dll` to the same directory as your executable
- Add `target\release` to your PATH
- Use the provided `build_and_test.ps1` script

### Linux: Library Not Found

**Problem**: "error while loading shared libraries: libmathlib.so"

**Solutions**:
```bash
export LD_LIBRARY_PATH=target/release:$LD_LIBRARY_PATH
```

Or install the library system-wide:
```bash
sudo cp target/release/libmathlib.so /usr/local/lib/
sudo ldconfig
```

### macOS: Library Not Found

**Problem**: Similar to Linux

**Solutions**:
```bash
export DYLD_LIBRARY_PATH=target/release:$DYLD_LIBRARY_PATH
```

## Project Structure

```
c_ffi_demo/
├── Cargo.toml              # Rust project configuration
├── cbindgen.toml           # cbindgen configuration
├── build_and_test.ps1      # Windows build script
├── build_and_test.sh       # Linux/macOS build script
├── src/
│   └── lib.rs              # Rust library implementation
└── examples/
    └── test.c              # C test program
```

## Learning Resources

- [The Rustonomicon - FFI](https://doc.rust-lang.org/nomicon/ffi.html)
- [Rust FFI Guide](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [cbindgen Documentation](https://github.com/eqrion/cbindgen)

## License

This example is provided for educational purposes.
