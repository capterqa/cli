use crate::assert::utils;
use serde_json::Value;
use utils::{did_pass, to};

pub fn to_have_length(a: &Value, b: &Value, not: bool) -> Option<String> {
    // try array
    if let (Some(number), Some(array)) = (utils::to_number(b), a.as_array()) {
        let result = array.len() as u32 == number;
        if utils::did_pass(result, not) {
            return None;
        }

        return Some(format!(
            "expected array {} have length {} but got {}",
            utils::to(not),
            number,
            array.len(),
        ));
    };

    // try string
    if let (Some(number), Some(string)) = (utils::to_number(b), a.as_str()) {
        let result = string.len() as u32 == number;
        if did_pass(result, not) {
            return None;
        }

        return Some(format!(
            "expected string {} have length {} but got {}",
            to(not),
            number,
            string.len(),
        ));
    };

    Some(format!("expected {} to be string or array", a))
}
