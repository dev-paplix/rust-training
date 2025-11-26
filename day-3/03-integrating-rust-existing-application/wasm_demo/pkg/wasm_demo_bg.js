let wasm;
export function __wbg_set_wasm(val) {
    wasm = val;
}


let cachedUint8ArrayMemory0 = null;

function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        const idx = addToExternrefTable0(e);
        wasm.__wbindgen_exn_store(idx);
    }
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let WASM_VECTOR_LEN = 0;

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    }
}

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedDataViewMemory0 = null;

function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}
/**
 * Multiplies two numbers.
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function multiply(a, b) {
    const ret = wasm.multiply(a, b);
    return ret;
}

/**
 * Divides two numbers, returning an Option.
 * @param {number} a
 * @param {number} b
 * @returns {number | undefined}
 */
export function divide(a, b) {
    const ret = wasm.divide(a, b);
    return ret[0] === 0 ? undefined : ret[1];
}

/**
 * Initializes the WASM module and logs a greeting.
 * This is automatically called when the module is loaded.
 */
export function init() {
    wasm.init();
}

/**
 * Logs a message to the browser console.
 * @param {string} message
 */
export function log(message) {
    const ptr0 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.log(ptr0, len0);
}

/**
 * Adds two numbers together.
 *
 * # Example (JavaScript)
 *
 * ```javascript
 * import { add } from './wasm_demo';
 * const result = add(5, 3); // 8
 * ```
 * @param {number} a
 * @param {number} b
 * @returns {number}
 */
export function add(a, b) {
    const ret = wasm.add(a, b);
    return ret;
}

/**
 * Creates a new paragraph element with the given text and appends it to the body.
 *
 * Returns true if successful, false otherwise.
 * @param {string} text
 * @returns {boolean}
 */
export function append_paragraph(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.append_paragraph(ptr0, len0);
    return ret !== 0;
}

/**
 * Reverses a string.
 * @param {string} text
 * @returns {string}
 */
