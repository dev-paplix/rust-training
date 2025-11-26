//! Statistical functions for data analysis

/// Statistical analysis functions
pub struct Statistics;

impl Statistics {
    /// Calculates the mean (average) of a dataset
    /// 
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(Statistics::mean(&data), Some(3.0));
    /// ```
    pub fn mean(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().sum::<f64>() / data.len() as f64)
        }
    }

    /// Calculates the median of a dataset
    /// 
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(Statistics::median(&data), Some(3.0));
    /// ```
    pub fn median(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let mut sorted = data.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let len = sorted.len();
        if len % 2 == 0 {
            Some((sorted[len / 2 - 1] + sorted[len / 2]) / 2.0)
        } else {
            Some(sorted[len / 2])
        }
    }

    /// Calculates the mode of a dataset
    /// 
    /// Returns the most frequently occurring value
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![1.0, 2.0, 2.0, 3.0, 4.0];
    /// assert_eq!(Statistics::mode(&data), Some(2.0));
    /// ```
    pub fn mode(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        use std::collections::HashMap;
        let mut counts = HashMap::new();

        for &value in data {
            *counts.entry(value.to_bits()).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(bits, _)| f64::from_bits(bits))
    }

    /// Calculates the range (max - min) of a dataset
    /// 
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    /// assert_eq!(Statistics::range(&data), Some(4.0));
    /// ```
    pub fn range(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            let min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
            let max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
            Some(max - min)
        }
    }

    /// Calculates the variance of a dataset
    /// 
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    /// let variance = Statistics::variance(&data).unwrap();
    /// assert!((variance - 4.0).abs() < 0.1);
    /// ```
    pub fn variance(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            return None;
        }

        let mean = Self::mean(data)?;
        let squared_diffs: f64 = data.iter().map(|&x| (x - mean).powi(2)).sum();
        Some(squared_diffs / data.len() as f64)
    }

    /// Calculates the standard deviation of a dataset
    /// 
    /// Returns `None` if the dataset is empty
    /// 
    /// # Examples
    /// 
    /// ```
    /// use simple_math_lib::Statistics;
    /// let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    /// let std_dev = Statistics::std_dev(&data).unwrap();
    /// assert!((std_dev - 2.0).abs() < 0.1);
    /// ```
    pub fn std_dev(data: &[f64]) -> Option<f64> {
        Self::variance(data).map(|v| v.sqrt())
    }

    /// Finds the minimum value in a dataset
    /// 
    /// Returns `None` if the dataset is empty
    pub fn min(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().fold(f64::INFINITY, |a, &b| a.min(b)))
        }
    }

    /// Finds the maximum value in a dataset
    /// 
    /// Returns `None` if the dataset is empty
    pub fn max(data: &[f64]) -> Option<f64> {
        if data.is_empty() {
            None
        } else {
            Some(data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b)))
        }
    }

    /// Calculates the sum of all values in a dataset
    pub fn sum(data: &[f64]) -> f64 {
        data.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::mean(&data), Some(3.0));
        assert_eq!(Statistics::mean(&[]), None);
    }

    #[test]
    fn test_median() {
        let data_odd = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::median(&data_odd), Some(3.0));

        let data_even = vec![1.0, 2.0, 3.0, 4.0];
        assert_eq!(Statistics::median(&data_even), Some(2.5));
    }

    #[test]
    fn test_mode() {
        let data = vec![1.0, 2.0, 2.0, 3.0, 4.0];
        assert_eq!(Statistics::mode(&data), Some(2.0));
    }

    #[test]
    fn test_range() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::range(&data), Some(4.0));
    }

    #[test]
    fn test_variance() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let variance = Statistics::variance(&data).unwrap();
        assert!((variance - 4.0).abs() < 0.1);
    }

    #[test]
    fn test_std_dev() {
        let data = vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
        let std_dev = Statistics::std_dev(&data).unwrap();
        assert!((std_dev - 2.0).abs() < 0.1);
    }

    #[test]
    fn test_min_max() {
        let data = vec![3.0, 1.0, 4.0, 1.0, 5.0, 9.0];
        assert_eq!(Statistics::min(&data), Some(1.0));
        assert_eq!(Statistics::max(&data), Some(9.0));
    }

    #[test]
    fn test_sum() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(Statistics::sum(&data), 15.0);
    }
}
