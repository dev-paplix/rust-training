# Node.js FFI Addon Demo

This project demonstrates how to integrate Rust with Node.js using simple FFI (Foreign Function Interface) instead of complex native bindings.

## Overview

The addon provides:

- **Basic functions**: `add()`, `multiply()`, `divide()`, `canDivide()`
- **String functions**: `greet()`, `toUppercase()`, `reverseString()`
- **Computational functions**: `factorial()`, `isPrime()`
- **Complex data**: `pointDistance()` using JSON for data exchange

## Why FFI Instead of Neon?

This demo uses `ffi-napi` instead of Neon because:

1. **Simplicity**: No complex N-API bindings or build configurations
2. **Portability**: Standard C FFI works across platforms
3. **Learning**: Demonstrates fundamental FFI concepts
4. **Flexibility**: Easy to extend with new functions
5. **Transparency**: Clear separation between Rust and JavaScript code

## Prerequisites

- Rust toolchain (rustc, cargo)
- Node.js 14+ and npm

## Installation

### Install Dependencies

```bash
npm install
```

This installs `ffi-napi` and `ref-napi` for calling Rust functions from JavaScript.

### Build the Rust Library

```bash
npm run build
```

This compiles the Rust code into a shared library (`mathlib.dll` on Windows, `libmathlib.so` on Linux, `libmathlib.dylib` on macOS).

## Usage

### Basic Functions

```javascript
const addon = require('./index.js');

// Arithmetic
console.log(addon.add(5, 3));        // 8
console.log(addon.multiply(6, 7));   // 42
console.log(addon.divide(10, 2));    // 5
console.log(addon.divide(10, 0));    // 0 (returns 0 for division by zero)

// Check if division is safe
console.log(addon.canDivide(5));     // true
console.log(addon.canDivide(0));     // false
```

### String Functions

```javascript
const addon = require('./index.js');

console.log(addon.greet('Alice'));           // "Hello, Alice! Welcome from Rust."
console.log(addon.toUppercase('hello'));     // "HELLO"
console.log(addon.reverseString('Rust'));    // "tsuR"
```

### Computational Functions

```javascript
const addon = require('./index.js');

console.log(addon.factorial(5));    // 120
console.log(addon.factorial(10));   // 3628800
console.log(addon.isPrime(17));     // true
console.log(addon.isPrime(18));     // false
```

### Complex Data (JSON Exchange)

```javascript
const addon = require('./index.js');

const p1 = { x: 0, y: 0 };
const p2 = { x: 3, y: 4 };
console.log(addon.pointDistance(p1, p2));  // 5.0
```


### Counter Class

```javascript

## Running Tests

```bash
npm test
```

Or run directly:

```bash
node test.js
```

## Memory Management

### String Handling

The addon automatically manages string memory:

```javascript
// Strings are automatically allocated and freed
const greeting = addon.greet('Alice');
// No manual cleanup needed!
```

Under the hood, `index.js` calls `free_string()` after reading the result:

```javascript
function callStringFunction(fnName, ...args) {
    const ptr = mathlib[fnName](...args);
    const result = ref.readCString(ptr, 0);
    mathlib.free_string(ptr);  // Automatic cleanup
    return result;
}
```

## Performance

FFI adds minimal overhead for simple function calls. For computationally intensive tasks, Rust provides significant performance improvements:

```javascript
const addon = require('./index.js');

console.time('factorial(20)');
addon.factorial(20);
console.timeEnd('factorial(20)');
// Typically < 1ms
```

## Project Structure

```
nodejs_addon_demo/
├── Cargo.toml          # Rust configuration (cdylib)
├── package.json        # Node.js dependencies (ffi-napi, ref-napi)
├── index.js            # FFI bindings and wrappers
├── test.js             # Test suite
├── src/
│   └── lib.rs          # Rust implementation
└── target/
    └── release/
        └── mathlib.dll/so/dylib  # Compiled library
```

## How It Works

### 1. Rust Side (lib.rs)

Functions are exported with C ABI:

```rust
#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}
```

### 2. JavaScript Side (index.js)

FFI loads the library and defines function signatures:

```javascript
const mathlib = ffi.Library(libPath, {
    'add': ['int', ['int', 'int']],
    // ... more functions
});
```

### 3. Usage

Simple wrapper functions provide a clean API:

```javascript
module.exports = {
    add: (a, b) => mathlib.add(a, b),
    // ... more exports
};
```

## Development

### Rebuilding After Changes

```bash
npm run build
```

### Testing Changes

```bash
cargo test          # Test Rust code
npm test            # Test full integration
```

## Common Issues

### "Cannot find module"

Make sure to build first:
```bash
npm run build
```

### Platform-Specific Library Names

The addon automatically detects the platform:
- Windows: `mathlib.dll`
- Linux: `libmathlib.so`
- macOS: `libmathlib.dylib`

### FFI Dependencies Not Installing

```bash
npm install ffi-napi ref-napi
```

## Extending the Addon

### Adding a New Function

1. **Add Rust function** in `src/lib.rs`:

```rust
#[no_mangle]
pub extern "C" fn square(n: c_int) -> c_int {
    n * n
}
```

2. **Define FFI signature** in `index.js`:

```javascript
const mathlib = ffi.Library(libPath, {
    // ... existing functions
    'square': ['int', ['int']],
});
```

3. **Export wrapper** in `index.js`:

```javascript
module.exports = {
    // ... existing exports
    square: (n) => mathlib.square(n),
};
```

4. **Test it**:

```javascript
const addon = require('./index.js');
console.log(addon.square(5));  // 25
```

## TypeScript Support

Create `index.d.ts`:

```typescript
export function add(a: number, b: number): number;
export function multiply(a: number, b: number): number;
export function divide(a: number, b: number): number;
export function canDivide(b: number): boolean;

export function greet(name: string): string;
export function toUppercase(text: string): string;
export function reverseString(text: string): string;

export function factorial(n: number): number;
export function isPrime(n: number): boolean;

export interface Point {
    x: number;
    y: number;
}

export function pointDistance(p1: Point, p2: Point): number;
```

## Comparison with Neon

| Aspect | FFI (This Demo) | Neon |
|--------|----------------|------|
| Complexity | Low | High |
| Build Tools | Standard cargo | cargo-cp-artifact, N-API |
| Learning Curve | Gentle | Steep |
| Type Conversion | Manual | Automatic |
| Performance | Excellent | Excellent |
| Objects/Classes | JSON exchange | Native N-API objects |
| Error Handling | Return codes | JavaScript exceptions |

## When to Use FFI vs Neon

**Use FFI when:**
- You want simplicity
- You're learning Rust/Node.js integration
- You have simple data types (numbers, strings)
- You don't need complex object lifetimes

**Use Neon when:**
- You need native JavaScript objects
- You want automatic type conversions
- You need complex class hierarchies
- You want exception-based error handling

## Learning Resources

- [node-ffi-napi Documentation](https://github.com/node-ffi-napi/node-ffi-napi)
- [Rust FFI Guide](https://doc.rust-lang.org/nomicon/ffi.html)
- [C FFI Examples](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code)

## License

This example is provided for educational purposes (MIT).

