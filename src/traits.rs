use crate::error::EnrichmentError;
use crate::types::MappingRule;
use std::collections::HashMap;

/// Main trait for types that can be enriched with data from external sources
pub trait Enrichable {
    /// Enrich the implementing type with data according to the provided rules
    fn enrich(&mut self, data: &HashMap<String, serde_json::Value>, rules: &[MappingRule]) -> Result<(), EnrichmentError>;
}

/// Trait for types that can be validated before enrichment
pub trait Validatable {
    /// Validate the implementing type before enrichment
    fn validate(&self) -> Result<(), EnrichmentError>;
}

/// Trait for types that can be serialized to a specific format
pub trait ValueSerializer {
    /// Convert a value to a specific type
    fn serialize_value<T>(&self, value: &serde_json::Value) -> Result<T, EnrichmentError>
    where
        T: for<'de> serde::Deserialize<'de>;
}

/// Trait for types that can extract values using paths
pub trait PathExtractor {
    /// Extract a value from data using a path
    fn extract_value(&self, data: &HashMap<String, serde_json::Value>, path: &str) -> Result<serde_json::Value, EnrichmentError>;
    
    /// Extract multiple values from data using multiple paths
    fn extract_values(&self, data: &HashMap<String, serde_json::Value>, paths: &[String]) -> Result<Vec<serde_json::Value>, EnrichmentError>;
}
