# Integrating Rust with Existing Applications - Quick Reference

## Integration Approaches

| Approach | Use Case | Complexity | Performance |
|----------|----------|------------|-------------|
| FFI (C bindings) | Any language with C support | Medium | Excellent |
| WebAssembly | JavaScript/Web applications | Low | Good |
| Embedded | Rewrite critical modules | Medium | Excellent |
| CLI Tools | Replace shell scripts | Low | Good |
| Microservices | New services in Rust | Low | Excellent |

## FFI (Foreign Function Interface)

### Rust → C → Other Languages

```rust
// Rust library
#[no_mangle]
pub extern "C" fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Build as C-compatible library
// cargo build --release
```

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

### Common C Types Mapping

| Rust Type | C Type | Notes |
|-----------|--------|-------|
| `i8`, `i16`, `i32`, `i64` | `int8_t`, `int16_t`, `int32_t`, `int64_t` | Signed integers |
| `u8`, `u16`, `u32`, `u64` | `uint8_t`, `uint16_t`, `uint32_t`, `uint64_t` | Unsigned integers |
| `f32`, `f64` | `float`, `double` | Floating point |
| `bool` | `uint8_t` | 0 = false, 1 = true |
| `*const T` | `const T*` | Const pointer |
| `*mut T` | `T*` | Mutable pointer |
| `()` | `void` | No return value |

## Python Integration (PyO3)

### Setup

```toml
[dependencies]
pyo3 = { version = "0.20", features = ["extension-module"] }

[lib]
crate-type = ["cdylib"]
```

### Basic Function

```rust
use pyo3::prelude::*;

#[pyfunction]
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
```

### Build and Use

```bash
# Build
maturin develop

# Use in Python
import my_module
result = my_module.multiply(5, 6)
```

## JavaScript/Node.js Integration (Neon)

### Setup

```bash
npm install -g neon-cli
neon new my-project
```

### Rust Function

```rust
use neon::prelude::*;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("Hello from Rust!"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    Ok(())
}
```

### Use in Node.js

```javascript
const addon = require('./native');
console.log(addon.hello()); // "Hello from Rust!"
```

## WebAssembly (wasm-bindgen)

### Setup

```toml
[dependencies]
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
```

### Rust Code

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

### Build

```bash
# Install wasm-pack
cargo install wasm-pack

# Build
wasm-pack build --target web
```

### Use in JavaScript

```javascript
import init, { greet } from './pkg/my_crate.js';

await init();
console.log(greet('WebAssembly'));
```

## C Integration

### Calling Rust from C

```rust
// lib.rs
#[no_mangle]
pub extern "C" fn process_data(data: *const u8, len: usize) -> i32 {
    // Process data
    0
}
```

```c
// main.c
extern int process_data(const unsigned char* data, size_t len);

int main() {
    unsigned char data[] = {1, 2, 3};
    int result = process_data(data, 3);
    return 0;
}
```

### Build

```bash
# Build Rust library
cargo build --release

# Compile C code
gcc -o app main.c -L./target/release -lmy_lib
```

## Java Integration (JNI)

### Setup

```toml
[dependencies]
jni = "0.21"

[lib]
crate-type = ["cdylib"]
```

### Rust Code

```rust
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jint;

#[no_mangle]
pub extern "system" fn Java_MyClass_add(
    _env: JNIEnv,
    _class: JClass,
    a: jint,
    b: jint,
) -> jint {
    a + b
}
```

### Java Code

```java
public class MyClass {
    static {
        System.loadLibrary("my_rust_lib");
    }
    
    public native int add(int a, int b);
}
```

## Ruby Integration (Rutie)

### Setup

```toml
[dependencies]
rutie = "0.8"

[lib]
crate-type = ["cdylib"]
```

### Rust Code

```rust
use rutie::{Class, Integer, Object, RString};

methods!(
    RString,
    _itself,
    fn uppercase() -> RString {
        RString::new_utf8(&_itself.to_string().to_uppercase())
    }
);

#[no_mangle]
pub extern "C" fn Init_my_gem() {
    Class::from_existing("String").define(|itself| {
        itself.def("rust_uppercase", uppercase);
    });
}
```

## Go Integration (cgo)

### Rust Library

```rust
#[no_mangle]
pub extern "C" fn calculate(x: i32) -> i32 {
    x * 2
}
```

### Go Code

```go
package main

/*
#cgo LDFLAGS: -L./target/release -lmy_lib
extern int calculate(int x);
*/
import "C"
import "fmt"

func main() {
    result := C.calculate(10)
    fmt.Println(result)
}
```

## Common Integration Patterns

### 1. Shared Library Pattern

```toml
[lib]
name = "my_lib"
crate-type = ["cdylib"]  # Dynamic library
```

**Produces**:
- Linux: `libmy_lib.so`
- macOS: `libmy_lib.dylib`
- Windows: `my_lib.dll`

### 2. Static Library Pattern

