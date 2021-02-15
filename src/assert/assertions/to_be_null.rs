use crate::assert::utils;
use serde_json::Value;

pub fn to_be_null(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_null();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be null", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_null(&Value::Null, &Value::Null, false), None);
        assert_eq!(to_be_null(&json!(0), &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_null(&json!("null"), &Value::Null, false),
            Some("expected \"null\" to be null".to_string())
        );
        assert_eq!(
            to_be_null(&json!(0), &Value::Null, false),
            Some("expected 0 to be null".to_string())
        );
        assert_eq!(
            to_be_null(&Value::Null, &Value::Null, true),
            Some("expected null to not be null".to_string())
        );
    }
}
