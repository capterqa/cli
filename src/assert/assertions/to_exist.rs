use crate::assert::utils;
use serde_json::Value;

// we might want to update this when we have something like this in place:
// https://stackoverflow.com/questions/44331037/how-can-i-distinguish-between-a-deserialized-field-that-is-missing-and-one-that
pub fn to_exist(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = !a.is_null();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} exist", a, utils::to(not)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_exist(&json!(0), &Value::Null, false), None);
        assert_eq!(to_exist(&json!(false), &Value::Null, false), None);
        assert_eq!(to_exist(&json!({}), &Value::Null, false), None);
        assert_eq!(to_exist(&Value::Null, &Value::Null, true), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_exist(&Value::Null, &Value::Null, false),
            Some("expected null to exist".to_string())
        );
    }
}
