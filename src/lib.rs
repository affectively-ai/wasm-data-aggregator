use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Initialize panic hook for better error messages
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

mod aggregation;
mod filtering;
mod decay;

use aggregation::*;
use filtering::*;
use decay::*;

/// Observation with timestamp for weighted aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Observation {
    pub timestamp: u64, // Unix timestamp in milliseconds
    pub value: f64,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Aggregation result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AggregationResult {
    pub sum: f64,
    pub average: f64,
    pub weighted_average: f64,
    pub min: f64,
    pub max: f64,
    pub count: usize,
}

/// Filter observations by date range and apply exponential decay aggregation
/// 
/// # Arguments
/// * `observations_json` - JSON string of Observation array
/// * `time_window_ms` - Time window in milliseconds for exponential decay
/// * `current_time_ms` - Current timestamp in milliseconds
/// 
/// # Returns
/// JSON string of AggregationResult
#[wasm_bindgen]
pub fn aggregate_with_decay(
    observations_json: &str,
    time_window_ms: f64,
    current_time_ms: u64,
) -> String {
    // Parse JSON input
    let observations: Vec<Observation> = match serde_json::from_str(observations_json) {
        Ok(obs) => obs,
        Err(_) => {
            return serde_json::to_string(&AggregationResult {
                sum: 0.0,
                average: 0.0,
                weighted_average: 0.0,
                min: 0.0,
                max: 0.0,
                count: 0,
            })
            .unwrap_or_else(|_| "{\"sum\":0,\"average\":0,\"weightedAverage\":0,\"min\":0,\"max\":0,\"count\":0}".to_string());
        }
    };

    if observations.is_empty() {
        return serde_json::to_string(&AggregationResult {
            sum: 0.0,
            average: 0.0,
            weighted_average: 0.0,
            min: 0.0,
            max: 0.0,
            count: 0,
        })
        .unwrap_or_else(|_| "{\"sum\":0,\"average\":0,\"weightedAverage\":0,\"min\":0,\"max\":0,\"count\":0}".to_string());
    }

    // Calculate exponential decay weights
    let mut weighted_sum = 0.0;
    let mut weight_sum = 0.0;
    let mut sum = 0.0;
    let mut min = observations[0].value;
    let mut max = observations[0].value;

    for obs in &observations {
        let age = (current_time_ms as f64) - (obs.timestamp as f64);
        let weight = calculate_decay_weight(age, time_window_ms);
        
        weighted_sum += obs.value * weight;
        weight_sum += weight;
        sum += obs.value;
        
        if obs.value < min {
            min = obs.value;
        }
        if obs.value > max {
            max = obs.value;
        }
    }

    let count = observations.len();
    let average = if count > 0 { sum / count as f64 } else { 0.0 };
    let weighted_average = if weight_sum > 0.0 { weighted_sum / weight_sum } else { 0.0 };

    let result = AggregationResult {
        sum,
        average,
        weighted_average,
        min,
        max,
        count,
    };

    serde_json::to_string(&result).unwrap_or_else(|_| "{\"sum\":0,\"average\":0,\"weightedAverage\":0,\"min\":0,\"max\":0,\"count\":0}".to_string())
}

/// Filter and aggregate observations by multiple dimensions
/// 
/// # Arguments
/// * `observations_json` - JSON string of Observation array
/// * `filters_json` - JSON string of filter criteria
/// * `group_by_json` - JSON string of grouping keys (optional)
/// 
/// # Returns
/// JSON string of aggregated results grouped by keys
#[wasm_bindgen]
pub fn filter_and_aggregate(
    observations_json: &str,
    filters_json: &str,
    group_by_json: &str,
) -> String {
    // Parse inputs
    let observations: Vec<Observation> = match serde_json::from_str(observations_json) {
        Ok(obs) => obs,
        Err(_) => return "{}".to_string(),
    };

    let filters: HashMap<String, serde_json::Value> = match serde_json::from_str(filters_json) {
        Ok(f) => f,
        Err(_) => return "{}".to_string(),
    };

    let group_by: Vec<String> = match serde_json::from_str(group_by_json) {
        Ok(g) => g,
        Err(_) => vec![],
    };

    // Apply filters
    let filtered: Vec<&Observation> = observations
        .iter()
        .filter(|obs| apply_filters(obs, &filters))
        .collect();

    // Group and aggregate
    if group_by.is_empty() {
        // No grouping - aggregate all filtered observations
        let result = aggregate_observations(&filtered);
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    } else {
        // Group by specified keys
        let grouped = group_observations(&filtered, &group_by);
        let result: HashMap<String, AggregationResult> = grouped
            .into_iter()
            .map(|(key, obs)| (key, aggregate_observations(&obs)))
            .collect();
        serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
    }
}

/// Calculate daily metrics aggregation (optimized for dashboard calculations)
/// 
/// # Arguments
/// * `users_json` - JSON string of user data array
/// * `date_range_json` - JSON string with startDate and endDate (ISO strings)
/// 
/// # Returns
/// JSON string of aggregated daily metrics
#[wasm_bindgen]
pub fn calculate_daily_metrics(
    users_json: &str,
    date_range_json: &str,
) -> String {
    // Parse inputs
    let _users: Vec<serde_json::Value> = match serde_json::from_str(users_json) {
        Ok(u) => u,
        Err(_) => return "{}".to_string(),
    };

    let _date_range: HashMap<String, String> = match serde_json::from_str(date_range_json) {
        Ok(d) => d,
        Err(_) => return "{}".to_string(),
    };

    // This is a placeholder - actual implementation would parse dates and aggregate
    // For now, return basic structure
    let result = HashMap::<String, serde_json::Value>::new();
    serde_json::to_string(&result).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_with_decay() {
        let observations = vec![
            Observation {
                timestamp: 1000,
                value: 10.0,
                metadata: HashMap::new(),
            },
            Observation {
                timestamp: 2000,
                value: 20.0,
                metadata: HashMap::new(),
            },
        ];

        let json = serde_json::to_string(&observations).unwrap();
        let result = aggregate_with_decay(&json, 10000.0, 3000);
        let parsed: AggregationResult = serde_json::from_str(&result).unwrap();

        assert_eq!(parsed.count, 2);
        assert_eq!(parsed.sum, 30.0);
        assert_eq!(parsed.average, 15.0);
    }

    #[test]
    fn test_filter_and_aggregate() {
        let observations = vec![
            Observation {
                timestamp: 1000,
                value: 10.0,
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("category".to_string(), serde_json::Value::String("A".to_string()));
                    m
                },
            },
            Observation {
                timestamp: 2000,
                value: 20.0,
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("category".to_string(), serde_json::Value::String("B".to_string()));
                    m
                },
            },
        ];

        let json = serde_json::to_string(&observations).unwrap();
        let filters = "{}";
        let group_by = "[]";
        let result = filter_and_aggregate(&json, filters, group_by);
        
        let parsed: AggregationResult = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed.count, 2);
    }
}
