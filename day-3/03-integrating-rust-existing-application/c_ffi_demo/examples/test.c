/*
 * C test program for the Rust mathlib library
 * 
 * This demonstrates how to call Rust functions from C code.
 * 
 * Compilation (Windows with MSVC):
 *   cl test.c /I.. /link ..\target\release\mathlib.lib
 * 
 * Compilation (Linux/macOS):
 *   gcc test.c -L../target/release -lmathlib -lm -o test
 *   export LD_LIBRARY_PATH=../target/release:$LD_LIBRARY_PATH  # Linux
 *   export DYLD_LIBRARY_PATH=../target/release:$DYLD_LIBRARY_PATH  # macOS
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Point structure (must match Rust definition) */
typedef struct {
    double x;
    double y;
} Point;

/* Error codes */
typedef enum {
    Success = 0,
    NullPointer = -1,
    InvalidUtf8 = -2,
    BufferTooSmall = -3,
} ErrorCode;

/* Operation result structure */
typedef struct {
    int success;  /* bool in C is usually int */
    double value;
    int error_code;
} OperationResult;

/* Function declarations - these are the Rust functions we'll call */
extern int add(int a, int b);
extern int multiply(int a, int b);
extern OperationResult divide(double a, double b);

extern char* greet(const char* name);
extern char* to_uppercase(const char* input);
extern int string_length(const char* input);
extern void free_rust_string(char* s);

extern int sum_array(const int* arr, size_t len);
extern int max_array(const int* arr, size_t len);
extern void sort_array(int* arr, size_t len);

extern Point point_new(double x, double y);
extern double point_distance(Point p1, Point p2);
extern Point point_midpoint(Point p1, Point p2);
extern void point_translate(Point* point, double dx, double dy);

extern ErrorCode parse_int(const char* input, int* output);
extern ErrorCode copy_string(const char* src, char* dest, size_t dest_len);

/* Helper function to print test results */
void print_test(const char* name, int passed) {
    printf("[%s] %s\n", passed ? "PASS" : "FAIL", name);
}

/* Helper function to print separator */
void print_separator() {
    printf("\n----------------------------------------\n\n");
}

