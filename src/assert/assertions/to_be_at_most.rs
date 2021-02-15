use crate::assert::utils;
use serde_json::Value;

pub fn to_be_at_most(a: &Value, b: &Value, not: bool) -> Option<String> {
    if let (Some(a_number), Some(b_number)) = (utils::to_number(a), utils::to_number(b)) {
        let result = a_number <= b_number;
        if utils::did_pass(result, not) {
            return None;
        }
    }

    Some(format!(
        "expected {} {} be at most {}",
        a,
        utils::to(not),
        b
    ))
}
