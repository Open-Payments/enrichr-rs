# Error Handling Guide

## Error Types

```rust
#[derive(Error, Debug)]
pub enum EnrichmentError {
    #[error("JSONPath error: {0}")]
    JsonPathError(String),

    #[error("Data type error: {0}")]
    DataTypeError(String),

    #[error("Spec error: {0}")]
    SpecError(String),

    #[error("Condition error: {0}")]
    ConditionError(String),

    #[error("Transform error: {0}")]
    TransformError(String),
}
```

## Common Error Scenarios

### 1. Invalid JSON Format

```rust
// Invalid JSON data
let invalid_data_json = r#"{
    "user": {
        "name": "John Doe",  // Invalid JSON comment
        email: invalid-email // Missing quotes
    }
}"#;

// Will result in serde_json::Error during parsing
let data: HashMap<String, serde_json::Value> = serde_json::from_str(invalid_data_json)?;

// Invalid JSON specification
let invalid_spec_json = r#"[
    {
        source: "$.user.name",  // Missing quotes
        "target": "$.name"
    }
]"#;

// Will result in serde_json::Error during parsing
let spec: Vec<MappingRule> = serde_json::from_str(invalid_spec_json)?;
```

### 2. Invalid JSONPath

```rust
let data_json = r#"{
    "user": {
        "name": "John Doe"
    }
}"#;

// Invalid JSONPath syntax in specification
let invalid_path_spec = r#"[
    {
        "source": "$.[invalid.path",
        "target": "$.name"
    }
]"#;

// Will result in JsonPathError during enrichment
let spec: Vec<MappingRule> = serde_json::from_str(invalid_path_spec)?;
user.enrich(&data, &spec)?; // Will fail with JsonPathError

// Nonexistent path
let missing_path_spec = r#"[
    {
        "source": "$.user.nonexistent.field",
        "target": "$.name"
    }
]"#;

// Will result in null value during enrichment
```

### 3. Type Mismatches

```rust
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct User {
    age: u32,
}

let data_json = r#"{
    "user": {
        "age": "thirty"  // String instead of number
    }
}"#;

let spec_json = r#"[
    {
        "source": "$.user.age",
        "target": "$.age"
    }
]"#;

// Will result in DataTypeError during enrichment
```

### 4. Transform Errors

```rust
let data_json = r#"{
    "user": {
        "id": 12345
    }
}"#;

// Invalid transform for numeric data
let invalid_transform_spec = r#"[
    {
        "source": "$.user.id",
        "target": "$.name",
        "transform": {
            "type": "ToUpperCase"
        }
    }
]"#;

// Will result in TransformError during enrichment
```

## Error Handling Patterns

### 1. Comprehensive Error Handling

```rust
use struct_enrichment::{Enrichable, EnrichmentError};

fn process_user_data(data_json: &str, spec_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Parse input data
    let data: HashMap<String, serde_json::Value> = match serde_json::from_str(data_json) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Invalid input data JSON: {}", e);
            return Err(e.into());
        }
    };

    // Parse specification
    let spec: Vec<MappingRule> = match serde_json::from_str(spec_json) {
        Ok(spec) => spec,
        Err(e) => {
            eprintln!("Invalid specification JSON: {}", e);
            return Err(e.into());
        }
    };

    // Perform enrichment
    match user.enrich(&data, &spec) {
        Ok(_) => println!("Enrichment successful"),
        Err(EnrichmentError::JsonPathError(e)) => {
            eprintln!("Invalid JSONPath: {}", e);
            return Err(e.into());
        },
        Err(EnrichmentError::TransformError(e)) => {
            eprintln!("Transform failed: {}", e);
            return Err(e.into());
        },
        Err(e) => {
            eprintln!("Other error: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
```

### 2. Validation Before Processing

```rust
fn validate_spec(spec_json: &str) -> Result<Vec<MappingRule>, Box<dyn std::error::Error>> {
    // Parse specification
    let spec: Vec<MappingRule> = serde_json::from_str(spec_json)?;

    // Validate each rule
    for rule in &spec {
        // Check for template transform requirements
        if let Some(transform) = &rule.transform {
            match transform {
                Transform::Template { .. } => {
                    if let JsonPath::Single(_) = rule.source {
                        return Err(EnrichmentError::SpecError(
                            "Template transform requires multiple sources".into()
                        ).into());
                    }
                },
                // Add more transform validations...
                _ => ()
            }
        }
    }

    Ok(spec)
}

// Usage
fn safe_process(data_json: &str, spec_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data: HashMap<String, serde_json::Value> = serde_json::from_str(data_json)?;
    let spec = validate_spec(spec_json)?;
    user.enrich(&data, &spec)?;
    Ok(())
}
```

### 3. Logging Integration

```rust
use log::{error, warn, info};

fn process_with_logging(data_json: &str, spec_json: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Processing data with spec");
    
    let data = match serde_json::from_str(data_json) {
        Ok(data) => data,
        Err(e) => {
            error!("Failed to parse input data: {}", e);
            return Err(e.into());
        }
    };

    let spec = match serde_json::from_str(spec_json) {
        Ok(spec) => spec,
        Err(e) => {
            error!("Failed to parse specification: {}", e);
            return Err(e.into());
        }
    };

    match user.enrich(&data, &spec) {
        Ok(_) => info!("Enrichment completed successfully"),
        Err(e) => {
            error!("Enrichment failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}
```