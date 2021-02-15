use crate::assert::utils;
use serde_json::Value;

pub fn to_be_true(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if let Some(val) = a.as_bool() {
        let result = val == true;
        if utils::did_pass(result, not) {
            return None;
        }
    }

    Some(format!("expected {} {} be true", a, utils::to(not)))
}
