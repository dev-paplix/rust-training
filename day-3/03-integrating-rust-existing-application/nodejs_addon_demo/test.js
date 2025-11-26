#!/usr/bin/env node

const addon = require('./index.js');

console.log('==========================================');
console.log('  Node.js FFI Addon Test Suite');
console.log('  Powered by Rust via FFI');
console.log('==========================================\n');

let passed = 0;
let failed = 0;

function test(name, fn) {
    try {
        fn();
        console.log(`✓ ${name}`);
        passed++;
    } catch (error) {
        console.log(`✗ ${name}`);
        console.error(`  Error: ${error.message}`);
        failed++;
    }
}

function assert(condition, message) {
    if (!condition) {
        throw new Error(message || 'Assertion failed');
    }
}

function assertEquals(actual, expected, message) {
    if (actual !== expected) {
        throw new Error(message || `Expected ${expected}, got ${actual}`);
    }
}

function assertClose(actual, expected, tolerance = 0.001, message) {
    if (Math.abs(actual - expected) > tolerance) {
        throw new Error(message || `Expected ${expected}, got ${actual}`);
    }
}

console.log('Test 1: Basic Functions');
console.log('------------------------');

test('add(5, 3) = 8', () => {
    assertEquals(addon.add(5, 3), 8);
});

test('multiply(6, 7) = 42', () => {
    assertEquals(addon.multiply(6, 7), 42);
});

test('divide(10, 2) = 5', () => {
    assertEquals(addon.divide(10, 2), 5);
});

test('divide by zero returns 0', () => {
    assertEquals(addon.divide(10, 0), 0);
});

test('canDivide(5) = true', () => {
    assertEquals(addon.canDivide(5), true);
});

test('canDivide(0) = false', () => {
    assertEquals(addon.canDivide(0), false);
});

console.log('\nTest 2: String Functions');
console.log('------------------------');

test('greet("Alice")', () => {
    const result = addon.greet('Alice');
    assert(result.includes('Alice'), 'Greeting should include name');
    assert(result.includes('Rust'), 'Greeting should mention Rust');
});

test('toUppercase("hello")', () => {
    assertEquals(addon.toUppercase('hello'), 'HELLO');
});

test('reverseString("Rust")', () => {
    assertEquals(addon.reverseString('Rust'), 'tsuR');
});

console.log('\nTest 3: Computational Functions');
console.log('--------------------------------');

test('factorial(5) = 120', () => {
    // Note: FFI returns i64 which is a BigInt-like number in JS
    const result = addon.factorial(5);
    assertEquals(Number(result), 120);
});

test('factorial(10) = 3628800', () => {
    const result = addon.factorial(10);
    assertEquals(Number(result), 3628800);
});

test('isPrime(2) = true', () => {
    assertEquals(addon.isPrime(2), true);
});

test('isPrime(17) = true', () => {
    assertEquals(addon.isPrime(17), true);
});

test('isPrime(18) = false', () => {
    assertEquals(addon.isPrime(18), false);
});

test('isPrime(0) = false', () => {
    assertEquals(addon.isPrime(0), false);
});

console.log('\nTest 4: Point Distance (JSON)');
console.log('------------------------------');

test('Distance from (0,0) to (3,4) = 5', () => {
    const p1 = { x: 0, y: 0 };
    const p2 = { x: 3, y: 4 };
    const distance = addon.pointDistance(p1, p2);
    assertClose(distance, 5.0);
});

test('Distance from (1,1) to (4,5) = 5', () => {
    const p1 = { x: 1, y: 1 };
    const p2 = { x: 4, y: 5 };
    const distance = addon.pointDistance(p1, p2);
    assertClose(distance, 5.0);
});

console.log('\n==========================================');
console.log(`  Results: ${passed} passed, ${failed} failed`);
console.log('==========================================');

process.exit(failed > 0 ? 1 : 0);

