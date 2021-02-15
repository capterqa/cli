use crate::assert::utils;
use serde_json::Value;

pub fn to_be_true(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if let Some(val) = a.as_bool() {
        let result = val == true;
        if utils::did_pass(result, not) {
            return None;
        }
    }

    Some(format!("expected {} {} be true", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_true(&json!(true), &Value::Null, false), None);
        assert_eq!(to_be_true(&json!(false), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_true(&Value::Null, &Value::Null, false),
            Some("expected null to be true".to_string())
        );
        assert_eq!(
            to_be_true(&json!(1), &Value::Null, false),
            Some("expected 1 to be true".to_string())
        );
        assert_eq!(
            to_be_true(&json!(true), &Value::Null, true),
            Some("expected true to not be true".to_string())
        );
    }
}
