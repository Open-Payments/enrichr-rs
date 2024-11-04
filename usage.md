# Usage Guide

This guide covers various use cases and patterns for using the struct-enrichment library.

## Basic Usage

### Simple Field Mapping

```rust
use struct_enrichment::{Enrichable, MappingRule};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct User {
    name: String,
    email: String,
}

// Input data in JSON format
let data_json = r#"{
    "user": {
        "name": "John Doe",
        "email": "john@example.com"
    }
}"#;

// Convert JSON to HashMap
let data: HashMap<String, serde_json::Value> = serde_json::from_str(data_json)?;

// Mapping specification in JSON
let spec_json = r#"[
    {
        "source": "$.user.name",
        "target": "$.name"
    },
    {
        "source": "$.user.email",
        "target": "$.email"
    }
]"#;

// Convert JSON spec to Vec<MappingRule>
let spec: Vec<MappingRule> = serde_json::from_str(spec_json)?;
```

### Transformations

```rust
// Case Transformation Example
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct Company {
    name: String,
    code: String,
}

let data_json = r#"{
    "company": {
        "name": "Acme Corp",
        "code": "acme123"
    }
}"#;

let spec_json = r#"[
    {
        "source": "$.company.name",
        "target": "$.name"
    },
    {
        "source": "$.company.code",
        "target": "$.code",
        "transform": {
            "type": "ToUpperCase"
        }
    }
]"#;

// Name/Value Splitting Example
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct Person {
    first_name: String,
    last_name: String,
}

let data_json = r#"{
    "person": {
        "full_name": "John Doe"
    }
}"#;

let spec_json = r#"[
    {
        "source": "$.person.full_name",
        "target": ["$.first_name", "$.last_name"],
        "transform": {
            "type": "Split",
            "params": {
                "delimiter": " "
            }
        }
    }
]"#;

// Template Formatting Example
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct Address {
    formatted: String,
}

let data_json = r#"{
    "location": {
        "street": "123 Main St",
        "city": "Springfield",
        "state": "IL",
        "zip": "62701"
    }
}"#;

let spec_json = r#"[
    {
        "source": ["$.location.street", "$.location.city", "$.location.state", "$.location.zip"],
        "target": "$.formatted",
        "transform": {
            "type": "Template",
            "params": {
                "template": "{0}, {1}, {2} {3}"
            }
        }
    }
]"#;
```

### Conditional Mapping

```rust
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct User {
    email: String,
    notification_email: Option<String>,
}

let data_json = r#"{
    "user": {
        "email": "primary@example.com",
        "alternate_email": "notifications@example.com",
        "preferences": {
            "notifications_enabled": true
        }
    }
}"#;

let spec_json = r#"[
    {
        "source": "$.user.email",
        "target": "$.email"
    },
    {
        "source": "$.user.alternate_email",
        "target": "$.notification_email",
        "condition": {
            "==": [
                {"var": "user.preferences.notifications_enabled"},
                true
            ]
        }
    }
]"#;
```

## Advanced Example

```rust
#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct UserProfile {
    first_name: String,
    last_name: String,
    display_name: String,
    contact: Contact,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    email: String,
    formatted_address: String,
}

// Complex input data
let data_json = r#"{
    "source": {
        "user": {
            "name": {
                "first": "John",
                "last": "Doe"
            },
            "contact": {
                "email": "john@example.com",
                "address": {
                    "street": "123 Main St",
                    "city": "Springfield",
                    "state": "IL",
                    "zip": "62701"
                }
            },
            "metadata": {
                "tags": "premium,active,verified",
                "status": "active"
            }
        }
    }
}"#;

// Complex mapping specification
let spec_json = r#"[
    {
        "source": "$.source.user.name.first",
        "target": "$.first_name",
        "transform": {
            "type": "ToUpperCase"
        }
    },
    {
        "source": "$.source.user.name.last",
        "target": "$.last_name",
        "transform": {
            "type": "ToUpperCase"
        }
    },
    {
        "source": ["$.source.user.name.first", "$.source.user.name.last"],
        "target": "$.display_name",
        "transform": {
            "type": "Template",
            "params": {
                "template": "{0} {1}"
            }
        }
    },
    {
        "source": "$.source.user.contact.email",
        "target": "$.contact.email"
    },
    {
        "source": ["$.source.user.contact.address.street", 
                  "$.source.user.contact.address.city",
                  "$.source.user.contact.address.state",
                  "$.source.user.contact.address.zip"],
        "target": "$.contact.formatted_address",
        "transform": {
            "type": "Template",
            "params": {
                "template": "{0}, {1}, {2} {3}"
            }
        }
    },
    {
        "source": "$.source.user.metadata.tags",
        "target": "$.tags",
        "transform": {
            "type": "Split",
            "params": {
                "delimiter": ","
            }
        },
        "condition": {
            "==": [
                {"var": "source.user.metadata.status"},
                "active"
            ]
        }
    }
]"#;
```