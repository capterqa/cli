use crate::assert::utils;
use serde_json::Value;

pub fn to_be_object(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_object();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be an object", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_object(&json!({}), &Value::Null, false), None);
        assert_eq!(
            to_be_object(&json!({"foo": "bar"}), &Value::Null, false),
            None
        );
        assert_eq!(to_be_object(&json!("foo bar"), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_object(&json!("{}"), &Value::Null, false),
            Some("expected \"{}\" to be an object".to_string())
        );
        assert_eq!(
            to_be_object(&json!([]), &Value::Null, false),
            Some("expected [] to be an object".to_string())
        );
        assert_eq!(
            to_be_object(&Value::Null, &Value::Null, false),
            Some("expected null to be an object".to_string())
        );
        assert_eq!(
            to_be_object(&json!({"foo": "bar"}), &Value::Null, true),
            Some("expected {\"foo\":\"bar\"} to not be an object".to_string())
        );
    }
}
