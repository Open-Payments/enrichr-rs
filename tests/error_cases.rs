use enrichr::Enrichable;
use enrichr::error::Error;

#[test]
fn test_invalid_spec() {
    let mut user = SimpleUser::default();
    let data = HashMap::new();
    let spec = r#"{ invalid json }"#;

    match user.enrich(&data, spec) {
        Err(Error::InvalidSpec(_)) => {},
        _ => panic!("Expected InvalidSpec error"),
    }
}

#[test]
fn test_path_not_found() {
    let mut user = SimpleUser::default();
    let mut data = HashMap::new();
    data.insert("user".to_string(), json!({}));
    
    let spec = r#"{
        "name": "$.user.nonexistent.field",
        "age": "$.user.details.age"
    }"#;

    match user.enrich(&data, spec) {
        Err(Error::PathNotFound(_)) => {},
        _ => panic!("Expected PathNotFound error"),
    }
}

#[test]
fn test_type_conversion_error() {
    let mut user = SimpleUser::default();
    let mut data = HashMap::new();
    data.insert("user".to_string(), json!({
        "name": "John",
        "details": {
            "age": "thirty" // age should be a number
        }
    }));

    let spec = r#"{
        "name": "$.user.name",
        "age": "$.user.details.age"
    }"#;

    match user.enrich(&data, spec) {
        Err(Error::TypeConversion { .. }) => {},
        _ => panic!("Expected TypeConversion error"),
    }
}
