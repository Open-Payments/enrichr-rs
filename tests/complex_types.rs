use enrichr::Enrichable;
use serde::Deserialize;

#[derive(Enrichable, Default, Debug, Deserialize, PartialEq)]
struct ComplexUser {
    name: String,
    age: u32,
    emails: Vec<String>,
    address: Address,
    status: Option<Status>,
}

#[derive(Default, Debug, Deserialize, PartialEq)]
struct Address {
    street: String,
    city: String,
}

#[derive(Debug, Deserialize, PartialEq)]
enum Status {
    Active,
    Inactive,
}

#[test]
fn test_complex_enrichment() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = HashMap::new();
    data.insert("user".to_string(), json!({
        "name": "John Doe",
        "details": {
            "age": 30,
            "contact": {
                "emails": ["john@example.com", "doe@example.com"]
            },
            "address": {
                "street": "123 Main St",
                "city": "New York"
            },
            "account_status": "Active"
        }
    }));

    let spec = r#"{
        "name": "$.user.name",
        "age": "$.user.details.age",
        "emails": "$.user.details.contact.emails",
        "address.street": "$.user.details.address.street",
        "address.city": "$.user.details.address.city",
        "status": "$.user.details.account_status"
    }"#;

    let mut user = ComplexUser::default();
    user.enrich(&data, spec)?;

    assert_eq!(user.name, "John Doe");
    assert_eq!(user.emails.len(), 2);
    assert_eq!(user.address.city, "New York");

    Ok(())
}