use regex::Regex;
use serde_json::Value;

pub struct ValueAssertions {}

impl ValueAssertions {
    pub fn get(name: &str) -> fn(&Value, &Value) -> bool {
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
            val => panic!("assertion not found [{}]", val),
        }
    }
}

fn assert_equal(a: &Value, b: &Value) -> bool {
    value_to_string(a).eq(&value_to_string(b))
}

fn assert_contains(a: &Value, b: &Value) -> bool {
    let a_string = value_to_string(a);
    let b_string = value_to_string(b);

    a_string.contains(&b_string)
}

fn assert_is_below(a: &Value, b: &Value) -> bool {
    if let Some(a_number) = value_to_number(a) {
        if let Some(b_number) = value_to_number(b) {
            return a_number < b_number;
        }
    };
    false
}

fn assert_is_above(a: &Value, b: &Value) -> bool {
    if let Some(a_number) = value_to_number(a) {
        if let Some(b_number) = value_to_number(b) {
            return a_number > b_number;
        }
    };
    false
}

fn assert_is_array(a: &Value, _b: &Value) -> bool {
    a.is_array()
}

fn assert_has_length(a: &Value, b: &Value) -> bool {
    if !a.is_array() {
        return false;
    }
    if let Some(number) = value_to_number(b) {
        return a.as_array().unwrap().len() as u32 == number;
    };
    false
}

fn assert_is_not_empty(a: &Value, _b: &Value) -> bool {
    if a.is_array() {
        return a.as_array().unwrap().len() > 0;
    }
    if a.is_null() {
        return false;
    }
    true
}

fn assert_not_equal(a: &Value, b: &Value) -> bool {
    value_to_string(a).ne(&value_to_string(b))
}

fn assert_match(a: &Value, b: &Value) -> bool {
    let a_string = &value_to_string(a);
    let b_string = &value_to_string(b);
    let search = Regex::new(b_string).unwrap();
    let did_match = search.captures_iter(a_string);
    did_match.count() > 0
}

fn assert_not_match(a: &Value, b: &Value) -> bool {
    let a_string = &value_to_string(a);
    let b_string = &value_to_string(b);
    let search = Regex::new(b_string).unwrap();
    let did_match = search.captures_iter(a_string);
    did_match.count() == 0
}

fn assert_is_undefined(a: &Value, _b: &Value) -> bool {
    a.is_null()
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
