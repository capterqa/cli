use crate::assert::utils;
use regex::Regex;
use serde_json::Value;

pub fn to_match(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = utils::to_string(a);
    let b_string = utils::to_string(b);
    let b_string = b_string.replace("{", "\\{");

    let search = Regex::new(&b_string).unwrap();
    let did_match = search.captures_iter(&a_string);
    let result = did_match.count() > 0;
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} match {}", a, utils::to(not), b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_match(&json!("foo bar"), &json!("foo"), false), None);
        assert_eq!(to_match(&json!("123"), &json!(1), false), None);
        assert_eq!(
            to_match(&json!({ "test": true }), &json!(true), false),
            None
        );
        assert_eq!(
            to_match(
                &json!({ "test": { "user": "Test McTest" } }),
                &json!("Test McTest"),
                false
            ),
            None
        );
        assert_eq!(
            to_match(
                &json!({ "nested": { "foo": "bar" } }),
                &json!({ "foo": "bar" }),
                false
            ),
            None
        );
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_match(&json!(true), &json!("false"), false),
            Some("expected true to match \"false\"".to_string())
        );
        assert_eq!(
            to_match(&json!(["foo bar", "bar"]), &json!("baz"), false),
            Some("expected [\"foo bar\",\"bar\"] to match \"baz\"".to_string())
        );
        assert_eq!(
            to_match(&json!(["foo", "bar"]), &json!("foo"), true),
            Some("expected [\"foo\",\"bar\"] to not match \"foo\"".to_string())
        );
    }
}
