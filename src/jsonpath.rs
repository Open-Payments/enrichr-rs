use crate::error::EnrichmentError;
use crate::traits::PathExtractor;
use serde_json::Value;
use std::collections::HashMap;

pub struct JsonPathExtractor;

impl JsonPathExtractor {
    pub fn new() -> Self {
        JsonPathExtractor
    }

    pub fn extract(data: &HashMap<String, Value>, path: &str) -> Result<Value, EnrichmentError> {
        // Convert HashMap to Value for jsonpath_lib
        let json_value = serde_json::to_value(data)
            .map_err(|e| EnrichmentError::JsonPathError(format!("Failed to convert data: {}", e)))?;

        // Use jsonpath_lib with the converted Value
        jsonpath_lib::select(&json_value, path)
            .map_err(|e| EnrichmentError::JsonPathError(e.to_string()))?
            .first()
            .cloned()
            .cloned()
            .ok_or_else(|| EnrichmentError::JsonPathError("Path not found".into()))
    }
}

impl PathExtractor for JsonPathExtractor {
    fn extract_value(&self, data: &HashMap<String, Value>, path: &str) -> Result<Value, EnrichmentError> {
        Self::extract(data, path)
    }

    fn extract_values(&self, data: &HashMap<String, Value>, paths: &[String]) -> Result<Vec<Value>, EnrichmentError> {
        paths.iter()
            .map(|path| self.extract_value(data, path))
            .collect()
    }
}