export function reverse_string(text) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.reverse_string(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Calculates the factorial of a number.
 * @param {number} n
 * @returns {bigint}
 */
export function factorial(n) {
    const ret = wasm.factorial(n);
    return BigInt.asUintN(64, ret);
}

/**
 * Logs a warning to the browser console.
 * @param {string} message
 */
export function warn(message) {
    const ptr0 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.warn(ptr0, len0);
}

/**
 * Logs an error to the browser console.
 * @param {string} message
 */
export function error(message) {
    const ptr0 = passStringToWasm0(message, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    wasm.error(ptr0, len0);
}

/**
 * Checks if a number is prime.
 * @param {number} n
 * @returns {boolean}
 */
export function is_prime(n) {
    const ret = wasm.is_prime(n);
    return ret !== 0;
}

/**
 * Checks if a string is a palindrome.
 * @param {string} text
 * @returns {boolean}
 */
export function is_palindrome(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.is_palindrome(ptr0, len0);
    return ret !== 0;
}

/**
 * Sets the text content of an element by ID.
 *
 * Returns true if successful, false otherwise.
 * @param {string} id
 * @param {string} text
 * @returns {boolean}
 */
export function set_element_text(id, text) {
    const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ptr1 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len1 = WASM_VECTOR_LEN;
    const ret = wasm.set_element_text(ptr0, len0, ptr1, len1);
    return ret !== 0;
}

/**
 * Converts a string to uppercase.
 * @param {string} text
 * @returns {string}
 */
export function to_uppercase(text) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.to_uppercase(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Counts the number of words in a text.
 * @param {string} text
 * @returns {number}
 */
export function word_count(text) {
    const ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.word_count(ptr0, len0);
    return ret >>> 0;
}

/**
 * Gets the text content of an element by ID.
 *
 * Returns the text content or an empty string if not found.
 * @param {string} id
 * @returns {string}
 */
export function get_element_text(id) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.get_element_text(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

/**
 * Greets a person by name.
 * @param {string} name
 * @returns {string}
 */
export function greet(name) {
    let deferred2_0;
    let deferred2_1;
    try {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.greet(ptr0, len0);
        deferred2_0 = ret[0];
        deferred2_1 = ret[1];
        return getStringFromWasm0(ret[0], ret[1]);
    } finally {
        wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
    }
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

const CalculatorFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_calculator_free(ptr >>> 0, 1));
/**
 * A calculator that maintains a running total.
 */
export class Calculator {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CalculatorFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_calculator_free(ptr, 0);
    }
    /**
     * Adds a number to the current value.
     * @param {number} n
     */
    add(n) {
        wasm.calculator_add(this.__wbg_ptr, n);
    }
    /**
     * Creates a new calculator starting at 0.
     */
    constructor() {
        const ret = wasm.calculator_new();
        this.__wbg_ptr = ret >>> 0;
        CalculatorFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Resets the calculator to 0.
     */
    reset() {
        wasm.calculator_reset(this.__wbg_ptr);
    }
    /**
     * Divides the current value by a number.
     * Returns false if dividing by zero.
     * @param {number} n
     * @returns {boolean}
     */
    divide(n) {
        const ret = wasm.calculator_divide(this.__wbg_ptr, n);
        return ret !== 0;
    }
    /**
     * Gets the current result.
     * @returns {number}
     */
    get result() {
        const ret = wasm.calculator_result(this.__wbg_ptr);
        return ret;
    }
    /**
     * Multiplies the current value by a number.
     * @param {number} n
     */
    multiply(n) {
        wasm.calculator_multiply(this.__wbg_ptr, n);
    }
    /**
     * Subtracts a number from the current value.
     * @param {number} n
     */
    subtract(n) {
        wasm.calculator_subtract(this.__wbg_ptr, n);
    }
}
if (Symbol.dispose) Calculator.prototype[Symbol.dispose] = Calculator.prototype.free;

const CounterFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_counter_free(ptr >>> 0, 1));
/**
 * A counter that can be incremented and decremented.
 */
export class Counter {

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        CounterFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_counter_free(ptr, 0);
    }
    /**
     * Increments the counter by a specific amount.
     * @param {number} amount
     */
    increment_by(amount) {
        wasm.counter_increment_by(this.__wbg_ptr, amount);
    }
    /**
     * Creates a new counter starting at 0.
     */
    constructor() {
        const ret = wasm.counter_new();
        this.__wbg_ptr = ret >>> 0;
        CounterFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Resets the counter to 0.
     */
    reset() {
        wasm.counter_reset(this.__wbg_ptr);
    }
    /**
     * Gets the current counter value.
     * @returns {number}
     */
    get value() {
        const ret = wasm.counter_value(this.__wbg_ptr);
        return ret;
    }
    /**
     * Decrements the counter by 1.
     */
    decrement() {
        wasm.counter_decrement(this.__wbg_ptr);
    }
    /**
     * Increments the counter by 1.
     */
    increment() {
        wasm.counter_increment(this.__wbg_ptr);
    }
}
if (Symbol.dispose) Counter.prototype[Symbol.dispose] = Counter.prototype.free;

const PointFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_point_free(ptr >>> 0, 1));
/**
 * A 2D point with x and y coordinates.
 */
export class Point {

    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Point.prototype);
        obj.__wbg_ptr = ptr;
        PointFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }

    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        PointFinalization.unregister(this);
        return ptr;
    }

    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_point_free(ptr, 0);
    }
    /**
     * Calculates the distance to another point.
     * @param {Point} other
     * @returns {number}
     */
    distance_to(other) {
        _assertClass(other, Point);
        const ret = wasm.point_distance_to(this.__wbg_ptr, other.__wbg_ptr);
        return ret;
    }
    /**
     * Gets the x coordinate.
     * @returns {number}
     */
    get x() {
        const ret = wasm.point_x(this.__wbg_ptr);
        return ret;
    }
    /**
     * Gets the y coordinate.
     * @returns {number}
     */
    get y() {
        const ret = wasm.point_y(this.__wbg_ptr);
        return ret;
    }
    /**
     * Creates a new point.
     * @param {number} x
     * @param {number} y
     */
    constructor(x, y) {
        const ret = wasm.point_new(x, y);
        this.__wbg_ptr = ret >>> 0;
        PointFinalization.register(this, this.__wbg_ptr, this);
        return this;
    }
    /**
     * Sets the x coordinate.
     * @param {number} x
     */
    set x(x) {
        wasm.point_set_x(this.__wbg_ptr, x);
    }
    /**
     * Sets the y coordinate.
     * @param {number} y
     */
    set y(y) {
        wasm.point_set_y(this.__wbg_ptr, y);
    }
    /**
     * Calculates the midpoint with another point.
     * @param {Point} other
     * @returns {Point}
     */
    midpoint(other) {
        _assertClass(other, Point);
        const ret = wasm.point_midpoint(this.__wbg_ptr, other.__wbg_ptr);
        return Point.__wrap(ret);
    }
    /**
     * Returns a string representation of the point.
     * @returns {string}
     */
    to_string() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.point_to_string(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * Moves the point by the given offsets.
     * @param {number} dx
     * @param {number} dy
     */
    translate(dx, dy) {
        wasm.point_translate(this.__wbg_ptr, dx, dy);
    }
}
if (Symbol.dispose) Point.prototype[Symbol.dispose] = Point.prototype.free;

