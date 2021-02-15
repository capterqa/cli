use crate::assert::utils;
use serde_json::Value;

// we might want to update this when we have something like this in place:
// https://stackoverflow.com/questions/44331037/how-can-i-distinguish-between-a-deserialized-field-that-is-missing-and-one-that
pub fn to_be_undefined(a: &Value, _b: &Value, not: bool) -> Option<String> {
    let result = a.is_null();
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} be undefined", a, utils::to(not)))
}
