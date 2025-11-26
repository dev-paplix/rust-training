#!/usr/bin/env python3
"""
Test script for the mathpy Python extension module.

This demonstrates all the functions and classes provided by the Rust extension.
"""

def test_basic_functions():
    """Test basic mathematical functions."""
    print("=" * 60)
    print("Testing Basic Functions")
    print("=" * 60)
    
    import mathpy
    
    # Test add
    result = mathpy.add(5, 3)
    print(f"add(5, 3) = {result}")
    assert result == 8, "add failed"
    
    # Test multiply
    result = mathpy.multiply(6, 7)
    print(f"multiply(6, 7) = {result}")
    assert result == 42, "multiply failed"
    
    # Test divide
    result = mathpy.divide(10.0, 2.0)
    print(f"divide(10.0, 2.0) = {result}")
    assert result == 5.0, "divide failed"
    
    # Test divide by zero
    try:
        mathpy.divide(10.0, 0.0)
        print("ERROR: divide by zero should have raised an exception")
    except ValueError as e:
        print(f"divide(10.0, 0.0) correctly raised ValueError: {e}")
    
    # Test sum_list
    numbers = [1, 2, 3, 4, 5]
    result = mathpy.sum_list(numbers)
    print(f"sum_list({numbers}) = {result}")
    assert result == 15, "sum_list failed"
    
    # Test max_list
    result = mathpy.max_list(numbers)
    print(f"max_list({numbers}) = {result}")
    assert result == 5, "max_list failed"
    
    # Test max_list with empty list
    try:
        mathpy.max_list([])
        print("ERROR: max_list([]) should have raised an exception")
    except ValueError as e:
        print(f"max_list([]) correctly raised ValueError: {e}")
    
    print("\n✓ All basic function tests passed!\n")


def test_string_functions():
    """Test string manipulation functions."""
    print("=" * 60)
    print("Testing String Functions")
    print("=" * 60)
    
    import mathpy
    
    # Test greet
    result = mathpy.greet("Alice")
    print(f"greet('Alice') = '{result}'")
    assert "Alice" in result, "greet failed"
    
    # Test to_uppercase
    result = mathpy.to_uppercase("hello world")
    print(f"to_uppercase('hello world') = '{result}'")
    assert result == "HELLO WORLD", "to_uppercase failed"
    
    # Test word_frequency
    text = "the quick brown fox jumps over the lazy dog the"
    result = mathpy.word_frequency(text)
    print(f"word_frequency('{text}') =")
    for word, count in sorted(result.items()):
        print(f"  {word}: {count}")
    assert result["the"] == 3, "word_frequency failed"
    assert result["quick"] == 1, "word_frequency failed"
    
    print("\n✓ All string function tests passed!\n")


def test_counter_class():
    """Test the Counter class."""
    print("=" * 60)
    print("Testing Counter Class")
    print("=" * 60)
    
    import mathpy
    
    # Create counter
    counter = mathpy.Counter()
    print(f"Created counter: {counter}")
    assert counter.get_value() == 0, "Initial value should be 0"
    
    # Test increment
    counter.increment()
    print(f"After increment(): {counter}")
    assert counter.get_value() == 1, "Value should be 1"
    
    # Test increment_by
    counter.increment_by(5)
    print(f"After increment_by(5): {counter}")
    assert counter.get_value() == 6, "Value should be 6"
    
    # Test decrement
    counter.decrement()
    print(f"After decrement(): {counter}")
    assert counter.get_value() == 5, "Value should be 5"
    
    # Test reset
    counter.reset()
    print(f"After reset(): {counter}")
    assert counter.get_value() == 0, "Value should be 0"
    
    # Test custom start
    counter2 = mathpy.Counter(start=10)
    print(f"Created counter with start=10: {counter2}")
    assert counter2.get_value() == 10, "Value should be 10"
    
    print("\n✓ All Counter class tests passed!\n")


