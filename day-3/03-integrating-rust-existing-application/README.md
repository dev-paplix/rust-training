# Integrating Rust with Existing Applications

A comprehensive guide to integrating Rust code into existing applications written in other languages.

## Table of Contents

1. [Introduction](#introduction)
2. [Why Integrate Rust?](#why-integrate-rust)
3. [Integration Approaches](#integration-approaches)
4. [FFI Fundamentals](#ffi-fundamentals)
5. [C Integration](#c-integration)
6. [Python Integration](#python-integration)
7. [JavaScript/Node.js Integration](#javascript-nodejs-integration)
8. [WebAssembly Integration](#webassembly-integration)
9. [Memory Management](#memory-management)
10. [Error Handling Across FFI](#error-handling-across-ffi)
11. [Best Practices](#best-practices)
12. [Performance Considerations](#performance-considerations)

---

## Introduction

Integrating Rust into existing applications allows you to leverage Rust's performance and safety without rewriting entire codebases. This guide covers multiple integration strategies and provides practical examples.

### What You'll Learn

- Foreign Function Interface (FFI) basics
- Creating C-compatible Rust libraries
- Python extensions with PyO3
- Node.js addons with Neon
- WebAssembly for web applications
- Memory safety across language boundaries
- Error handling strategies
- Performance optimization

---

## Why Integrate Rust?

### Performance-Critical Components

Replace slow parts of your application with Rust:

```
Before: Python image processing (slow)
After:  Rust image processing + Python wrapper (fast)
Result: 10-100x performance improvement
```

### Memory Safety

Eliminate memory bugs in critical paths:
- Buffer overflows → Prevented by Rust's borrow checker
- Use-after-free → Compile-time error in Rust
- Data races → Caught at compile time

### Gradual Migration

Start small and expand:
1. Identify performance bottleneck
2. Rewrite in Rust
3. Expose through FFI
4. Repeat for other components

### Leverage Existing Ecosystems

Combine strengths:
- Python: Data science, ML libraries
- Rust: High-performance computation
- JavaScript: Frontend, UI
- Rust: WebAssembly for browser performance

---

## Integration Approaches

### 1. Foreign Function Interface (FFI)

**Best for**: Any language with C interop

**Pros**:
- Universal compatibility
- Mature tooling
- Fine control over interface

**Cons**:
- Manual memory management
- Requires understanding of C ABIs
- More boilerplate

### 2. Language-Specific Bindings

**Best for**: Deep integration with specific languages

**Examples**:
- **PyO3** for Python
- **Neon** for Node.js
- **wasm-bindgen** for WebAssembly

**Pros**:
- Idiomatic APIs
- Automatic type conversions
- Better developer experience

**Cons**:
- Language-specific
- Learning curve for each tool

### 3. WebAssembly

**Best for**: Web applications

**Pros**:
- Browser-native
- Sandboxed execution
- Growing ecosystem

**Cons**:
- Limited system access
- Startup overhead
- Still maturing

### 4. Microservices

**Best for**: Distributed systems

**Pros**:
- Language independence
- Easy deployment
- Scalability

**Cons**:
- Network overhead
- Operational complexity

---

## FFI Fundamentals

### The C ABI

FFI relies on the C Application Binary Interface (ABI):

```rust
// Rust function with C ABI
#[no_mangle]
pub extern \"C\" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

**Key points**:
- `#[no_mangle]`: Prevents Rust from changing function names
- `extern \"C\"`: Uses C calling convention
- `pub`: Makes function visible outside crate

### Type Compatibility

| Rust Type | C Type | Size | Notes |
|-----------|--------|------|-------|
| `i8` | `int8_t` | 1 byte | Signed |
| `i16` | `int16_t` | 2 bytes | Signed |
| `i32` | `int32_t` | 4 bytes | Signed |
| `i64` | `int64_t` | 8 bytes | Signed |
| `u8` | `uint8_t` | 1 byte | Unsigned |
| `u16` | `uint16_t` | 2 bytes | Unsigned |
| `u32` | `uint32_t` | 4 bytes | Unsigned |
| `u64` | `uint64_t` | 8 bytes | Unsigned |
| `f32` | `float` | 4 bytes | IEEE 754 |
| `f64` | `double` | 8 bytes | IEEE 754 |
| `bool` | `uint8_t` | 1 byte | 0 or 1 |
| `()` | `void` | 0 bytes | No return |
| `*const T` | `const T*` | 8 bytes | Pointer |
| `*mut T` | `T*` | 8 bytes | Mutable pointer |

### Library Types

```toml
[lib]
crate-type = [\"cdylib\"]    # Dynamic library (.so, .dll, .dylib)
# OR
crate-type = [\"staticlib\"]  # Static library (.a, .lib)
# OR
crate-type = [\"cdylib\", \"rlib\"]  # Both C-compatible and Rust
```

**Output**:
- Linux: `libmylib.so` (cdylib) or `libmylib.a` (staticlib)
- macOS: `libmylib.dylib` (cdylib) or `libmylib.a` (staticlib)
- Windows: `mylib.dll` (cdylib) or `mylib.lib` (staticlib)

---

## C Integration

### Creating a Rust Library for C

**Cargo.toml**:
```toml
[package]
name = \"math_lib\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]
```

**src/lib.rs**:
```rust
use std::os::raw::c_int;

/// Add two numbers
#[no_mangle]
pub extern \"C\" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

/// Multiply two numbers
#[no_mangle]
pub extern \"C\" fn multiply(a: c_int, b: c_int) -> c_int {
    a * b
}
```

### Generating C Headers

Install `cbindgen`:
```bash
cargo install cbindgen
```

**cbindgen.toml**:
```toml
language = \"C\"
include_guard = \"MATH_LIB_H\"
autogen_warning = \"/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */\"
```

Generate header:
```bash
cbindgen --config cbindgen.toml --crate math_lib --output math_lib.h
```

**Generated math_lib.h**:
```c
#ifndef MATH_LIB_H
#define MATH_LIB_H

#include <stdint.h>

int32_t add(int32_t a, int32_t b);
int32_t multiply(int32_t a, int32_t b);

#endif /* MATH_LIB_H */
```

### Using from C

**main.c**:
```c
#include <stdio.h>
#include \"math_lib.h\"

int main() {
    int result = add(5, 3);
    printf(\"5 + 3 = %d\\n\", result);
    
    result = multiply(4, 6);
    printf(\"4 * 6 = %d\\n\", result);
    
    return 0;
}
```

**Build**:
```bash
# Build Rust library
cargo build --release

# Compile C program (Linux/macOS)
gcc -o main main.c -L./target/release -lmath_lib

# Run
LD_LIBRARY_PATH=./target/release ./main
```

### Handling Strings

Strings require special handling due to different representations:

**Rust**:
```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern \"C\" fn greet(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    
    let c_str = unsafe { CStr::from_ptr(name) };
    let r_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    
    let greeting = format!(\"Hello, {}!\", r_str);
    
    match CString::new(greeting) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern \"C\" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}
```

**C**:
```c
#include <stdio.h>
#include \"lib.h\"

int main() {
    char* greeting = greet(\"Alice\");
    if (greeting != NULL) {
        printf(\"%s\\n\", greeting);
        free_string(greeting);  // Important: free the string!
    }
    return 0;
}
```

### Handling Arrays

**Rust**:
```rust
#[no_mangle]
pub extern \"C\" fn sum_array(data: *const i32, len: usize) -> i32 {
    if data.is_null() {
        return 0;
    }
    
    let slice = unsafe {
        std::slice::from_raw_parts(data, len)
    };
    
    slice.iter().sum()
}
```

**C**:
```c
#include <stdio.h>
#include \"lib.h\"

int main() {
    int numbers[] = {1, 2, 3, 4, 5};
    int sum = sum_array(numbers, 5);
    printf(\"Sum: %d\\n\", sum);
    return 0;
}
```

### Complex Structures

**Rust**:
```rust
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub extern \"C\" fn point_distance(p1: Point, p2: Point) -> f64 {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    (dx * dx + dy * dy).sqrt()
}

#[no_mangle]
pub extern \"C\" fn point_new(x: f64, y: f64) -> Point {
    Point { x, y }
}
```

**Important**: Use `#[repr(C)]` for C-compatible struct layout!

---

## Python Integration

### Using PyO3

PyO3 provides Rust bindings for Python, making it easy to create Python extensions.

### Setup

**Cargo.toml**:
```toml
[package]
name = \"string_utils\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
name = \"string_utils\"
crate-type = [\"cdylib\"]

[dependencies]
pyo3 = { version = \"0.20\", features = [\"extension-module\"] }
```

**pyproject.toml**:
```toml
[build-system]
requires = [\"maturin>=1.0,<2.0\"]
build-backend = \"maturin\"

[project]
name = \"string_utils\"
requires-python = \">=3.7\"
```

### Simple Function

**src/lib.rs**:
```rust
use pyo3::prelude::*;

#[pyfunction]
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

#[pyfunction]
fn reverse_string(text: &str) -> String {
    text.chars().rev().collect()
}

#[pymodule]
fn string_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    m.add_function(wrap_pyfunction!(reverse_string, m)?)?;
    Ok(())
}
```

### Python Class

```rust
use pyo3::prelude::*;

#[pyclass]
struct Counter {
    count: i32,
}

#[pymethods]
impl Counter {
    #[new]
    fn new() -> Self {
        Counter { count: 0 }
    }
    
    fn increment(&mut self) {
        self.count += 1;
    }
    
    fn get_count(&self) -> i32 {
        self.count
    }
    
    fn reset(&mut self) {
        self.count = 0;
    }
}

#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Counter>()?;
    Ok(())
}
```

### Building and Using

```bash
# Install maturin
pip install maturin

# Develop mode (for testing)
maturin develop

# Build wheel
maturin build --release
```

**Python usage**:
```python
import string_utils

# Use functions
word_count = string_utils.count_words(\"Hello world\")
reversed_text = string_utils.reverse_string(\"Rust\")

# Use classes
from my_module import Counter
counter = Counter()
counter.increment()
print(counter.get_count())  # 1
```

### Error Handling

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyfunction]
fn divide(a: f64, b: f64) -> PyResult<f64> {
    if b == 0.0 {
        Err(PyValueError::new_err(\"Cannot divide by zero\"))
    } else {
        Ok(a / b)
    }
}
```

**Python**:
```python
try:
    result = divide(10, 0)
except ValueError as e:
    print(f\"Error: {e}\")
```

### Type Conversions

PyO3 automatically handles conversions:

| Rust Type | Python Type |
|-----------|-------------|
| `i32`, `i64` | `int` |
| `f32`, `f64` | `float` |
| `bool` | `bool` |
| `String`, `&str` | `str` |
| `Vec<T>` | `list` |
| `HashMap<K, V>` | `dict` |
| `Option<T>` | `T` or `None` |
| `PyResult<T>` | `T` or exception |

---

## JavaScript/Node.js Integration

### Using Neon

Neon allows you to write native Node.js modules in Rust.

### Setup

```bash
# Install Neon CLI
npm install -g neon-cli

# Create new project
neon new my-addon

# Or add to existing project
cd my-project
neon init
```

### Basic Function

**src/lib.rs**:
```rust
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(\"Hello from Rust!\"))
}

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value(&mut cx);
    let b = cx.argument::<JsNumber>(1)?.value(&mut cx);
    Ok(cx.number(a + b))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function(\"hello\", hello)?;
    cx.export_function(\"add\", add)?;
    Ok(())
}
```

### Using in Node.js

```javascript
const addon = require('./');

console.log(addon.hello());        // \"Hello from Rust!\"
console.log(addon.add(5, 3));      // 8
```

### Async Operations

```rust
use neon::prelude::*;
use std::thread;
use std::time::Duration;

fn async_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    
    let channel = cx.channel();
    
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));
        
        channel.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![cx.string(\"Task completed!\")];
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    
    Ok(cx.undefined())
}
```

**JavaScript**:
```javascript
addon.asyncTask((result) => {
    console.log(result);  // \"Task completed!\" (after 2 seconds)
});
```

---

## WebAssembly Integration

### Using wasm-bindgen

wasm-bindgen provides bindings between WebAssembly and JavaScript.

### Setup

**Cargo.toml**:
```toml
[package]
name = \"wasm_demo\"
version = \"0.1.0\"
edition = \"2021\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
wasm-bindgen = \"0.2\"

[dependencies.web-sys]
version = \"0.3\"
features = [\"console\"]
```

### Basic Functions

**src/lib.rs**:
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!(\"Hello, {}!\", name)
}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
```

### Building

```bash
# Install wasm-pack
cargo install wasm-pack

# Build for web
wasm-pack build --target web

# Build for Node.js
wasm-pack build --target nodejs

# Build for bundlers (webpack, etc.)
wasm-pack build --target bundler
```

### Using in Browser

**index.html**:
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset=\"utf-8\">
    <title>WASM Demo</title>
</head>
<body>
    <script type=\"module\">
        import init, { greet, add, fibonacci } from './pkg/wasm_demo.js';
        
        async function run() {
            await init();
            
            console.log(greet('WebAssembly'));  // \"Hello, WebAssembly!\"
            console.log(add(5, 7));             // 12
            console.log(fibonacci(10));         // 55
        }
        
        run();
    </script>
</body>
</html>
```

### DOM Manipulation

```rust
use wasm_bindgen::prelude::*;
use web_sys::{Document, Element, HtmlElement};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect(\"no global window\");
    let document = window.document().expect(\"no document\");
    
    let body = document.body().expect(\"no body\");
    
    let p = document.create_element(\"p\")?;
    p.set_text_content(Some(\"Hello from Rust!\"));
    
    body.append_child(&p)?;
    
    Ok(())
}

#[wasm_bindgen]
pub fn update_text(id: &str, text: &str) {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    
    if let Some(element) = document.get_element_by_id(id) {
        element.set_text_content(Some(text));
    }
}
```

**JavaScript**:
```javascript
import { update_text } from './pkg/wasm_demo.js';

update_text('my-element', 'Updated from Rust!');
```

---

## Memory Management

### Ownership Across FFI

When data crosses FFI boundary, ownership becomes crucial.

### Rule 1: Who Allocates, Who Frees

**Rust allocates, Rust frees**:
```rust
#[no_mangle]
pub extern \"C\" fn create_string() -> *mut c_char {
    let s = CString::new(\"Hello\").unwrap();
    s.into_raw()  // Ownership transferred
}

#[no_mangle]
pub extern \"C\" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);  // Takes ownership and frees
        }
    }
}
```

**C usage**:
```c
char* str = create_string();
printf(\"%s\\n\", str);
free_string(str);  // MUST call this!
```

### Rule 2: Validate Pointers

Always check for null:
```rust
#[no_mangle]
pub extern \"C\" fn process(data: *const u8, len: usize) -> i32 {
    if data.is_null() {
        return -1;  // Error code
    }
    
    let slice = unsafe {
        std::slice::from_raw_parts(data, len)
    };
    
    // Process slice...
    0  // Success
}
```

### Rule 3: No Panics Across FFI

Panics across FFI are undefined behavior!

```rust
use std::panic::catch_unwind;

#[no_mangle]
pub extern \"C\" fn safe_divide(a: i32, b: i32, result: *mut i32) -> i32 {
    if result.is_null() {
        return -1;
    }
    
    let computation = catch_unwind(|| {
        if b == 0 {
            panic!(\"Division by zero\");
        }
        a / b
    });
    
    match computation {
        Ok(value) => {
            unsafe { *result = value; }
            0  // Success
        }
        Err(_) => -2,  // Panic occurred
    }
}
```

### Memory Leaks Detection

Use tools to detect leaks:

```bash
# Valgrind (Linux)
valgrind --leak-check=full ./my_program

# AddressSanitizer
RUSTFLAGS=\"-Z sanitizer=address\" cargo build --target x86_64-unknown-linux-gnu
```

---

## Error Handling Across FFI

### Approach 1: Error Codes

```rust
#[repr(C)]
pub enum ErrorCode {
    Success = 0,
    NullPointer = 1,
    InvalidInput = 2,
    OutOfMemory = 3,
}

#[no_mangle]
pub extern \"C\" fn process_data(data: *const u8, len: usize) -> ErrorCode {
    if data.is_null() {
        return ErrorCode::NullPointer;
    }
    
    if len == 0 {
        return ErrorCode::InvalidInput;
    }
    
    // Process data...
    
    ErrorCode::Success
}
```

### Approach 2: Result Struct

```rust
#[repr(C)]
pub struct Result {
    success: bool,
    value: i32,
    error_code: i32,
}

#[no_mangle]
pub extern \"C\" fn divide(a: i32, b: i32) -> Result {
    if b == 0 {
        Result {
            success: false,
            value: 0,
            error_code: 1,
        }
    } else {
        Result {
            success: true,
            value: a / b,
            error_code: 0,
        }
    }
}
```

### Approach 3: Output Parameters

```rust
#[no_mangle]
pub extern \"C\" fn safe_operation(
    input: i32,
    output: *mut i32,
    error: *mut *const c_char
) -> bool {
    if output.is_null() || error.is_null() {
        return false;
    }
    
    if input < 0 {
        unsafe {
            *error = CString::new(\"Negative input\")
                .unwrap()
                .into_raw();
        }
        return false;
    }
    
    unsafe {
        *output = input * 2;
        *error = std::ptr::null();
    }
    true
}
```

---

## Best Practices

### 1. Use Semantic Versioning

```toml
[package]
version = \"1.2.3\"  # MAJOR.MINOR.PATCH
```

### 2. Document Your API

```rust
/// Adds two numbers and returns the result.
///
/// # Safety
///
/// This function is safe to call with any i32 values.
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
#[no_mangle]
pub extern \"C\" fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

### 3. Provide Header Files

Use `cbindgen` to generate headers automatically.

### 4. Write Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
    
    #[test]
    fn test_null_safety() {
        let result = process(std::ptr::null(), 0);
        assert_eq!(result, -1);
    }
}
```

### 5. Version Your ABI

```rust
#[no_mangle]
pub extern \"C\" fn get_version() -> u32 {
    1  // Increment when ABI changes
}
```

### 6. Handle Platform Differences

```rust
#[cfg(target_os = \"windows\")]
fn platform_specific() {
    // Windows implementation
}

#[cfg(target_os = \"linux\")]
fn platform_specific() {
    // Linux implementation
}
```

---

## Performance Considerations

### 1. Minimize FFI Calls

```rust
// BAD: Multiple FFI calls
for i in 0..1000 {
    ffi_function(i);
}

// GOOD: Batch processing
ffi_function_batch(data, 1000);
```

### 2. Use Bulk Operations

```rust
#[no_mangle]
pub extern \"C\" fn process_array(
    input: *const f64,
    output: *mut f64,
    len: usize
) {
    let input_slice = unsafe {
        std::slice::from_raw_parts(input, len)
    };
    let output_slice = unsafe {
        std::slice::from_raw_parts_mut(output, len)
    };
    
    // Process entire array at once
    for (i, &value) in input_slice.iter().enumerate() {
        output_slice[i] = value * 2.0;
    }
}
```

### 3. Optimize Build Settings

```toml
[profile.release]
lto = true
codegen-units = 1
opt-level = 3
```

### 4. Profile Your Code

```bash
# Use profiling tools
cargo install flamegraph
cargo flamegraph
```

---

## Examples in This Module

1. **c_ffi_demo**: Complete C FFI integration example
2. **python_extension_demo**: PyO3-based Python extension
3. **wasm_demo**: WebAssembly for web applications
4. **nodejs_addon_demo**: Neon-based Node.js addon

Each example includes complete source code, build instructions, and usage examples.

---

## Resources

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/)
- [FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [PyO3 User Guide](https://pyo3.rs/)
- [wasm-bindgen Book](https://rustwasm.github.io/wasm-bindgen/)
- [Neon Documentation](https://neon-bindings.com/)
- [cbindgen Documentation](https://github.com/eqrion/cbindgen)
