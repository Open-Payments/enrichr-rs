use enrichr::*;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Enrichable, Default)]
struct UserProfile {
    pub display_name: String,
    pub username: String,
    pub contact: Contact,
    pub preferences: Preferences,
    pub tags: Vec<String>,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Contact {
    pub email: String,
    pub formatted_address: String,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct Preferences {
    pub theme: String,
    pub notifications_enabled: bool,
    pub language: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a user profile with default values
    let mut profile = UserProfile::default();

    // Complex source data
    let data_json = r#"{
        "user": {
            "personal": {
                "first_name": "John",
                "last_name": "Doe",
                "user_handle": "johndoe123"
            },
            "contact_info": {
                "email": "john.doe@example.com",
                "phone": "+1-555-123-4567",
                "address": {
                    "street": "123 Main St",
                    "city": "Springfield",
                    "state": "IL",
                    "zip": "62701"
                }
            },
            "settings": {
                "theme_preference": "dark",
                "notifications": true,
                "preferred_language": "en-US"
            },
            "metadata": {
                "tags": "premium,verified,active",
                "account_status": "active",
                "last_login": "2024-01-01T12:00:00Z"
            }
        }
    }"#;

    let data: HashMap<String, serde_json::Value> = serde_json::from_str(data_json)?;

    // Complex mapping rules
    let rules_json = r#"[
        {
            "source": ["$.user.personal.first_name", "$.user.personal.last_name"],
            "target": "$.display_name",
            "transform": {
                "type": "template",
                "params": {
                    "template": "{0} {1}"
                }
            }
        },
        {
            "source": "$.user.personal.user_handle",
            "target": "$.username",
            "transform": {
                "type": "toLowerCase"
            }
        },
        {
            "source": "$.user.contact_info.email",
            "target": "$.contact.email"
        },
        {
            "source": ["$.user.contact_info.address.street",
                      "$.user.contact_info.address.city",
                      "$.user.contact_info.address.state",
                      "$.user.contact_info.address.zip"],
            "target": "$.contact.formatted_address",
            "transform": {
                "type": "template",
                "params": {
                    "template": "{0}, {1}, {2} {3}"
                }
            }
        },
        {
            "source": "$.user.contact_info.phone",
            "target": "$.contact.phone"
        },
        {
            "source": "$.user.settings.theme_preference",
            "target": "$.preferences.theme"
        },
        {
            "source": "$.user.settings.notifications",
            "target": "$.preferences.notifications_enabled"
        },
        {
            "source": "$.user.settings.preferred_language",
            "target": "$.preferences.language"
        },
        {
            "source": "$.user.metadata.tags",
            "target": "$.tags",
            "transform": {
                "type": "split",
                "params": {
                    "delimiter": ","
                }
            }
        },
        {
            "source": "$.user.metadata.account_status",
            "target": "$.status",
            "transform": {
                "type": "toUpperCase"
            }
        }
    ]"#;

    let rules: Vec<MappingRule> = serde_json::from_str(rules_json)?;

    // Apply the enrichment
    profile.enrich(&data, &rules)?;

    // Verify the results
    println!("\nEnriched Profile:");
    println!("Display Name: {}", profile.display_name);
    println!("Username: {}", profile.username);
    println!("Email: {}", profile.contact.email);
    println!("Address: {}", profile.contact.formatted_address);
    println!("Phone: {:?}", profile.contact.phone);
    println!("Theme: {}", profile.preferences.theme);
    println!("Notifications Enabled: {}", profile.preferences.notifications_enabled);
    println!("Language: {}", profile.preferences.language);
    println!("Tags: {:?}", profile.tags);
    println!("Status: {}", profile.status);

    // Assert expected values
    assert_eq!(profile.display_name, "John Doe");
    assert_eq!(profile.username, "johndoe123");
    assert_eq!(profile.contact.email, "john.doe@example.com");
    assert_eq!(profile.contact.formatted_address, "123 Main St, Springfield, IL 62701");
    assert_eq!(profile.contact.phone, Some("+1-555-123-4567".to_string()));
    assert_eq!(profile.preferences.theme, "dark");
    assert_eq!(profile.preferences.notifications_enabled, true);
    assert_eq!(profile.preferences.language, "en-US");
    assert_eq!(profile.tags, vec!["premium", "verified", "active"]);
    assert_eq!(profile.status, "ACTIVE");

    Ok(())
}