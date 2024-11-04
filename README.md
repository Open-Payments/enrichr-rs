# struct-enrichment

A Rust library for enriching structs using JSONPath-based mapping rules with support for complex transformations and conditional logic.

## Features

- ✨ Derive macro for automatic implementation
- 🗺️ JSONPath-based field mapping
- 🔄 Rich set of transformations
- 🎯 Multiple source and target paths
- 🔍 Conditional mapping using JSONLogic
- 🎨 Template-based formatting

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
struct_enrichment = "0.1.0"
```

## Quick Start

```rust
use struct_enrichment::{Enrichable, MappingRule, JsonPath, Transform};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct User {
    first_name: String,
    last_name: String,
    email: Option<String>,
    tags: Vec<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut user = User {
        first_name: String::new(),
        last_name: String::new(),
        email: None,
        tags: vec![],
    };

    // Input data in JSON format for readability
    let data_json = r#"{
        "user_data": {
            "full_name": "John Doe",
            "email": "john@example.com",
            "tags": "admin,user"
        }
    }"#;

    // Convert JSON to HashMap
    let data: HashMap<String, serde_json::Value> = serde_json::from_str(data_json)?;

    // Mapping specification in JSON format
    let spec_json = r#"[
        {
            "source": "$.user_data.full_name",
            "target": ["$.first_name", "$.last_name"],
            "transform": {
                "type": "Split",
                "params": {
                    "delimiter": " "
                }
            }
        },
        {
            "source": "$.user_data.email",
            "target": "$.email"
        },
        {
            "source": "$.user_data.tags",
            "target": "$.tags",
            "transform": {
                "type": "Split",
                "params": {
                    "delimiter": ","
                }
            }
        }
    ]"#;

    // Convert JSON spec to Vec<MappingRule>
    let spec: Vec<MappingRule> = serde_json::from_str(spec_json)?;

    // Perform enrichment
    user.enrich(&data, &spec)?;

    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.email, Some("john@example.com".to_string()));
    assert_eq!(user.tags, vec!["admin", "user"]);

    Ok(())
}
```

## Available Transformations

- `ToString`: Convert any value to string
- `ToUpperCase`: Convert string to uppercase
- `ToLowerCase`: Convert string to lowercase
- `Split`: Split string into array using delimiter
- `Concat`: Join array values with delimiter
- `Replace`: Replace substrings
- `Substring`: Extract substring
- `Template`: Format string using placeholders

## Documentation
- [Usage Guide](./docs/usage.md) - Detailed examples and patterns
- [Error Handling](./docs/error_handling.md) - Error handling guide
