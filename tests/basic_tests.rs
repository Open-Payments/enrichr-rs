use enrichr::Enrichable;
use serde::Deserialize;
use std::collections::HashMap;
use serde_json::json;

#[derive(Enrichable, Default, Debug, Deserialize, PartialEq)]
struct SimpleUser {
    name: String,
    age: u32,
}

#[test]
fn test_simple_enrichment() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("user".to_string(), json!({
        "name": "John Doe",
        "details": {
            "age": 30
        }
    }));

    let spec = r#"{
        "name": "$.user.name",
        "age": "$.user.details.age"
    }"#;

    let mut user = SimpleUser::default();
    user.enrich(&data, spec)?;

    assert_eq!(user, SimpleUser {
        name: "John Doe".to_string(),
        age: 30,
    });

    Ok(())
}