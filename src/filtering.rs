use super::Observation;
use std::collections::HashMap;

/// Apply filters to an observation
pub fn apply_filters(obs: &Observation, filters: &HashMap<String, serde_json::Value>) -> bool {
    for (key, value) in filters {
        match key.as_str() {
            "minValue" => {
                if let Some(min) = value.as_f64() {
                    if obs.value < min {
                        return false;
                    }
                } else if let Some(min) = value.as_i64() {
                    if obs.value < min as f64 {
                        return false;
                    }
                }
            }
            "maxValue" => {
                if let Some(max) = value.as_f64() {
                    if obs.value > max {
                        return false;
                    }
                } else if let Some(max) = value.as_i64() {
                    if obs.value > max as f64 {
                        return false;
                    }
                }
            }
            "minTimestamp" => {
                if let Some(min) = value.as_u64() {
                    if obs.timestamp < min {
                        return false;
                    }
                } else if let Some(min) = value.as_i64() {
                    if obs.timestamp < min as u64 {
                        return false;
                    }
                }
            }
            "maxTimestamp" => {
                if let Some(max) = value.as_u64() {
                    if obs.timestamp > max {
                        return false;
                    }
                } else if let Some(max) = value.as_i64() {
                    if obs.timestamp > max as u64 {
                        return false;
                    }
                }
            }
            _ => {
                // Check metadata
                if let Some(metadata_value) = obs.metadata.get(key) {
                    if metadata_value != value {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_apply_filters_min_value() {
        let obs = Observation {
            timestamp: 1000,
            value: 10.0,
            metadata: HashMap::new(),
        };

        let mut filters = HashMap::new();
        filters.insert(
            "minValue".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(5.0).unwrap()),
        );
        
        assert!(apply_filters(&obs, &filters));
        
        filters.insert(
            "minValue".to_string(),
            serde_json::Value::Number(serde_json::Number::from_f64(15.0).unwrap()),
        );
        assert!(!apply_filters(&obs, &filters));
    }

    #[test]
    fn test_apply_filters_metadata() {
        let mut metadata = HashMap::new();
        metadata.insert("category".to_string(), serde_json::Value::String("A".to_string()));
        
        let obs = Observation {
            timestamp: 1000,
            value: 10.0,
            metadata,
        };

        let mut filters = HashMap::new();
        filters.insert("category".to_string(), serde_json::Value::String("A".to_string()));
        
        assert!(apply_filters(&obs, &filters));
        
        filters.insert("category".to_string(), serde_json::Value::String("B".to_string()));
        assert!(!apply_filters(&obs, &filters));
    }
}
