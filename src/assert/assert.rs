use crate::assert::ValueAssertions;
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize, Clone)]
pub struct AssertionResultData {
    pub passed: bool,
    pub message: Option<String>,
    pub assertion: Assertion,
}

#[derive(Debug, Serialize)]
pub struct AssertionData {
    pub status: Option<u16>,
    pub body: serde_json::Value,
    pub headers: serde_json::Value,
    pub duration: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct Assertion {
    pub test: String,
    pub property: String,
    pub value: serde_json::Value,
}

pub fn assert(assertion: &Assertion, assertion_data: &AssertionData) -> Option<String> {
    let assertion_data_json = json!(&assertion_data);

    // create pointer
    let pointer = format!("/{}", assertion.property.replace(".", "/"));

    // grab data a pointer
    let data = assertion_data_json
        .pointer(&pointer)
        .unwrap_or(&Value::Null);

    let assert_fn = ValueAssertions::get(&assertion.test);

    let result = assert_fn(data, &assertion.value);

    return result;
}
