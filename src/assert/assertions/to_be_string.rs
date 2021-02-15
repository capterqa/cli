use crate::assert::utils;
use serde_json::Value;

pub fn to_be_string(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_string();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a string", a, utils::to(not)))
}
