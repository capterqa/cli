use crate::utils::exit_with_code;
use regex::Regex;
use serde_json::Value;

pub struct ValueAssertions {}

impl ValueAssertions {
    pub fn get(name: &str) -> fn(&Value, &Value, bool) -> Option<String> {
        match name {
            "equal" => assert_equal,
            "contains" => assert_contains,
            "isBelow" => assert_is_below,
            "isAbove" => assert_is_above,
            "isArray" => assert_is_array,
            "hasLength" => assert_has_length,
            "isNotEmpty" => assert_is_not_empty,
            "notEqual" => assert_not_equal,
            "match" => assert_match,
            "notMatch" => assert_not_match,
            "isUndefined" => assert_is_undefined,
            val => exit_with_code(
                exitcode::CONFIG,
                Some(&format!("assertion not found: `{}`", val)),
            ),
        }
    }
}

fn assert_equal(a: &Value, b: &Value, not: bool) -> Option<String> {
    let result = value_to_string(a).eq(&value_to_string(b));

    if did_pass(result, not) {
        return None;
    }

    Some(format!(
        "expected {} {} equal {}",
        value_to_string(a),
        to(not),
        value_to_string(b),
    ))
}

fn assert_contains(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = value_to_string(a);
    let b_string = value_to_string(b);

    if a_string.contains(&b_string) {
        return None;
    }

    Some(format!("expected {} {} contain {}", a, to(not), b))
}

fn assert_is_below(a: &Value, b: &Value, not: bool) -> Option<String> {
    if let (Some(a_number), Some(b_number)) = (value_to_number(a), value_to_number(b)) {
        let result = a_number < b_number;
        if did_pass(result, not) {
            return None;
        }
    }

    Some(format!("expected {} {} be below {}", a, to(not), b))
}

fn assert_is_above(a: &Value, b: &Value, not: bool) -> Option<String> {
    if let (Some(a_number), Some(b_number)) = (value_to_number(a), value_to_number(b)) {
        let result = a_number > b_number;
        if did_pass(result, not) {
            return None;
        }
    }

    Some(format!("expected {} {} be above {}", a, to(not), b))
}

fn assert_is_array(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if a.is_array() {
        return None;
    }

    Some(format!(
        "expected {} to be an array but got {}",
        a,
        value_type(a)
    ))
}

fn assert_has_length(a: &Value, b: &Value, not: bool) -> Option<String> {
    // try array
    if let (Some(number), Some(array)) = (value_to_number(b), a.as_array()) {
        if array.len() as u32 == number {
            return None;
        }
        return Some(format!(
            "expected array to have length {} but got {}",
            number,
            array.len(),
        ));
    };

    // try string
    if let (Some(number), Some(string)) = (value_to_number(b), a.as_str()) {
        if string.len() as u32 == number {
            return None;
        }
        return Some(format!(
            "expected string to have length {} but got {}",
            number,
            string.len(),
        ));
    };

    Some(format!(
        "expected string or array but got {}",
        value_type(a)
    ))
}

fn assert_is_not_empty(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if a.is_array() {
        if a.as_array().unwrap().len() > 0 {
            return Some(format!("expected array to not be empty"));
        }
    }
    if a.is_null() {
        return Some(format!("expected null to not be empty"));
    }

    None
}

fn assert_not_equal(a: &Value, b: &Value, not: bool) -> Option<String> {
    if value_to_string(a).ne(&value_to_string(b)) {
        return None;
    }

    Some(format!("expected {} to not equal {}", a, b))
}

fn assert_match(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = &value_to_string(a);
    let b_string = &value_to_string(b);
    let search = Regex::new(b_string).unwrap();
    let did_match = search.captures_iter(a_string);

    if did_match.count() > 0 {
        return None;
    }

    Some(format!("expected {} to match {}", a, b))
}

fn assert_not_match(a: &Value, b: &Value, not: bool) -> Option<String> {
    let a_string = &value_to_string(a);
    let b_string = &value_to_string(b);
    let search = Regex::new(b_string).unwrap();
    let did_match = search.captures_iter(a_string);
    if did_match.count() == 0 {
        return None;
    }

    Some(format!("expected {} to not match {}", a, b))
}

fn assert_is_undefined(a: &Value, _b: &Value, not: bool) -> Option<String> {
    if a.is_null() {
        return None;
    }

    Some(format!("expected {} to be null", a))
}

fn value_to_number(v: &Value) -> Option<u32> {
    if v.is_number() {
        return Some(v.as_u64().unwrap() as u32);
    }
    if let Ok(number) = v.as_str().unwrap_or("").parse::<u32>() {
        return Some(number);
    }
    None
}

fn value_to_string(v: &Value) -> String {
    if v.is_string() {
        return v.as_str().unwrap().to_string();
    }

    // try to convert to string
    if let Some(v) = v.as_str() {
        return v.to_string();
    };

    format!("{}", v)
}

fn value_type(v: &Value) -> String {
    if v.is_string() {
        return "string".to_string();
    }
    if v.is_array() {
        return "array".to_string();
    }
    if v.is_boolean() {
        return "boolean".to_string();
    }
    if v.is_number() {
        return "number".to_string();
    }
    if v.is_null() {
        return "null".to_string();
    }
    if v.is_object() {
        return "object".to_string();
    }
    return "unknown".to_string();
}

fn verb(is: &str, not: &str, is_inverted: bool) -> String {
    match is_inverted {
        true => not.to_string(),
        false => is.to_string(),
    }
}

fn to(is_inverted: bool) -> String {
    verb("to", "to not", is_inverted)
}

fn did_pass(result: bool, not: bool) -> bool {
    match not {
        true => !result,
        false => result,
    }
}
