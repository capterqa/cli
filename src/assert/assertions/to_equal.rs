use crate::assert::utils;
use serde_json::Value;

pub fn to_equal(a: &Value, b: &Value, not: bool) -> Option<String> {
    let result = utils::to_string(a).eq(&utils::to_string(b));

    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!(
        "expected {} {} equal {}",
        utils::to_string(a),
        utils::to(not),
        utils::to_string(b),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_boolean() {
        assert_eq!(to_equal(&json!(true), &json!(true), false), None);
        assert_eq!(
            to_equal(&json!(true), &json!(false), false),
            Some("expected true to equal false".to_string())
        );
        assert_eq!(
            to_equal(&json!(true), &json!(true), true),
            Some("expected true to not equal true".to_string())
        );
    }

    #[test]
    fn test_number() {
        assert_eq!(to_equal(&json!(0), &json!(0), false), None);
        assert_eq!(to_equal(&json!(0.4), &json!(0.4), false), None);
        assert_eq!(
            to_equal(&json!(0), &json!(1), false),
            Some("expected 0 to equal 1".to_string())
        );
        assert_eq!(
            to_equal(&json!(0.5), &json!(0.5), true),
            Some("expected 0.5 to not equal 0.5".to_string())
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(to_equal(&json!("foo"), &json!("foo"), false), None);
        assert_eq!(
            to_equal(&json!("foo"), &json!("bar"), false),
            Some("expected foo to equal bar".to_string())
        );
        assert_eq!(
            to_equal(&json!("foo"), &json!("foo"), true),
            Some("expected foo to not equal foo".to_string())
        );
    }

    #[test]
    fn test_object() {
        assert_eq!(
            to_equal(&json!({ "a": "b" }), &json!({ "a": "b" }), false),
            None
        );
        assert_eq!(
            to_equal(&json!({ "a": "b" }), &json!({ "c": "d" }), false),
            Some("expected {\"a\":\"b\"} to equal {\"c\":\"d\"}".to_string())
        );
        assert_eq!(
            to_equal(&json!({ "a": "b" }), &json!({ "a": "b" }), true),
            Some("expected {\"a\":\"b\"} to not equal {\"a\":\"b\"}".to_string())
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            to_equal(&json!(["a", "b"]), &json!(["a", "b"]), false),
            None
        );
        assert_eq!(
            to_equal(&json!(["a", "b"]), &json!(["c", "d"]), false),
            Some("expected [\"a\",\"b\"] to equal [\"c\",\"d\"]".to_string())
        );
        assert_eq!(
            to_equal(&json!(["a", "b"]), &json!(["a", "b"]), true),
            Some("expected [\"a\",\"b\"] to not equal [\"a\",\"b\"]".to_string())
        );
    }

    #[test]
    fn test_nested_object() {
        assert_eq!(
            to_equal(
                &json!({ "user": { "name": "Test", "friends": [{ "name": "Test friend" }] } }),
                &json!({ "user": { "name": "Test", "friends": [{ "name": "Test friend" }] } }),
                false
            ),
            None
        );
        assert_eq!(
            to_equal(
                &json!({ "user": { "name": "Test", "friends": [{ "name": "Test friend" }] } }),
                &json!({ "c": "d" }),
                false
            ),
            Some("expected {\"user\":{\"friends\":[{\"name\":\"Test friend\"}],\"name\":\"Test\"}} to equal {\"c\":\"d\"}".to_string())
        );
        assert_eq!(
            to_equal(
                &json!({ "user": { "name": "Test", "friends": [{ "name": "Test friend" }] } }),
                &json!({ "user": { "name": "Test", "friends": [{ "name": "Test friend" }] } }),
                true
            ),
            Some("expected {\"user\":{\"friends\":[{\"name\":\"Test friend\"}],\"name\":\"Test\"}} to not equal {\"user\":{\"friends\":[{\"name\":\"Test friend\"}],\"name\":\"Test\"}}".to_string())
        );
    }
}
