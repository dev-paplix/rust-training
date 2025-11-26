const ffi = require('ffi-napi');
const ref = require('ref-napi');
const path = require('path');
const os = require('os');

// Determine the library path based on platform
function getLibraryPath() {
    const platform = os.platform();
    const libDir = path.join(__dirname, 'target', 'release');
    
    if (platform === 'win32') {
        return path.join(libDir, 'mathlib.dll');
    } else if (platform === 'darwin') {
        return path.join(libDir, 'libmathlib.dylib');
    } else {
        return path.join(libDir, 'libmathlib.so');
    }
}

// Define types
const int = 'int';
const double = 'double';
const bool = 'bool';
const int64 = 'int64';
const string = 'string';
const charPtr = ref.refType('char');

// Load the Rust library
const libPath = getLibraryPath();
const mathlib = ffi.Library(libPath, {
    // Basic numeric functions
    'add': [int, [int, int]],
    'multiply': [int, [int, int]],
    'divide': [double, [double, double]],
    'can_divide': [bool, [double]],
    
    // String functions (return allocated strings that must be freed)
    'greet': [charPtr, [string]],
    'to_uppercase': [charPtr, [string]],
    'reverse_string': [charPtr, [string]],
    'free_string': ['void', [charPtr]],
    
    // Computational functions
    'factorial': [int64, [int]],
    'is_prime': [bool, [int]],
    
    // JSON-based data exchange
    'point_distance': [double, [string]]
});

// Helper function to call string functions and automatically free memory
function callStringFunction(fnName, ...args) {
    const ptr = mathlib[fnName](...args);
    if (ptr.isNull()) {
        return null;
    }
    const result = ref.readCString(ptr, 0);
    mathlib.free_string(ptr);
    return result;
}

// Export wrapper functions
module.exports = {
    // Basic functions
    add: (a, b) => mathlib.add(a, b),
    multiply: (a, b) => mathlib.multiply(a, b),
    divide: (a, b) => mathlib.divide(a, b),
    canDivide: (b) => mathlib.can_divide(b),
    
    // String functions
    greet: (name) => callStringFunction('greet', name),
    toUppercase: (text) => callStringFunction('to_uppercase', text),
    reverseString: (text) => callStringFunction('reverse_string', text),
    
    // Computational functions
    factorial: (n) => mathlib.factorial(n),
    isPrime: (n) => mathlib.is_prime(n),
    
    // Point distance calculation
    pointDistance: (p1, p2) => {
        const json = JSON.stringify([p1, p2]);
        return mathlib.point_distance(json);
    }
};

