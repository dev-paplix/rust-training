# How Rust WebAssembly Connects to the Frontend

## 🔄 The Complete Flow

```
┌─────────────────────────────────────────────────────────────────────┐
│                         1. RUST SOURCE CODE                         │
│                         (src/lib.rs)                                │
├─────────────────────────────────────────────────────────────────────┤
│  #[wasm_bindgen]                                                    │
│  pub fn add(a: i32, b: i32) -> i32 {                               │
│      a + b                                                          │
│  }                                                                  │
│                                                                     │
│  #[wasm_bindgen]                                                    │
│  pub struct Counter { value: i32 }                                 │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
                    wasm-pack build --target web
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                    2. COMPILED WASM + JS GLUE                       │
│                         (pkg/ folder)                               │
├─────────────────────────────────────────────────────────────────────┤
│  wasm_demo_bg.wasm     ← Binary WebAssembly code                   │
│  wasm_demo.js          ← JavaScript wrapper/glue code              │
│  wasm_demo.d.ts        ← TypeScript definitions                    │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
                    import in HTML/JavaScript
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                    3. JAVASCRIPT IMPORTS                            │
│                      (www/index.html)                               │
├─────────────────────────────────────────────────────────────────────┤
│  <script type="module">                                             │
│    import init, * as wasm from './pkg/wasm_demo.js';               │
│                                                                     │
│    async function run() {                                           │
│      await init();  // Loads the .wasm file                        │
│      const result = wasm.add(5, 3);  // Call Rust function!        │
│    }                                                                │
│  </script>                                                          │
└─────────────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────────────┐
│                    4. HTML BUTTON CLICKS                            │
│                      (User Interaction)                             │
├─────────────────────────────────────────────────────────────────────┤
│  <button onclick="doAdd()">Add</button>                             │
│                                                                     │
│  window.doAdd = function() {                                        │
│    const a = parseInt(document.getElementById('num1').value);      │
│    const b = parseInt(document.getElementById('num2').value);      │
│    const result = wasm.add(a, b);  // ← Calls Rust!                │
│    showResult(`${a} + ${b} = ${result}`);                          │
│  }                                                                  │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 📝 Step-by-Step Explanation

### Step 1: Write Rust Code with `#[wasm_bindgen]`

**File: `src/lib.rs`**

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}
```

**Key Points:**
- `#[wasm_bindgen]` = "Make this available to JavaScript"
- It works on functions, structs, and impl blocks
- Automatically handles type conversions (Rust ↔ JavaScript)

---

### Step 2: Build with `wasm-pack`

**Command:**
```bash
wasm-pack build --target web
```

**This creates the `pkg/` folder with:**

#### `pkg/wasm_demo_bg.wasm` (Binary)
- Compiled Rust code in WebAssembly format
- Not human-readable
- Efficient binary format for browsers

#### `pkg/wasm_demo.js` (JavaScript Wrapper)
- **Auto-generated** by wasm-bindgen
- Bridges JavaScript ↔ WebAssembly

**Example of generated code:**

```javascript
// Auto-generated wrapper for your Rust add() function
export function add(a, b) {
    const ret = wasm.add(a, b);  // Calls the WASM function
    return ret;
}

// Auto-generated wrapper for your Rust greet() function
export function greet(name) {
    const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.greet(ptr0, len0);
    return takeString(ret);
}

// Auto-generated wrapper for your Counter class
export class Counter {
    constructor() {
        const ret = wasm.counter_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    
    increment() {
        wasm.counter_increment(this.__wbg_ptr);
    }
    
    get value() {
        const ret = wasm.counter_value(this.__wbg_ptr);
        return ret;
    }
}

// Initialization function
export default async function init(input) {
    const imports = wbgImports;
    
    if (typeof input === 'string' || input instanceof URL) {
        input = fetch(input);
    }
    
    const { instance, module } = await load(await input, imports);
    
    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    
    return wasm;
}
```

---

### Step 3: Import in HTML

**File: `www/index.html`**

```html
<!DOCTYPE html>
<html>
<head>
    <title>Rust WASM Demo</title>
</head>
<body>
    <h1>WebAssembly Demo</h1>
    
    <!-- Input fields -->
    <input type="number" id="num1" value="5">
    <input type="number" id="num2" value="3">
    <button onclick="doAdd()">Add</button>
    <div id="result"></div>

    <script type="module">
        // STEP 1: Import the WASM module
        import init, * as wasm from './pkg/wasm_demo.js';
        
        // STEP 2: Initialize (loads the .wasm file)
        async function run() {
            await init();  // Downloads and compiles wasm_demo_bg.wasm
            
            console.log('✅ WASM loaded!');
            
            // STEP 3: Make wasm functions available globally
            window.wasm = wasm;
        }
        
        // STEP 4: Call from button click
        window.doAdd = function() {
            const a = parseInt(document.getElementById('num1').value);
            const b = parseInt(document.getElementById('num2').value);
            
            // This calls the Rust function via the JavaScript wrapper!
            const result = wasm.add(a, b);
            
            document.getElementById('result').textContent = 
                `${a} + ${b} = ${result}`;
        }
        
        run();
    </script>
</body>
</html>
```

