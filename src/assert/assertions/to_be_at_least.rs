use crate::assert::utils;
use serde_json::Value;

pub fn to_be_at_least(a: &Value, b: &Value, not: bool) -> Option<String> {
    if let (Some(a_number), Some(b_number)) = (utils::to_number(a), utils::to_number(b)) {
        let result = a_number >= b_number;
        if utils::did_pass(result, not) {
            return None;
        }

        return Some(format!(
            "expected {} {} be at least {}",
            a,
            utils::to(not),
            b
        ));
    }

    Some(format!("expected {} and {} to be numbers", a, b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_pass() {
        assert_eq!(to_be_at_least(&json!(5), &json!(5), false), None);
        assert_eq!(to_be_at_least(&json!(5), &json!(4), false), None);
        assert_eq!(to_be_at_least(&json!(1), &json!(0), false), None);
        assert_eq!(to_be_at_least(&json!(-100), &json!(-300), false), None);
        assert_eq!(to_be_at_least(&json!(0.6), &json!(0.5), false), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_at_least(&json!(4), &json!(5), false),
            Some("expected 4 to be at least 5".to_string())
        );
        assert_eq!(
            to_be_at_least(&json!(-2), &json!(-3), true),
            Some("expected -2 to not be at least -3".to_string())
        );
    }

    #[test]
    fn test_invalid_type() {
        assert_eq!(
            to_be_at_least(&json!("foo"), &json!(5), false),
            Some("expected \"foo\" and 5 to be numbers".to_string())
        );
    }
}
