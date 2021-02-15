use crate::assert::utils;
use serde_json::Value;

pub fn to_be_empty(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if let Some(array) = a.as_array() {
        let result = array.len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected {} {} be empty", a, utils::to(not)));
        }
        return None;
    }

    if let Some(string) = a.as_str() {
        let result = string.len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected {} {} be empty", a, utils::to(not)));
        }
        return None;
    }

    if let Some(object) = a.as_object() {
        let result = object.keys().len() > 0;
        if utils::did_pass(result, not) {
            return Some(format!("expected {} {} be empty", a, utils::to(not)));
        }
        return None;
    }

    return Some(format!("expected a string, array or object"));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_empty(&json!([]), &Value::Null, false), None);
        assert_eq!(to_be_empty(&json!(""), &Value::Null, false), None);
        assert_eq!(to_be_empty(&json!({}), &Value::Null, false), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_empty(&Value::Null, &Value::Null, false),
            Some("expected a string, array or object".to_string())
        );

        assert_eq!(
            to_be_empty(&json!([3]), &Value::Null, false),
            Some("expected [3] to be empty".to_string())
        );
        assert_eq!(
            to_be_empty(&json!([]), &Value::Null, true),
            Some("expected [] to not be empty".to_string())
        );
        assert_eq!(
            to_be_empty(&json!("foo"), &Value::Null, false),
            Some("expected \"foo\" to be empty".to_string())
        );
        assert_eq!(
            to_be_empty(&json!({"foo": "bar"}), &Value::Null, false),
            Some("expected {\"foo\":\"bar\"} to be empty".to_string())
        );
    }
}
