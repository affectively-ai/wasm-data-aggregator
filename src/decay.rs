/// Calculate exponential decay weight
/// 
/// Formula: exp(-age / time_window_ms)
/// 
/// # Arguments
/// * `age` - Age in milliseconds
/// * `time_window_ms` - Time window in milliseconds
/// 
/// # Returns
/// Weight between 0.0 and 1.0
pub fn calculate_decay_weight(age: f64, time_window_ms: f64) -> f64 {
    if time_window_ms <= 0.0 {
        return 1.0;
    }
    
    if age < 0.0 {
        return 1.0;
    }
    
    // Use exponential decay: exp(-age / time_window)
    // This gives weight of 1.0 for age=0, and approaches 0 as age increases
    (-age / time_window_ms).exp()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_weight_zero_age() {
        let weight = calculate_decay_weight(0.0, 1000.0);
        assert!((weight - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_decay_weight_old_age() {
        let weight = calculate_decay_weight(10000.0, 1000.0);
        assert!(weight < 0.001); // Should be very small
    }

    #[test]
    fn test_decay_weight_negative_age() {
        let weight = calculate_decay_weight(-100.0, 1000.0);
        assert!((weight - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_decay_weight_zero_window() {
        let weight = calculate_decay_weight(100.0, 0.0);
        assert!((weight - 1.0).abs() < 0.0001);
    }
}
