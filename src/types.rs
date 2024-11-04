use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingRule {
    pub source: JsonPath,
    pub target: Target,
    pub transform: Option<Transform>,
    pub condition: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonPath {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Target {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    #[serde(rename = "type")]
    pub transform_type: TransformType,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransformType {
    ToString,
    ToUpperCase,
    ToLowerCase,
    Split,
    Concat,
    Replace,
    Substring,
    Template,
}