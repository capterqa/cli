use crate::assert::utils;
use serde_json::Value;
use utils::did_pass;

pub fn to_contain(a: &Value, b: &Value, not: bool) -> Option<String> {
    // handle arrays
    if let Some(array) = a.as_array() {
        let result = array.contains(b);
        if did_pass(result, not) {
            return None;
        }

        return Some(format!("expected {} {} contain {}", a, utils::to(not), b));
    }

    // handle everything else by converting it to
    // a string and matching on that
    let a_string = utils::to_string(a);
    let b_string = utils::to_string(b);

    let result = a_string.contains(&b_string);
    if did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} contain {}", a, utils::to(not), b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_contain(&json!("foo bar"), &json!("bar"), false), None);
        assert_eq!(to_contain(&json!(true), &json!(true), false), None);
        assert_eq!(
            to_contain(&json!(["foo", "bar"]), &json!("foo"), false),
            None
        );
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_contain(&json!(true), &json!("false"), false),
            Some("expected true to contain \"false\"".to_string())
        );
        assert_eq!(
            to_contain(&json!(["foo bar", "bar"]), &json!("foo"), false),
            Some("expected [\"foo bar\",\"bar\"] to contain \"foo\"".to_string())
        );
        assert_eq!(
            to_contain(&json!(["foo", "bar"]), &json!("foo"), true),
            Some("expected [\"foo\",\"bar\"] to not contain \"foo\"".to_string())
        );
    }
}
