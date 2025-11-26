/* tslint:disable */
/* eslint-disable */
/**
 * Multiplies two numbers.
 */
export function multiply(a: number, b: number): number;
/**
 * Divides two numbers, returning an Option.
 */
export function divide(a: number, b: number): number | undefined;
/**
 * Initializes the WASM module and logs a greeting.
 * This is automatically called when the module is loaded.
 */
export function init(): void;
/**
 * Logs a message to the browser console.
 */
export function log(message: string): void;
/**
 * Adds two numbers together.
 *
 * # Example (JavaScript)
 *
 * ```javascript
 * import { add } from './wasm_demo';
 * const result = add(5, 3); // 8
 * ```
 */
export function add(a: number, b: number): number;
/**
 * Creates a new paragraph element with the given text and appends it to the body.
 *
 * Returns true if successful, false otherwise.
 */
export function append_paragraph(text: string): boolean;
/**
 * Reverses a string.
 */
export function reverse_string(text: string): string;
/**
 * Calculates the factorial of a number.
 */
export function factorial(n: number): bigint;
/**
 * Logs a warning to the browser console.
 */
export function warn(message: string): void;
/**
 * Logs an error to the browser console.
 */
export function error(message: string): void;
/**
 * Checks if a number is prime.
 */
export function is_prime(n: number): boolean;
/**
 * Checks if a string is a palindrome.
 */
export function is_palindrome(text: string): boolean;
/**
 * Sets the text content of an element by ID.
 *
 * Returns true if successful, false otherwise.
 */
export function set_element_text(id: string, text: string): boolean;
/**
 * Converts a string to uppercase.
 */
export function to_uppercase(text: string): string;
/**
 * Counts the number of words in a text.
 */
export function word_count(text: string): number;
/**
 * Gets the text content of an element by ID.
 *
 * Returns the text content or an empty string if not found.
 */
export function get_element_text(id: string): string;
/**
 * Greets a person by name.
 */
export function greet(name: string): string;
/**
 * A calculator that maintains a running total.
 */
export class Calculator {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Adds a number to the current value.
   */
  add(n: number): void;
  /**
   * Creates a new calculator starting at 0.
   */
  constructor();
  /**
   * Resets the calculator to 0.
   */
  reset(): void;
  /**
   * Divides the current value by a number.
   * Returns false if dividing by zero.
   */
  divide(n: number): boolean;
  /**
   * Multiplies the current value by a number.
   */
  multiply(n: number): void;
  /**
   * Subtracts a number from the current value.
   */
  subtract(n: number): void;
  /**
   * Gets the current result.
   */
  readonly result: number;
}
/**
 * A counter that can be incremented and decremented.
 */
export class Counter {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Increments the counter by a specific amount.
   */
  increment_by(amount: number): void;
  /**
   * Creates a new counter starting at 0.
   */
  constructor();
  /**
   * Resets the counter to 0.
   */
  reset(): void;
  /**
   * Decrements the counter by 1.
   */
  decrement(): void;
  /**
   * Increments the counter by 1.
   */
  increment(): void;
  /**
   * Gets the current counter value.
   */
  readonly value: number;
}
/**
 * A 2D point with x and y coordinates.
 */
export class Point {
  free(): void;
  [Symbol.dispose](): void;
  /**
   * Calculates the distance to another point.
   */
  distance_to(other: Point): number;
  /**
   * Creates a new point.
   */
  constructor(x: number, y: number);
  /**
   * Calculates the midpoint with another point.
   */
  midpoint(other: Point): Point;
  /**
   * Returns a string representation of the point.
   */
  to_string(): string;
  /**
   * Moves the point by the given offsets.
   */
  translate(dx: number, dy: number): void;
  /**
   * Gets the x coordinate.
   */
  x: number;
  /**
   * Gets the y coordinate.
   */
  y: number;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_calculator_free: (a: number, b: number) => void;
  readonly __wbg_counter_free: (a: number, b: number) => void;
  readonly __wbg_point_free: (a: number, b: number) => void;
  readonly add: (a: number, b: number) => number;
  readonly append_paragraph: (a: number, b: number) => number;
  readonly calculator_add: (a: number, b: number) => void;
  readonly calculator_divide: (a: number, b: number) => number;
  readonly calculator_multiply: (a: number, b: number) => void;
  readonly calculator_new: () => number;
  readonly calculator_reset: (a: number) => void;
  readonly calculator_result: (a: number) => number;
  readonly calculator_subtract: (a: number, b: number) => void;
  readonly counter_decrement: (a: number) => void;
  readonly counter_increment: (a: number) => void;
  readonly counter_increment_by: (a: number, b: number) => void;
  readonly counter_new: () => number;
  readonly counter_reset: (a: number) => void;
  readonly counter_value: (a: number) => number;
  readonly divide: (a: number, b: number) => [number, number];
  readonly error: (a: number, b: number) => void;
  readonly factorial: (a: number) => bigint;
  readonly get_element_text: (a: number, b: number) => [number, number];
  readonly greet: (a: number, b: number) => [number, number];
  readonly init: () => void;
  readonly is_palindrome: (a: number, b: number) => number;
  readonly is_prime: (a: number) => number;
  readonly log: (a: number, b: number) => void;
  readonly multiply: (a: number, b: number) => number;
  readonly point_distance_to: (a: number, b: number) => number;
  readonly point_midpoint: (a: number, b: number) => number;
  readonly point_new: (a: number, b: number) => number;
  readonly point_set_x: (a: number, b: number) => void;
  readonly point_set_y: (a: number, b: number) => void;
  readonly point_to_string: (a: number) => [number, number];
  readonly point_translate: (a: number, b: number, c: number) => void;
  readonly point_x: (a: number) => number;
  readonly point_y: (a: number) => number;
  readonly reverse_string: (a: number, b: number) => [number, number];
  readonly set_element_text: (a: number, b: number, c: number, d: number) => number;
  readonly to_uppercase: (a: number, b: number) => [number, number];
  readonly warn: (a: number, b: number) => void;
  readonly word_count: (a: number, b: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
