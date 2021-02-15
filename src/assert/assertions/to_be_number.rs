use crate::assert::utils;
use serde_json::Value;

pub fn to_be_number(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_number();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a number", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_number(&json!(-5), &Value::Null, false), None);
        assert_eq!(to_be_number(&json!(0), &Value::Null, false), None);
        assert_eq!(to_be_number(&json!(0.5), &Value::Null, false), None);
        assert_eq!(to_be_number(&json!("foo"), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_number(&json!("1"), &Value::Null, false),
            Some("expected \"1\" to be a number".to_string())
        );
        assert_eq!(
            to_be_number(&json!(true), &Value::Null, false),
            Some("expected true to be a number".to_string())
        );
        assert_eq!(
            to_be_number(&Value::Null, &Value::Null, false),
            Some("expected null to be a number".to_string())
        );
    }
}
