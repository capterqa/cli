use crate::assert::utils;
use serde_json::Value;

pub fn to_be_number(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_number();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be a number", a, utils::to(not)))
}
