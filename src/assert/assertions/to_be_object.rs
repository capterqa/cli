use crate::assert::utils;
use serde_json::Value;

pub fn to_be_object(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_object();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be an object", a, utils::to(not)))
}