def test_point_class():
    """Test the Point class."""
    print("=" * 60)
    print("Testing Point Class")
    print("=" * 60)
    
    import mathpy
    
    # Create points
    p1 = mathpy.Point(0.0, 0.0)
    p2 = mathpy.Point(3.0, 4.0)
    print(f"Created p1: {p1}")
    print(f"Created p2: {p2}")
    
    # Test properties
    assert p1.x == 0.0 and p1.y == 0.0, "p1 coordinates wrong"
    assert p2.x == 3.0 and p2.y == 4.0, "p2 coordinates wrong"
    
    # Test distance
    distance = p1.distance_to(p2)
    print(f"distance(p1, p2) = {distance}")
    assert abs(distance - 5.0) < 0.001, "Distance should be 5.0"
    
    # Test midpoint
    mid = p1.midpoint(p2)
    print(f"midpoint(p1, p2) = {mid}")
    assert mid.x == 1.5 and mid.y == 2.0, "Midpoint wrong"
    
    # Test translate
    p3 = mathpy.Point(10.0, 20.0)
    print(f"Created p3: {p3}")
    p3.translate(5.0, -3.0)
    print(f"After translate(5.0, -3.0): {p3}")
    assert p3.x == 15.0 and p3.y == 17.0, "Translation failed"
    
    # Test property setter
    p3.x = 100.0
    p3.y = 200.0
    print(f"After setting x=100.0, y=200.0: {p3}")
    assert p3.x == 100.0 and p3.y == 200.0, "Property setter failed"
    
    print("\n✓ All Point class tests passed!\n")


def test_calculator_class():
    """Test the Calculator class."""
    print("=" * 60)
    print("Testing Calculator Class")
    print("=" * 60)
    
    import mathpy
    
    calc = mathpy.Calculator()
    print(f"Created calculator: {calc}")
    assert calc.get_result() == 0.0, "Initial value should be 0"
    
    # Test add
    calc.add(10.0)
    print(f"After add(10.0): {calc}")
    assert calc.get_result() == 10.0, "Result should be 10.0"
    
    # Test multiply
    calc.multiply(2.0)
    print(f"After multiply(2.0): {calc}")
    assert calc.get_result() == 20.0, "Result should be 20.0"
    
    # Test subtract
    calc.subtract(5.0)
    print(f"After subtract(5.0): {calc}")
    assert calc.get_result() == 15.0, "Result should be 15.0"
    
    # Test divide
    calc.divide(3.0)
    print(f"After divide(3.0): {calc}")
    assert calc.get_result() == 5.0, "Result should be 5.0"
    
    # Test divide by zero
    try:
        calc.divide(0.0)
        print("ERROR: divide by zero should have raised an exception")
    except ValueError as e:
        print(f"divide(0.0) correctly raised ValueError: {e}")
    
    # Test reset
    calc.reset()
    print(f"After reset(): {calc}")
    assert calc.get_result() == 0.0, "Result should be 0.0"
    
    print("\n✓ All Calculator class tests passed!\n")


def test_person_class():
    """Test the Person class."""
    print("=" * 60)
    print("Testing Person Class")
    print("=" * 60)
    
    import mathpy
    
    # Create person
    person = mathpy.Person("Alice", 30)
    print(f"Created person: {person}")
    assert person.name == "Alice", "Name should be Alice"
    assert person.age == 30, "Age should be 30"
    
    # Test greet
    greeting = person.greet()
    print(f"person.greet() = '{greeting}'")
    assert "Alice" in greeting and "30" in greeting, "Greeting wrong"
    
    # Test is_adult
    is_adult = person.is_adult()
    print(f"person.is_adult() = {is_adult}")
    assert is_adult == True, "Should be adult"
    
    # Test have_birthday
    person.have_birthday()
    print(f"After have_birthday(): {person}")
    assert person.age == 31, "Age should be 31"
    
    # Test property setters
    person.name = "Bob"
    person.age = 25
    print(f"After setting name='Bob', age=25: {person}")
    assert person.name == "Bob" and person.age == 25, "Properties not set"
    
    # Test negative age
    try:
        mathpy.Person("Charlie", -5)
        print("ERROR: negative age should have raised an exception")
    except ValueError as e:
        print(f"Person('Charlie', -5) correctly raised ValueError: {e}")
    
    # Test child
    child = mathpy.Person("Emma", 10)
    print(f"Created child: {child}")
    is_adult = child.is_adult()
    print(f"child.is_adult() = {is_adult}")
    assert is_adult == False, "Child should not be adult"
    
    print("\n✓ All Person class tests passed!\n")


def main():
    """Run all tests."""
    print("\n" + "=" * 60)
    print("  Python Extension Module Test Suite")
    print("  Powered by PyO3 and Rust")
    print("=" * 60 + "\n")
    
    try:
        test_basic_functions()
        test_string_functions()
        test_counter_class()
        test_point_class()
        test_calculator_class()
        test_person_class()
        
        print("=" * 60)
        print("  ✓ ALL TESTS PASSED!")
        print("=" * 60)
        
    except ImportError as e:
        print(f"ERROR: Failed to import mathpy module: {e}")
        print("\nPlease build the module first:")
        print("  maturin develop")
        print("\nOr install it:")
        print("  pip install .")
        return 1
    except AssertionError as e:
        print(f"\n✗ Test failed: {e}")
        return 1
    except Exception as e:
        print(f"\n✗ Unexpected error: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
