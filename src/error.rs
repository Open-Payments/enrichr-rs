use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnrichmentError {
    #[error("JSONPath error: {0}")]
    JsonPathError(String),

    #[error("Data type error: {0}")]
    DataTypeError(String),

    #[error("Spec error: {0}")]
    SpecError(String),

    #[error("Transform error: {0}")]
    TransformError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}