---

## 🔍 What Happens When You Click a Button?

### Example: User clicks "Add" button

```
User clicks button
       ↓
HTML button onclick="doAdd()"
       ↓
JavaScript function doAdd() executes
       ↓
Calls wasm.add(5, 3)
       ↓
pkg/wasm_demo.js wrapper receives call
       ↓
Converts JavaScript numbers to WASM format
       ↓
Calls actual WASM function in wasm_demo_bg.wasm
       ↓
Rust code executes (a + b)
       ↓
Result returned to JavaScript wrapper
       ↓
Converted back to JavaScript number
       ↓
Displayed in the webpage: "5 + 3 = 8"
```

---

## 🎯 Key Connections

### 1. **Rust → WASM**
```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```
↓ (compiled by wasm-pack)
```
Binary WASM function in wasm_demo_bg.wasm
```

### 2. **WASM → JavaScript**
Auto-generated `pkg/wasm_demo.js`:
```javascript
export function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}
```

### 3. **JavaScript → HTML**
```html
<script type="module">
  import init, * as wasm from './pkg/wasm_demo.js';
  await init();
  const result = wasm.add(5, 3);
</script>
```

### 4. **HTML → User**
```html
<button onclick="wasm.add(5, 3)">Calculate</button>
```

---

## 📦 File Structure

```
wasm_demo/
├── src/
│   └── lib.rs              ← YOUR RUST CODE
├── www/
│   └── index.html          ← YOUR HTML/JAVASCRIPT
└── pkg/                    ← AUTO-GENERATED (don't edit!)
    ├── wasm_demo_bg.wasm   ← Compiled binary
    ├── wasm_demo.js        ← JavaScript wrapper
    └── wasm_demo.d.ts      ← TypeScript types
```

---

## 🧪 Example Flow: Counter Class

### 1. Rust Code (src/lib.rs)
```rust
#[wasm_bindgen]
pub struct Counter {
    value: i32,
}

#[wasm_bindgen]
impl Counter {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {
        self.value += 1;
    }
    
    #[wasm_bindgen(getter)]
    pub fn value(&self) -> i32 {
        self.value
    }
}
```

### 2. Generated JavaScript (pkg/wasm_demo.js)
```javascript
export class Counter {
    constructor() {
        const ret = wasm.counter_new();
        this.__wbg_ptr = ret >>> 0;
        return this;
    }
    
    increment() {
        wasm.counter_increment(this.__wbg_ptr);
    }
    
    get value() {
        const ret = wasm.counter_value(this.__wbg_ptr);
        return ret;
    }
}
```

### 3. HTML Usage (www/index.html)
```html
<script type="module">
  import init, * as wasm from './pkg/wasm_demo.js';
  
  await init();
  
  const counter = new wasm.Counter();  // Create Rust object!
  counter.increment();                  // Call Rust method!
  console.log(counter.value);           // Access Rust property! → 1
</script>
```

---

## 🚀 How to Run

1. **Build the WASM:**
   ```bash
   cd wasm_demo
   wasm-pack build --target web
   ```

2. **Serve the HTML:**
   ```bash
   cd www
   python -m http.server 8080
   # or
   npx serve
   ```

3. **Open browser:**
   ```
   http://localhost:8080
   ```

4. **Open DevTools (F12)** to see:
   - Network tab: `wasm_demo_bg.wasm` loading
   - Console: Your Rust functions being called!

---

## 🔧 Type Conversions (Automatic!)

| Rust Type | JavaScript Type |
|-----------|----------------|
| `i32`, `u32` | `number` |
| `f64` | `number` |
| `bool` | `boolean` |
| `String`, `&str` | `string` |
| `Vec<T>` | `Array` |
| `struct` | `class` |
| `Option<T>` | `null` or value |
| `Result<T, E>` | Exception on error |

**wasm-bindgen handles ALL of this automatically!**

---

## 💡 Summary

1. **Write Rust** with `#[wasm_bindgen]` annotations
2. **Run `wasm-pack build`** to compile to WASM + generate JS glue
3. **Import in HTML** using `import * as wasm from './pkg/wasm_demo.js'`
4. **Call Rust functions** from JavaScript like normal JS functions!

The magic is the **auto-generated JavaScript wrapper** (`pkg/wasm_demo.js`) that bridges the gap between JavaScript and WebAssembly binary code.

**You write Rust, the browser runs it as if it were JavaScript! 🎉**
