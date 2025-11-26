# WebAssembly Demo

This project demonstrates how to create WebAssembly modules using Rust and wasm-bindgen that can be used in web browsers.

## Overview

The WASM module provides:

- **Arithmetic functions**: `add()`, `multiply()`, `divide()`, `factorial()`, `is_prime()`
- **String functions**: `greet()`, `to_uppercase()`, `reverse_string()`, `word_count()`, `is_palindrome()`
- **Console logging**: `log()`, `warn()`, `error()`
- **DOM manipulation**: `set_element_text()`, `get_element_text()`, `append_paragraph()`
- **Classes**: `Counter`, `Point`, `Calculator`

## Prerequisites

- Rust toolchain (rustc, cargo)
- wasm-pack: `cargo install wasm-pack`
- A simple HTTP server (e.g., `python -m http.server` or `npx serve`)

## Building

### Install wasm-pack

```bash
cargo install wasm-pack
```

### Build for Web

```bash
wasm-pack build --target web
```

This creates the `pkg/` directory containing:
- `wasm_demo_bg.wasm` - The WebAssembly binary
- `wasm_demo.js` - JavaScript bindings
- `wasm_demo.d.ts` - TypeScript definitions

### Build for Node.js

```bash
wasm-pack build --target nodejs
```

### Build for Bundlers (webpack, rollup, etc.)

```bash
wasm-pack build --target bundler
```

## Running

### Using a Local Server

```bash
# Build the WASM module
wasm-pack build --target web

# Copy the pkg directory to www
# (or symlink: mklink /D www\pkg pkg on Windows)

# Start a local server in the www directory
cd www
python -m http.server 8080
# Or: npx serve
# Or: npx http-server

# Open http://localhost:8080 in your browser
```

### Using PowerShell (Windows)

```powershell
# Build
wasm-pack build --target web

# Copy pkg to www
Copy-Item -Path pkg -Destination www\pkg -Recurse -Force

# Start server
cd www
python -m http.server 8080
```

## Usage in JavaScript

### Basic Functions

```javascript
import init, { add, greet, is_prime } from './pkg/wasm_demo.js';

async function run() {
    await init();
    
    // Arithmetic
    const sum = add(5, 3);           // 8
    const result = divide(10.0, 2.0); // 5.0
    
    // Strings
    const greeting = greet("Alice");  // "Hello, Alice! Welcome from Rust WebAssembly."
    const upper = to_uppercase("hello"); // "HELLO"
    
    // Number functions
    const fact = factorial(5);        // 120
    const prime = is_prime(17);       // true
}

run();
```

### Using Classes

```javascript
import init, { Counter, Point, Calculator } from './pkg/wasm_demo.js';

async function run() {
    await init();
    
    // Counter
    const counter = new Counter();
    counter.increment();
    counter.increment_by(5);
    console.log(counter.value);  // 6
    
    // Point
    const p1 = new Point(0.0, 0.0);
    const p2 = new Point(3.0, 4.0);
    const distance = p1.distance_to(p2);  // 5.0
    const mid = p1.midpoint(p2);          // Point(1.5, 2.0)
    
    // Calculator
    const calc = new Calculator();
    calc.add(10.0);
    calc.multiply(2.0);
    console.log(calc.result);  // 20.0
}

run();
```

### Console Logging

```javascript
import init, { log, warn, error } from './pkg/wasm_demo.js';

async function run() {
    await init();
    
    log("This is a log message");
    warn("This is a warning");
    error("This is an error");
}

run();
```

### DOM Manipulation

```javascript
import init, { set_element_text, append_paragraph } from './pkg/wasm_demo.js';

async function run() {
    await init();
    
    set_element_text("my-div", "Updated from Rust!");
    append_paragraph("This paragraph was created by Rust!");
}

run();
```

## Running Tests

### Rust Unit Tests

```bash
cargo test
```

### WASM Tests in Browser

```bash
wasm-pack test --headless --firefox
# Or: wasm-pack test --headless --chrome
```

## Optimization

The release profile is already optimized for size:

```toml
[profile.release]
opt-level = "s"  # Optimize for size
lto = true       # Enable link-time optimization
```

To further reduce size:

```bash
# Install wasm-opt
npm install -g wasm-opt

# Build with wasm-pack
wasm-pack build --target web --release

# Optimize the WASM binary
wasm-opt -Os -o pkg/wasm_demo_bg_opt.wasm pkg/wasm_demo_bg.wasm
```

## Performance

WebAssembly provides near-native performance. Here's a benchmark example:

```javascript
import init, { factorial } from './pkg/wasm_demo.js';

async function benchmark() {
    await init();
    
    // JavaScript implementation
    function factorialJS(n) {
        let result = 1;
        for (let i = 2; i <= n; i++) {
            result *= i;
        }
        return result;
    }
    
    // Benchmark
    const n = 20;
    const iterations = 100000;
    
    // JavaScript
    console.time('JavaScript');
    for (let i = 0; i < iterations; i++) {
        factorialJS(n);
    }
    console.timeEnd('JavaScript');
    
    // WebAssembly
    console.time('WebAssembly');
    for (let i = 0; i < iterations; i++) {
        factorial(n);
    }
    console.timeEnd('WebAssembly');
}

benchmark();
```

## Project Structure

```
wasm_demo/
├── Cargo.toml              # Rust configuration
├── src/
│   └── lib.rs              # Rust source code
├── pkg/                    # Generated by wasm-pack
│   ├── wasm_demo_bg.wasm
│   ├── wasm_demo.js
│   └── wasm_demo.d.ts
└── www/                    # Web demo
    └── index.html          # Interactive demo page
```

## Using in Production

### With a Bundler (webpack, vite, etc.)

```bash
# Build for bundler
wasm-pack build --target bundler

# In your JavaScript
import init, { add } from 'wasm-demo';

async function run() {
    await init();
    console.log(add(1, 2));
}

run();
```

### Publishing to npm

```bash
# Build the package
wasm-pack build --target bundler

# Login to npm
wasm-pack login

# Publish
wasm-pack publish
```

## Common Issues

### Module Not Found

Make sure to:
1. Build the WASM module first: `wasm-pack build --target web`
2. Serve the files via HTTP (not `file://`)
3. Check that the import path is correct

### CORS Errors

Always serve WASM files via HTTP server, not directly from the filesystem.

### Memory Issues

For large data processing:
- Pass data by reference when possible
- Use typed arrays for efficient data transfer
- Consider streaming APIs for large datasets

## TypeScript Support

The generated `pkg/wasm_demo.d.ts` provides full TypeScript support:

```typescript
import init, { Counter, Point } from './pkg/wasm_demo';

async function run() {
    await init();
    
    const counter: Counter = new Counter();
    counter.increment();
    
    const p1: Point = new Point(0, 0);
    const p2: Point = new Point(3, 4);
    const distance: number = p1.distance_to(p2);
}

run();
```

## Browser Compatibility

WebAssembly is supported in:
- Chrome/Edge 57+
- Firefox 52+
- Safari 11+
- Opera 44+

For older browsers, consider using a polyfill or fallback to JavaScript.

## Learning Resources

- [Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/)
- [wasm-bindgen Guide](https://rustwasm.github.io/docs/wasm-bindgen/)
- [wasm-pack Documentation](https://rustwasm.github.io/docs/wasm-pack/)
- [WebAssembly MDN](https://developer.mozilla.org/en-US/docs/WebAssembly)

## License

This example is provided for educational purposes.
