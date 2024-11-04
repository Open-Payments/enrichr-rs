use enrichr::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Enrichable)]
struct User {
    first_name: String,
    last_name: String,
    email: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new user with empty/default values
    let mut user = User {
        first_name: String::new(),
        last_name: String::new(),
        email: None,
    };

    // Source data in JSON format
    let data_json = r#"{
        "user_data": {
            "full_name": "John Doe",
            "contact": {
                "email": "john.doe@example.com"
            }
        }
    }"#;

    // Parse the JSON data into a HashMap
    let data: HashMap<String, serde_json::Value> = serde_json::from_str(data_json)?;

    // Define mapping rules
    let rules_json = r#"[
        {
            "source": "$.user_data.full_name",
            "target": ["$.first_name", "$.last_name"],
            "transform": {
                "type": "split",
                "params": {
                    "delimiter": " "
                }
            }
        },
        {
            "source": "$.user_data.contact.email",
            "target": "$.email"
        }
    ]"#;

    // Parse the rules
    let rules: Vec<MappingRule> = serde_json::from_str(rules_json)?;

    // Apply the enrichment
    user.enrich(&data, &rules)?;

    // Verify the results
    assert_eq!(user.first_name, "John");
    assert_eq!(user.last_name, "Doe");
    assert_eq!(user.email, Some("john.doe@example.com".to_string()));

    println!("Enriched user: {:?}", user);
    Ok(())
}