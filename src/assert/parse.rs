use crate::assert::{Assertion, ASSERTION_TYPES};
use serde_json::{json, Value};

pub fn parse_assertion_string(assertion_string: &str) -> Assertion {
    let mut parts = assertion_string.split(' ').collect::<Vec<&str>>();

    // pull the property from the array
    let property = parts[0];
    parts.remove(0);

    // !assert x isArray
    if parts.len() == 1 && ASSERTION_TYPES.contains(&parts[0]) {
        return Assertion {
            test: parts[0].to_owned(),
            property: property.to_owned(),
            value: Value::Null,
        };
    }

    // !assert x data.0.title isNotEmpty
    if parts.len() == 2 && ASSERTION_TYPES.contains(&parts[1]) {
        return Assertion {
            test: parts[1].to_owned(),
            property: format!("{}.{}", property, parts[0]),
            value: Value::Null,
        };
    }

    // !assert x isAbove 5
    if parts.len() > 1 && ASSERTION_TYPES.contains(&parts[0]) {
        let mut value = parts.clone();
        value.remove(0);
        let value = value.join(" ");

        return Assertion {
            test: parts[0].to_owned(),
            property: property.to_owned(),
            value: json!(value),
        };
    };

    // !assert x data.0.id equal 0
    if parts.len() > 2 && ASSERTION_TYPES.contains(&parts[1]) {
        let mut value = parts.clone();
        value.remove(0);
        value.remove(0);
        let value = value.join(" ");

        return Assertion {
            test: parts[1].to_owned(),
            property: format!("{}.{}", property, parts[0]),
            value: json!(value),
        };
    }

    panic!("could not parse assertion [{}]", assertion_string);
}