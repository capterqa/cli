use serde_json::Value;

pub fn to_number(v: &Value) -> Option<f64> {
    if let Some(val) = v.as_f64() {
        return Some(val);
    }
    if let Ok(number) = v.as_str().unwrap_or("").parse::<f64>() {
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
