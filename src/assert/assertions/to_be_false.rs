use crate::assert::utils;
use serde_json::Value;

pub fn to_be_false(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if let Some(val) = a.as_bool() {
        let result = val == false;
        if utils::did_pass(result, not) {
            return None;
        }
    }

    Some(format!("expected {} {} be false", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_false(&json!(false), &Value::Null, false), None);
        assert_eq!(to_be_false(&json!(true), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_false(&Value::Null, &Value::Null, false),
            Some("expected null to be false".to_string())
        );
        assert_eq!(
            to_be_false(&json!(0), &Value::Null, false),
            Some("expected 0 to be false".to_string())
        );
        assert_eq!(
            to_be_false(&json!(false), &Value::Null, true),
            Some("expected false to not be false".to_string())
        );
    }
}
