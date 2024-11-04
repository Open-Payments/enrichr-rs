use enrichr::prelude::*;
use enrichr::enrichable::helpers::default_enrich;
use serde_json::json;

#[derive(Debug, PartialEq)]
struct User {
    name: String,
    age: u32,
    location: String,
    tags: Vec<String>,
}

impl Default for User {
    fn default() -> Self {
        Self {
            name: String::new(),
            age: 0,
            location: String::new(),
            tags: Vec::new(),
        }
    }
}

impl Enrichable for User {
    fn enrich(&mut self, data: &serde_json::Value, spec: &str) -> Result<(), Error> {
        default_enrich(self, data, spec)
    }
    
    fn set_field(&mut self, field: &str, value: serde_json::Value) -> Result<(), Error> {
        match field {
            "name" => {
                self.name = value.as_str()
                    .ok_or_else(|| Error::TypeConversion("Expected string for name".into()))?
                    .to_string();
            },
            "age" => {
                self.age = value.as_u64()
                    .ok_or_else(|| Error::TypeConversion("Expected number for age".into()))?
                    as u32;
            },
            "location" => {
                self.location = value.as_str()
                    .ok_or_else(|| Error::TypeConversion("Expected string for location".into()))?
                    .to_string();
            },
            "tags" => {
                self.tags = value.as_array()
                    .ok_or_else(|| Error::TypeConversion("Expected array for tags".into()))?
                    .iter()
                    .map(|v| v.as_str()
                        .ok_or_else(|| Error::TypeConversion("Expected string array for tags".into()))
                        .map(|s| s.to_string()))
                    .collect::<Result<_, _>>()?;
            },
            _ => return Err(Error::Field(format!("Unknown field: {}", field))),
        }
        Ok(())
    }
}

#[test]
fn test_simple_enrichment() -> Result<(), Error> {
    let mut user = User::default();
    
    let data = json!({
        "user": {
            "first_name": "John",
            "last_name": "Doe",
            "details": {
                "age": 30,
                "city": "New York",
                "country": "USA"
            }
        }
    });
    
    let spec = r#"{
        "name": {
            "path": "$.user.first_name",
            "transform": {
                "type": "concat",
                "additional_paths": ["$.user.last_name"],
                "separator": " "
            }
        },
        "age": "$.user.details.age",
        "location": {
            "path": "$.user.details.city",
            "transform": {
                "type": "concat",
                "additional_paths": ["$.user.details.country"],
                "separator": ", "
            }
        }
    }"#;
    
    user.enrich(&data, spec)?;
    
    assert_eq!(user.name, "John Doe");
    assert_eq!(user.age, 30);
    assert_eq!(user.location, "New York, USA");
    
    Ok(())
}

#[test]
fn test_transforms() -> Result<(), Error> {
    let mut user = User::default();
    
    let data = json!({
        "profile": {
            "name": "john doe",
            "age": 25,
            "tags_string": "developer,rust,backend"
        }
    });
    
    let spec = r#"{
        "name": {
            "path": "$.profile.name",
            "transform": {
                "type": "function",
                "name": "uppercase"
            }
        },
        "age": "$.profile.age",
        "tags": {
            "path": "$.profile.tags_string",
            "transform": {
                "type": "split",
                "separator": ","
            }
        }
    }"#;
    
    user.enrich(&data, spec)?;
    
    assert_eq!(user.name, "JOHN DOE");
    assert_eq!(user.age, 25);
    assert_eq!(user.tags, vec!["developer", "rust", "backend"]);
    
    Ok(())
}

#[test]
fn test_missing_path() {
    let mut user = User::default();
    
    let data = json!({});
    let spec = r#"{"name": "$.nonexistent.path"}"#;
    
    let result = user.enrich(&data, spec);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::Path(_)));
}

#[test]
fn test_invalid_type() {
    let mut user = User::default();
    
    let data = json!({"age": "not a number"});
    let spec = r#"{"age": "$.age"}"#;
    
    let result = user.enrich(&data, spec);
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::TypeConversion(_)));
}