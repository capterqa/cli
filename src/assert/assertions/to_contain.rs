use crate::assert::utils;
use serde_json::Value;
use utils::did_pass;

pub fn to_contain(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = utils::to_string(a);
    let b_string = utils::to_string(b);

    let result = a_string.contains(&b_string);
    if did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} contain {}", a, utils::to(not), b))
}
