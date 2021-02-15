use crate::assert::utils;
use serde_json::Value;

pub fn to_be_boolean(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_boolean();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a boolean", a, utils::to(not)))
}
