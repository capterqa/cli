use crate::assert::utils;
use serde_json::Value;

pub fn to_be_boolean(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_boolean();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a boolean", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_boolean(&json!(true), &Value::Null, false), None);
        assert_eq!(to_be_boolean(&json!(false), &Value::Null, false), None);
        assert_eq!(to_be_boolean(&json!("foo"), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_boolean(&json!("foo"), &Value::Null, false),
            Some("expected \"foo\" to be a boolean".to_string())
        );
        assert_eq!(
            to_be_boolean(&json!("true"), &Value::Null, false),
            Some("expected \"true\" to be a boolean".to_string())
        );
        assert_eq!(
            to_be_boolean(&json!(0), &Value::Null, false),
            Some("expected 0 to be a boolean".to_string())
        );
    }
}