int main() {
    printf("===========================================\n");
    printf("  Rust C FFI Demo - Test Program\n");
    printf("===========================================\n\n");

    /* Test 1: Basic numeric functions */
    printf("Test 1: Basic Numeric Functions\n");
    printf("--------------------------------\n");
    
    int sum = add(5, 3);
    printf("add(5, 3) = %d\n", sum);
    print_test("add function", sum == 8);
    
    int product = multiply(6, 7);
    printf("multiply(6, 7) = %d\n", product);
    print_test("multiply function", product == 42);
    
    OperationResult div_result = divide(10.0, 2.0);
    printf("divide(10.0, 2.0) = %.2f (success: %d)\n", 
           div_result.value, div_result.success);
    print_test("divide function", div_result.success && div_result.value == 5.0);
    
    OperationResult div_by_zero = divide(10.0, 0.0);
    printf("divide(10.0, 0.0) = failure (success: %d)\n", div_by_zero.success);
    print_test("divide by zero handling", !div_by_zero.success);
    
    print_separator();

    /* Test 2: String functions */
    printf("Test 2: String Functions\n");
    printf("------------------------\n");
    
    char* greeting = greet("Alice");
    if (greeting) {
        printf("greet(\"Alice\") = \"%s\"\n", greeting);
        print_test("greet function", strstr(greeting, "Alice") != NULL);
        free_rust_string(greeting);
    } else {
        print_test("greet function", 0);
    }
    
    char* upper = to_uppercase("hello world");
    if (upper) {
        printf("to_uppercase(\"hello world\") = \"%s\"\n", upper);
        print_test("to_uppercase function", strcmp(upper, "HELLO WORLD") == 0);
        free_rust_string(upper);
    } else {
        print_test("to_uppercase function", 0);
    }
    
    int length = string_length("Rust");
    printf("string_length(\"Rust\") = %d\n", length);
    print_test("string_length function", length == 4);
    
    print_separator();

    /* Test 3: Array functions */
    printf("Test 3: Array Functions\n");
    printf("-----------------------\n");
    
    int numbers[] = {1, 2, 3, 4, 5};
    int array_len = sizeof(numbers) / sizeof(numbers[0]);
    
    int array_sum = sum_array(numbers, array_len);
    printf("sum_array([1, 2, 3, 4, 5]) = %d\n", array_sum);
    print_test("sum_array function", array_sum == 15);
    
    int max_val = max_array(numbers, array_len);
    printf("max_array([1, 2, 3, 4, 5]) = %d\n", max_val);
    print_test("max_array function", max_val == 5);
    
    int unsorted[] = {5, 2, 8, 1, 9};
    int unsorted_len = sizeof(unsorted) / sizeof(unsorted[0]);
    printf("Before sort: [");
    for (int i = 0; i < unsorted_len; i++) {
        printf("%d%s", unsorted[i], i < unsorted_len - 1 ? ", " : "");
    }
    printf("]\n");
    
    sort_array(unsorted, unsorted_len);
    printf("After sort:  [");
    for (int i = 0; i < unsorted_len; i++) {
        printf("%d%s", unsorted[i], i < unsorted_len - 1 ? ", " : "");
    }
    printf("]\n");
    
    int sorted = 1;
    for (int i = 0; i < unsorted_len - 1; i++) {
        if (unsorted[i] > unsorted[i + 1]) {
            sorted = 0;
            break;
        }
    }
    print_test("sort_array function", sorted);
    
    print_separator();

    /* Test 4: Struct functions */
    printf("Test 4: Struct Functions (Point)\n");
    printf("---------------------------------\n");
    
    Point p1 = point_new(0.0, 0.0);
    Point p2 = point_new(3.0, 4.0);
    printf("p1 = (%.1f, %.1f)\n", p1.x, p1.y);
    printf("p2 = (%.1f, %.1f)\n", p2.x, p2.y);
    
    double distance = point_distance(p1, p2);
    printf("distance(p1, p2) = %.2f\n", distance);
    print_test("point_distance function", distance > 4.99 && distance < 5.01);
    
    Point mid = point_midpoint(p1, p2);
    printf("midpoint(p1, p2) = (%.1f, %.1f)\n", mid.x, mid.y);
    print_test("point_midpoint function", mid.x == 1.5 && mid.y == 2.0);
    
    Point p3 = point_new(10.0, 20.0);
    printf("p3 before translation = (%.1f, %.1f)\n", p3.x, p3.y);
    point_translate(&p3, 5.0, -3.0);
    printf("p3 after translate(5.0, -3.0) = (%.1f, %.1f)\n", p3.x, p3.y);
    print_test("point_translate function", p3.x == 15.0 && p3.y == 17.0);
    
    print_separator();

    /* Test 5: Error handling */
    printf("Test 5: Error Handling\n");
    printf("----------------------\n");
    
    int parsed_value;
    ErrorCode result = parse_int("42", &parsed_value);
    printf("parse_int(\"42\") = %d (error code: %d)\n", parsed_value, result);
    print_test("parse_int success", result == Success && parsed_value == 42);
    
    result = parse_int("not a number", &parsed_value);
    printf("parse_int(\"not a number\") = error code %d\n", result);
    print_test("parse_int error handling", result != Success);
    
    char buffer[50];
    result = copy_string("Hello, C!", buffer, sizeof(buffer));
    printf("copy_string(\"Hello, C!\") = \"%s\" (error code: %d)\n", buffer, result);
    print_test("copy_string success", result == Success && strcmp(buffer, "Hello, C!") == 0);
    
    char small_buffer[5];
    result = copy_string("This is too long", small_buffer, sizeof(small_buffer));
    printf("copy_string to small buffer = error code %d\n", result);
    print_test("copy_string buffer too small", result == BufferTooSmall);
    
    print_separator();

    printf("===========================================\n");
    printf("  All tests completed!\n");
    printf("===========================================\n");

    return 0;
}
