use crate::assert::utils;
use serde_json::Value;

pub fn to_be_string(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_string();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a string", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_string(&json!(""), &Value::Null, false), None);
        assert_eq!(to_be_string(&json!("foo bar"), &Value::Null, false), None);
        assert_eq!(to_be_string(&json!(1), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_string(&json!(true), &Value::Null, false),
            Some("expected true to be a string".to_string())
        );
        assert_eq!(
            to_be_string(&Value::Null, &Value::Null, false),
            Some("expected null to be a string".to_string())
        );
        assert_eq!(
            to_be_string(&json!("foo"), &Value::Null, true),
            Some("expected \"foo\" to not be a string".to_string())
        );
    }
}
