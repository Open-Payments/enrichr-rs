mod string;
mod array;
mod template;

pub use string::StringTransformer;
pub use array::ArrayTransformer;
pub use template::TemplateTransformer;

use crate::error::EnrichmentError;

pub trait Transformer {
    fn transform(&self, value: &serde_json::Value) -> Result<serde_json::Value, EnrichmentError>;
}