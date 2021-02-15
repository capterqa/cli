use crate::assert::assertions::prelude::*;
use crate::utils::exit_with_code;
use serde_json::Value;

pub struct ValueAssertions {}

pub const ASSERTION_TYPES: &[&str] = &[
    "to_be_ok",
    "to_equal",
    "to_be_above",
    "to_be_at_least",
    "to_be_below",
    "to_be_at_most",
    "to_be_true",
    "to_be_false",
    "to_be_null",
    "to_exist",
    "to_be_undefined",
    "to_be_object",
    "to_be_array",
    "to_be_string",
    "to_be_number",
    "to_be_boolean",
    "to_contain",
    "to_have_length",
    "to_be_empty",
    "to_match",
];

impl ValueAssertions {
    pub fn get(name: &str) -> fn(&Value, &Value, bool) -> Option<String> {
        match name {
            "to_equal" => to_equal,
            "to_be_above" => to_be_above,
            "to_be_at_least" => to_be_at_least,
            "to_be_below" => to_be_below,
            "to_be_at_most" => to_be_at_most,
            "to_have_length" => to_have_length,
            "to_be_true" => to_be_true,
            "to_be_false" => to_be_false,
            "to_be_null" => to_be_null,
            "to_exist" => to_exist,
            "to_be_undefined" => to_be_undefined,
            "to_be_object" => to_be_object,
            "to_be_array" => to_be_array,
            "to_be_string" => to_be_string,
            "to_be_number" => to_be_number,
            "to_be_boolean" => to_be_boolean,
            "to_contain" => to_contain,
            "to_be_empty" => to_be_empty,
            "to_match" => to_match,
            val => exit_with_code(
                exitcode::CONFIG,
                Some(&format!("assertion not found: `{}`", val)),
            ),
        }
    }
}