export function __wbg___wbindgen_is_undefined_2d472862bd29a478(arg0) {
    const ret = arg0 === undefined;
    return ret;
};

export function __wbg___wbindgen_throw_b855445ff6a94295(arg0, arg1) {
    throw new Error(getStringFromWasm0(arg0, arg1));
};

export function __wbg_appendChild_aec7a8a4bd6cac61() { return handleError(function (arg0, arg1) {
    const ret = arg0.appendChild(arg1);
    return ret;
}, arguments) };

export function __wbg_body_8c26b54829a0c4cb(arg0) {
    const ret = arg0.body;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_call_e762c39fa8ea36bf() { return handleError(function (arg0, arg1) {
    const ret = arg0.call(arg1);
    return ret;
}, arguments) };

export function __wbg_createElement_964ab674a0176cd8() { return handleError(function (arg0, arg1, arg2) {
    const ret = arg0.createElement(getStringFromWasm0(arg1, arg2));
    return ret;
}, arguments) };

export function __wbg_document_725ae06eb442a6db(arg0) {
    const ret = arg0.document;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_error_a7f8fbb0523dae15(arg0) {
    console.error(arg0);
};

export function __wbg_getElementById_c365dd703c4a88c3(arg0, arg1, arg2) {
    const ret = arg0.getElementById(getStringFromWasm0(arg1, arg2));
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_instanceof_Window_4846dbb3de56c84c(arg0) {
    let result;
    try {
        result = arg0 instanceof Window;
    } catch (_) {
        result = false;
    }
    const ret = result;
    return ret;
};

export function __wbg_log_8cec76766b8c0e33(arg0) {
    console.log(arg0);
};

export function __wbg_new_no_args_ee98eee5275000a4(arg0, arg1) {
    const ret = new Function(getStringFromWasm0(arg0, arg1));
    return ret;
};

export function __wbg_set_textContent_12af0b0f84feb710(arg0, arg1, arg2) {
    arg0.textContent = arg1 === 0 ? undefined : getStringFromWasm0(arg1, arg2);
};

export function __wbg_static_accessor_GLOBAL_89e1d9ac6a1b250e() {
    const ret = typeof global === 'undefined' ? null : global;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_GLOBAL_THIS_8b530f326a9e48ac() {
    const ret = typeof globalThis === 'undefined' ? null : globalThis;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_SELF_6fdf4b64710cc91b() {
    const ret = typeof self === 'undefined' ? null : self;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_static_accessor_WINDOW_b45bfc5a37f6cfa2() {
    const ret = typeof window === 'undefined' ? null : window;
    return isLikeNone(ret) ? 0 : addToExternrefTable0(ret);
};

export function __wbg_textContent_5f62e83b3244a091(arg0, arg1) {
    const ret = arg1.textContent;
    var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len1 = WASM_VECTOR_LEN;
    getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
    getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
};

export function __wbg_warn_1d74dddbe2fd1dbb(arg0) {
    console.warn(arg0);
};

export function __wbindgen_cast_2241b6af4c4b2941(arg0, arg1) {
    // Cast intrinsic for `Ref(String) -> Externref`.
    const ret = getStringFromWasm0(arg0, arg1);
    return ret;
};

export function __wbindgen_init_externref_table() {
    const table = wasm.__wbindgen_externrefs;
    const offset = table.grow(4);
    table.set(0, undefined);
    table.set(offset + 0, undefined);
    table.set(offset + 1, null);
    table.set(offset + 2, true);
    table.set(offset + 3, false);
    ;
};

