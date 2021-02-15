use crate::assert::utils;
use serde_json::Value;

pub fn to_be_at_most(a: &Value, b: &Value, not: bool) -> Option<String> {
    if let (Some(a_number), Some(b_number)) = (utils::to_number(a), utils::to_number(b)) {
        let result = a_number <= b_number;
        if utils::did_pass(result, not) {
            return None;
        }

        return Some(format!(
            "expected {} {} be at most {}",
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
        assert_eq!(to_be_at_most(&json!(3), &json!(3), false), None);
        assert_eq!(to_be_at_most(&json!(3), &json!(4), false), None);
        assert_eq!(to_be_at_most(&json!(0), &json!(1), false), None);
        assert_eq!(to_be_at_most(&json!(-300), &json!(-100), false), None);
        assert_eq!(to_be_at_most(&json!(0.3), &json!(0.6), false), None);
    }

    #[test]
    fn test_fail() {
        assert_eq!(
            to_be_at_most(&json!(4), &json!(3), false),
            Some("expected 4 to be at most 3".to_string())
        );
        assert_eq!(
            to_be_at_most(&json!(-3), &json!(-2), true),
            Some("expected -3 to not be at most -2".to_string())
        );
    }

    #[test]
    fn test_invalid_type() {
        assert_eq!(
            to_be_at_most(&json!("foo"), &json!(3), false),
            Some("expected \"foo\" and 3 to be numbers".to_string())
        );
    }
}
