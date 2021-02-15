use crate::assert::utils;
use serde_json::Value;

pub fn to_be_array(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_array();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be an array", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(
            to_be_array(&json!(["foo", "bar"]), &Value::Null, false),
            None
        );
        assert_eq!(to_be_array(&json!([]), &Value::Null, false), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_array(&json!(3), &Value::Null, false),
            Some("expected 3 to be an array".to_string())
        );
        assert_eq!(
            to_be_array(&json!(["foo", "bar"]), &Value::Null, true),
            Some("expected [\"foo\",\"bar\"] to not be an array".to_string())
        );
        assert_eq!(
            to_be_array(&Value::Null, &Value::Null, false),
            Some("expected null to be an array".to_string())
        );
    }
}
