//! Geometric shapes and calculations

use std::f64::consts::PI;

/// A circle defined by its radius
#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    /// Creates a new circle with the given radius
    /// 
    /// # Panics
    /// 
    /// Panics if radius is negative
    pub fn new(radius: f64) -> Self {
        if radius < 0.0 {
            panic!("Radius cannot be negative");
        }
        Circle { radius }
    }

    /// Returns the radius
    pub fn radius(&self) -> f64 {
        self.radius
    }

    /// Calculates the area of the circle
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    /// Calculates the circumference of the circle
    pub fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }

    /// Calculates the diameter of the circle
    pub fn diameter(&self) -> f64 {
        2.0 * self.radius
    }
}

/// A rectangle defined by width and height
#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    /// Creates a new rectangle with the given dimensions
    /// 
    /// # Panics
    /// 
    /// Panics if width or height is negative
    pub fn new(width: f64, height: f64) -> Self {
        if width < 0.0 || height < 0.0 {
            panic!("Dimensions cannot be negative");
        }
        Rectangle { width, height }
    }

    /// Returns the width
    pub fn width(&self) -> f64 {
        self.width
    }

    /// Returns the height
    pub fn height(&self) -> f64 {
        self.height
    }

    /// Calculates the area of the rectangle
    pub fn area(&self) -> f64 {
        self.width * self.height
    }

    /// Calculates the perimeter of the rectangle
    pub fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    /// Checks if the rectangle is a square
    pub fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }
}

/// A triangle defined by three sides
#[derive(Debug, Clone)]
pub struct Triangle {
    side_a: f64,
    side_b: f64,
    side_c: f64,
}

impl Triangle {
    /// Creates a new triangle with the given sides
    /// 
    /// Returns `None` if the sides don't form a valid triangle
    pub fn new(side_a: f64, side_b: f64, side_c: f64) -> Option<Self> {
        if side_a <= 0.0 || side_b <= 0.0 || side_c <= 0.0 {
            return None;
        }

        // Check triangle inequality
        if side_a + side_b <= side_c || side_b + side_c <= side_a || side_a + side_c <= side_b {
            return None;
        }

        Some(Triangle { side_a, side_b, side_c })
    }

    /// Returns the sides of the triangle
    pub fn sides(&self) -> (f64, f64, f64) {
        (self.side_a, self.side_b, self.side_c)
    }

    /// Calculates the perimeter of the triangle
    pub fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    /// Calculates the area using Heron's formula
    pub fn area(&self) -> f64 {
        let s = self.perimeter() / 2.0;
        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }

    /// Checks if the triangle is equilateral (all sides equal)
    pub fn is_equilateral(&self) -> bool {
        (self.side_a - self.side_b).abs() < f64::EPSILON
            && (self.side_b - self.side_c).abs() < f64::EPSILON
    }

    /// Checks if the triangle is isosceles (two sides equal)
    pub fn is_isosceles(&self) -> bool {
        (self.side_a - self.side_b).abs() < f64::EPSILON
            || (self.side_b - self.side_c).abs() < f64::EPSILON
            || (self.side_a - self.side_c).abs() < f64::EPSILON
    }

    /// Checks if the triangle is right-angled (Pythagorean theorem)
    pub fn is_right(&self) -> bool {
        let mut sides = vec![self.side_a, self.side_b, self.side_c];
        sides.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let [a, b, c] = [sides[0], sides[1], sides[2]];
        (a * a + b * b - c * c).abs() < 0.0001
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle::new(5.0);
        assert_eq!(circle.radius(), 5.0);
        assert!((circle.area() - 78.54).abs() < 0.01);
        assert!((circle.circumference() - 31.42).abs() < 0.01);
        assert_eq!(circle.diameter(), 10.0);
    }

    #[test]
    #[should_panic(expected = "Radius cannot be negative")]
    fn test_circle_negative_radius() {
        Circle::new(-5.0);
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(4.0, 6.0);
        assert_eq!(rect.width(), 4.0);
        assert_eq!(rect.height(), 6.0);
        assert_eq!(rect.area(), 24.0);
        assert_eq!(rect.perimeter(), 20.0);
        assert!(!rect.is_square());
    }

    #[test]
    fn test_square() {
        let square = Rectangle::new(5.0, 5.0);
        assert!(square.is_square());
    }

    #[test]
    fn test_triangle() {
        let triangle = Triangle::new(3.0, 4.0, 5.0).unwrap();
        assert_eq!(triangle.perimeter(), 12.0);
        assert!((triangle.area() - 6.0).abs() < 0.01);
        assert!(triangle.is_right());
    }

    #[test]
    fn test_invalid_triangle() {
        assert!(Triangle::new(1.0, 2.0, 10.0).is_none());
        assert!(Triangle::new(-1.0, 2.0, 3.0).is_none());
    }

    #[test]
    fn test_equilateral_triangle() {
        let triangle = Triangle::new(5.0, 5.0, 5.0).unwrap();
        assert!(triangle.is_equilateral());
        assert!(triangle.is_isosceles());
    }

    #[test]
    fn test_isosceles_triangle() {
        let triangle = Triangle::new(5.0, 5.0, 6.0).unwrap();
        assert!(!triangle.is_equilateral());
        assert!(triangle.is_isosceles());
    }
}
