// Statistical operations

/// Calculates the mean (average) of numbers
pub fn mean(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    sum(numbers) / numbers.len() as f64
}

/// Calculates the median of numbers
pub fn median(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return 0.0;
    }
    
    let mut sorted = numbers.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let len = sorted.len();
    if len % 2 == 0 {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    } else {
        sorted[len / 2]
    }
}

/// Calculates the sum of numbers
pub fn sum(numbers: &[f64]) -> f64 {
    numbers.iter().sum()
}

/// Finds the maximum value
pub fn max(numbers: &[f64]) -> Option<f64> {
    numbers.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).copied()
}

/// Finds the minimum value
pub fn min(numbers: &[f64]) -> Option<f64> {
    numbers.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).copied()
}

/// Calculates the range (max - min)
pub fn range(numbers: &[f64]) -> Option<f64> {
    if let (Some(max_val), Some(min_val)) = (max(numbers), min(numbers)) {
        Some(max_val - min_val)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mean() {
        assert_eq!(mean(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(mean(&[10.0, 20.0, 30.0]), 20.0);
    }
    
    #[test]
    fn test_median() {
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0, 5.0]), 3.0);
        assert_eq!(median(&[1.0, 2.0, 3.0, 4.0]), 2.5);
    }
    
    #[test]
    fn test_sum() {
        assert_eq!(sum(&[1.0, 2.0, 3.0]), 6.0);
    }
    
    #[test]
    fn test_max_min() {
        let numbers = vec![3.0, 1.0, 4.0, 1.0, 5.0];
        assert_eq!(max(&numbers), Some(5.0));
        assert_eq!(min(&numbers), Some(1.0));
    }
}
