//! Calculator with method chaining support

/// A calculator that supports method chaining
/// 
/// # Examples
/// 
/// ```
/// use simple_math_lib::Calculator;
/// 
/// let mut calc = Calculator::new(10.0);
/// calc.add(5.0).multiply(2.0).subtract(10.0);
/// assert_eq!(calc.result(), 20.0);
/// ```
#[derive(Debug, Clone)]
pub struct Calculator {
    value: f64,
    history: Vec<Operation>,
}

#[derive(Debug, Clone)]
struct Operation {
    kind: OperationKind,
    operand: f64,
    result: f64,
}

#[derive(Debug, Clone)]
enum OperationKind {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

impl Calculator {
    /// Creates a new calculator with an initial value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let calc = Calculator::new(0.0);
    /// assert_eq!(calc.result(), 0.0);
    /// ```
    pub fn new(initial: f64) -> Self {
        Calculator {
            value: initial,
            history: Vec::new(),
        }
    }

    /// Adds a value to the calculator
    /// 
    /// Returns a mutable reference for method chaining
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(10.0);
    /// calc.add(5.0);
    /// assert_eq!(calc.result(), 15.0);
    /// ```
    pub fn add(&mut self, n: f64) -> &mut Self {
        self.value += n;
        self.history.push(Operation {
            kind: OperationKind::Add,
            operand: n,
            result: self.value,
        });
        self
    }

    /// Subtracts a value from the calculator
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(10.0);
    /// calc.subtract(3.0);
    /// assert_eq!(calc.result(), 7.0);
    /// ```
    pub fn subtract(&mut self, n: f64) -> &mut Self {
        self.value -= n;
        self.history.push(Operation {
            kind: OperationKind::Subtract,
            operand: n,
            result: self.value,
        });
        self
    }

    /// Multiplies the calculator value by a number
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(5.0);
    /// calc.multiply(3.0);
    /// assert_eq!(calc.result(), 15.0);
    /// ```
    pub fn multiply(&mut self, n: f64) -> &mut Self {
        self.value *= n;
        self.history.push(Operation {
            kind: OperationKind::Multiply,
            operand: n,
            result: self.value,
        });
        self
    }

    /// Divides the calculator value by a number
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(10.0);
    /// calc.divide(2.0);
    /// assert_eq!(calc.result(), 5.0);
    /// ```
    /// 
    /// # Panics
    /// 
    /// Panics if divisor is zero
    pub fn divide(&mut self, n: f64) -> &mut Self {
        if n == 0.0 {
            panic!("Cannot divide by zero");
        }
        self.value /= n;
        self.history.push(Operation {
            kind: OperationKind::Divide,
            operand: n,
            result: self.value,
        });
        self
    }

    /// Raises the calculator value to a power
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(2.0);
    /// calc.power(3.0);
    /// assert_eq!(calc.result(), 8.0);
    /// ```
    pub fn power(&mut self, n: f64) -> &mut Self {
        self.value = self.value.powf(n);
        self.history.push(Operation {
            kind: OperationKind::Power,
            operand: n,
            result: self.value,
        });
        self
    }

    /// Gets the current result
    pub fn result(&self) -> f64 {
        self.value
    }

    /// Resets the calculator to a new value
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Calculator;
    /// let mut calc = Calculator::new(10.0);
    /// calc.add(5.0);
    /// calc.reset(0.0);
    /// assert_eq!(calc.result(), 0.0);
    /// ```
    pub fn reset(&mut self, value: f64) {
        self.value = value;
        self.history.clear();
    }

    /// Returns the number of operations performed
    pub fn operation_count(&self) -> usize {
        self.history.len()
    }

    /// Clears the operation history
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculator_chain() {
        let mut calc = Calculator::new(10.0);
        calc.add(5.0).multiply(2.0).subtract(10.0);
        assert_eq!(calc.result(), 20.0);
    }

    #[test]
    fn test_calculator_operations() {
        let mut calc = Calculator::new(100.0);
        calc.add(50.0);
        assert_eq!(calc.result(), 150.0);
        
        calc.subtract(30.0);
        assert_eq!(calc.result(), 120.0);
        
        calc.multiply(2.0);
        assert_eq!(calc.result(), 240.0);
        
        calc.divide(4.0);
        assert_eq!(calc.result(), 60.0);
    }

    #[test]
    fn test_calculator_power() {
        let mut calc = Calculator::new(2.0);
        calc.power(3.0);
        assert_eq!(calc.result(), 8.0);
    }

    #[test]
    #[should_panic(expected = "Cannot divide by zero")]
    fn test_divide_by_zero() {
        let mut calc = Calculator::new(10.0);
        calc.divide(0.0);
    }

    #[test]
    fn test_calculator_reset() {
        let mut calc = Calculator::new(10.0);
        calc.add(5.0);
        calc.reset(0.0);
        assert_eq!(calc.result(), 0.0);
    }

    #[test]
    fn test_operation_count() {
        let mut calc = Calculator::new(0.0);
        calc.add(1.0).add(2.0).add(3.0);
        assert_eq!(calc.operation_count(), 3);
    }
}
