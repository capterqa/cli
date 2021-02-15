use crate::assert::utils;
use serde_json::Value;

pub fn to_be_empty(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if let Some(array) = a.as_array() {
        let result = array.len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected array {} be empty", utils::to(not)));
        }
    }

    if let Some(string) = a.as_str() {
        let result = string.len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected string {} be empty", utils::to(not)));
        }
    }

    if let Some(object) = a.as_object() {
        let result = object.keys().len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected object {} be empty", utils::to(not)));
        }
    }

    if a.is_null() {
        return Some(format!("expected a value"));
    }

    None
}
