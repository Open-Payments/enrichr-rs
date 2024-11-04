use super::Transformer;
use crate::error::EnrichmentError;
use serde_json::Value;

pub struct TemplateTransformer {
    template: String,
}

impl TemplateTransformer {
    pub fn new(template: String) -> Self {
        Self { template }
    }
}

impl Transformer for TemplateTransformer {
    fn transform(&self, value: &Value) -> Result<Value, EnrichmentError> {
        match value {
            Value::Array(values) => {
                let mut result = self.template.clone();
                for (i, val) in values.iter().enumerate() {
                    let placeholder = format!("{{{}}}", i);
                    let value_str = val.as_str()
                        .ok_or_else(|| EnrichmentError::TransformError("Array value is not a string".into()))?;
                    result = result.replace(&placeholder, value_str);
                }
                Ok(Value::String(result))
            }
            _ => Err(EnrichmentError::TransformError("Value is not an array".into())),
        }
    }
}