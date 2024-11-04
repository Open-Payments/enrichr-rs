use super::Transformer;
use crate::error::EnrichmentError;
use serde_json::Value;

pub struct StringTransformer;

impl StringTransformer {
    pub fn new() -> Self {
        StringTransformer
    }
}

impl Transformer for StringTransformer {
    fn transform(&self, value: &Value) -> Result<Value, EnrichmentError> {
        match value {
            Value::String(s) => Ok(Value::String(s.to_uppercase())),
            _ => Err(EnrichmentError::TransformError("Value is not a string".into())),
        }
    }
}
