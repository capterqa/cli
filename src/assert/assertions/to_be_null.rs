use crate::assert::utils;
use serde_json::Value;

pub fn to_be_null(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_null();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be null", a, utils::to(not)))
}
