use crate::assert::utils;
use serde_json::Value;

pub fn to_be_array(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_array();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be an array", a, utils::to(not)))
}
