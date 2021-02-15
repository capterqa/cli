use crate::assert::utils;
use regex::Regex;
use serde_json::Value;

pub fn to_match(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = &utils::to_string(a);
    let b_string = &utils::to_string(b);
    let search = Regex::new(b_string).unwrap();
    let did_match = search.captures_iter(a_string);

    let result = did_match.count() > 0;
    if utils::did_pass(result, not) {
        return None;
    }

    Some(format!("expected {} {} match {}", a, utils::to(not), b))
}
