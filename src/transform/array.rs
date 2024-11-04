use super::Transformer;
use crate::error::EnrichmentError;
use serde_json::Value;

pub struct ArrayTransformer {
    delimiter: String,
}

impl ArrayTransformer {
    pub fn new(delimiter: String) -> Self {
        Self { delimiter }
    }
}

impl Transformer for ArrayTransformer {
    fn transform(&self, value: &Value) -> Result<Value, EnrichmentError> {
        match value {
            Value::String(s) => {
                let parts: Vec<Value> = s
                    .split(&self.delimiter)
                    .map(|part| Value::String(part.to_string()))
                    .collect();
                Ok(Value::Array(parts))
            }
            _ => Err(EnrichmentError::TransformError("Value is not a string".into())),
        }
    }
}
