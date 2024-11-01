use serde_json::Value;
use std::collections::HashMap;
use crate::error::Error;

pub trait Enrichable {
    fn enrich(&mut self, data: &HashMap<String, Value>, spec: &str) -> Result<(), Error>;
}
