use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid enrichment spec: {0}")]
    InvalidSpec(#[from] serde_json::Error),
    
    #[error("Invalid JSON path: {0}")]
    InvalidPath(String),
    
    #[error("Path not found: {0}")]
    PathNotFound(String),
    
    #[error("Type conversion failed for field {field}: {details}")]
    TypeConversion { field: String, details: String },
    
    #[error("Unknown field: {0}")]
    UnknownField(String),
}
