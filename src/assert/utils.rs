use serde_json::Value;

pub fn to_number(v: &Value) -> Option<u32> {
    if v.is_number() {
        return Some(v.as_u64().unwrap() as u32);
    }
    if let Ok(number) = v.as_str().unwrap_or("").parse::<u32>() {
        return Some(number);
    }
    None
}

pub fn to_string(v: &Value) -> String {
    if v.is_string() {
        return v.as_str().unwrap().to_string();
    }

    // try to convert to string
    if let Some(v) = v.as_str() {
        return v.to_string();
    };

    format!("{}", v)
}

pub fn verb(is: &str, not: &str, is_inverted: bool) -> String {
    match is_inverted {
        true => not.to_string(),
        false => is.to_string(),
    }
}

pub fn to(not: bool) -> String {
    verb("to", "to not", not)
}

pub fn did_pass(result: bool, not: bool) -> bool {
    match not {
        true => !result,
        false => result,
    }
}