```toml
[lib]
crate-type = ["staticlib"]  # Static library
```

**Produces**:
- Linux/macOS: `libmy_lib.a`
- Windows: `my_lib.lib`

### 3. Header Generation

Use `cbindgen` to generate C headers:

```bash
cargo install cbindgen
cbindgen --config cbindgen.toml --crate my_lib --output my_lib.h
```

## Memory Safety Guidelines

### String Handling

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn process_string(s: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(s) };
    let r_str = c_str.to_str().unwrap();
    
    let result = format!("Processed: {}", r_str);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            let _ = CString::from_raw(s);
        }
    }
}
```

### Array Handling

```rust
#[no_mangle]
pub extern "C" fn process_array(
    data: *const i32,
    len: usize,
    out: *mut i32
) -> i32 {
    if data.is_null() || out.is_null() {
        return -1;
    }
    
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    let sum: i32 = slice.iter().sum();
    unsafe { *out = sum; }
    0
}
```

### Error Handling

```rust
#[repr(C)]
pub struct Result {
    success: bool,
    value: i32,
    error_code: i32,
}

#[no_mangle]
pub extern "C" fn safe_divide(a: i32, b: i32) -> Result {
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

## Build Configuration

### Cross-Compilation

```bash
# Install target
rustup target add x86_64-pc-windows-gnu

# Build for target
cargo build --target x86_64-pc-windows-gnu --release
```

### Common Targets

- `x86_64-unknown-linux-gnu` - Linux 64-bit
- `x86_64-pc-windows-msvc` - Windows 64-bit (MSVC)
- `x86_64-pc-windows-gnu` - Windows 64-bit (MinGW)
- `x86_64-apple-darwin` - macOS 64-bit (Intel)
- `aarch64-apple-darwin` - macOS ARM64 (Apple Silicon)
- `wasm32-unknown-unknown` - WebAssembly

## Testing Integration

### C Test

```c
#include "my_lib.h"
#include <assert.h>

int main() {
    int result = add(2, 3);
    assert(result == 5);
    return 0;
}
```

### Python Test

```python
import my_module

def test_multiply():
    assert my_module.multiply(3, 4) == 12

if __name__ == "__main__":
    test_multiply()
    print("All tests passed!")
```

## Common Issues & Solutions

| Issue | Solution |
|-------|----------|
| Undefined symbols | Check library path and linking |
| ABI mismatch | Use `extern "C"` |
| Memory leaks | Always free Rust-allocated memory |
| String encoding | Use UTF-8, handle null terminators |
| Panic across FFI | Use `catch_unwind` |

## Panic Safety

```rust
use std::panic::catch_unwind;

#[no_mangle]
pub extern "C" fn safe_function(x: i32) -> i32 {
    match catch_unwind(|| {
        // Code that might panic
        if x < 0 {
            panic!("Negative value!");
        }
        x * 2
    }) {
        Ok(result) => result,
        Err(_) => -1,  // Error indicator
    }
}
```

## Performance Optimization

```toml
[profile.release]
lto = true              # Link-time optimization
codegen-units = 1       # Better optimization
opt-level = 3           # Maximum optimization
strip = true            # Remove debug symbols
```

## Debugging

```bash
# Build with debug info
cargo build

# Use debugger
gdb ./target/debug/my_app
lldb ./target/debug/my_app
```

## Documentation

### Generate C Header

```toml
# cbindgen.toml
language = "C"
include_guard = "MY_LIB_H"
autogen_warning = "/* Warning: This file is auto-generated */"
```

```bash
cbindgen --output my_lib.h
```

## Best Practices

1. **Always use `#[no_mangle]`** for exported functions
2. **Use `extern "C"`** for C ABI compatibility
3. **Handle panics** - don't let them cross FFI boundary
4. **Validate pointers** - check for null before dereferencing
5. **Free memory correctly** - provide cleanup functions
6. **Use repr(C)** for structs shared across FFI
7. **Test thoroughly** - write integration tests
8. **Document ABI** - generate headers and docs

## Useful Tools

| Tool | Purpose |
|------|---------|
| `cbindgen` | Generate C headers |
| `bindgen` | Generate Rust bindings from C |
| `maturin` | Build Python extensions |
| `wasm-pack` | Build WebAssembly packages |
| `neon-cli` | Build Node.js addons |

## Quick Commands

```bash
# Install tools
cargo install cbindgen
cargo install wasm-pack
pip install maturin

# Generate C header
cbindgen --output bindings.h

# Build WebAssembly
wasm-pack build --target web

# Build Python extension
maturin develop

# Build with specific crate type
cargo build --release --lib
```

## Resources

- [The Rustonomicon (Unsafe Rust)](https://doc.rust-lang.org/nomicon/)
- [FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [PyO3 Documentation](https://pyo3.rs/)
- [wasm-bindgen Guide](https://rustwasm.github.io/wasm-bindgen/)
- [Neon Documentation](https://neon-bindings.com/)
