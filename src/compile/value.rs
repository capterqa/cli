use crate::compile::compile_string;
use serde::Serialize;
use serde_yaml::Value;

#[derive(Debug, Serialize)]
pub struct CompiledValue {
    pub raw: serde_yaml::Value,
    pub masked: serde_yaml::Value,
}

/// Compiles a serde_json Value.
pub fn compile_value(value: Option<serde_yaml::Value>, data: &serde_json::Value) -> CompiledValue {
    if let Some(value) = value {
        let raw = deep_keys(&value, &data, false);
        let masked = deep_keys(&value, &data, true);

        return CompiledValue { raw, masked };
    }

    return CompiledValue {
        raw: Value::Null,
        masked: Value::Null,
    };
}

fn deep_keys(
    value: &serde_yaml::Value,
    data: &serde_json::Value,
    masked: bool,
) -> serde_yaml::Value {
    match value {
        Value::String(val) => {
            let result = compile_string(val, data);

            let val = match masked {
                true => result.masked,
                false => result.raw,
            };

            // this will try to parse the value as a number or boolean
            // and if that fails we'll just return it as a string below
            let yaml_value = serde_yaml::from_str(&val);
            if let Ok(val) = yaml_value {
                return val;
            }

            return serde_yaml::Value::String(val);
        }
        Value::Mapping(map) => {
            let mut new_map = map.clone();
            for (k, v) in map.iter() {
                new_map[k] = deep_keys(v, data, masked);
            }
            return new_map.into();
        }
        Value::Sequence(vec) => {
            let mut new_vec = vec![];
            for v in vec {
                new_vec.push(deep_keys(v, data, masked));
            }
            return new_vec.into();
        }
        _ => value.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use serde_json::json;
    use serde_yaml::{from_str, Value};

    #[test]
    fn test_plain() {
        let yaml = indoc! {"
            ---
            name: ${{ user.name }}
        "};
        let test_value: Value = from_str(yaml).unwrap();
        let data = json!({ "user": { "name":  "Test McTest" }});
        let output = compile_value(Some(test_value), &data);

        assert_eq!(output.raw["name"], "Test McTest");
        assert_eq!(output.masked["name"], "Test McTest");
    }

    #[test]
    fn test_masking() {
        let yaml = indoc! {"
            ---
            name: ${{ mask user.name }}
        "};
        let test_value: Value = from_str(yaml).unwrap();
        let data = json!({ "user": { "name":  "Test McTest" }});
        let output = compile_value(Some(test_value), &data);

        assert_eq!(output.raw["name"], "Test McTest");
        assert_eq!(output.masked["name"], "****");
    }

    #[test]
    fn test_complex_structures() {
        let yaml = indoc! {"
            ---
            array: 
              - ${{ mask user.name }}
              - Age ${{ user.age }}
              - static text
            user:
              name: ${{ mask user.name }}
              age: ${{ mask user.age }}
        "};
        let test_value: Value = from_str(yaml).unwrap();
        let data = json!({ "user": { "name":  "Test McTest", "age": 30 }});
        let output = compile_value(Some(test_value), &data);

        assert_eq!(output.raw["array"][0], "Test McTest");
        assert_eq!(output.masked["array"][0], "****");
        assert_eq!(output.raw["array"][1], "Age 30");
        assert_eq!(output.masked["array"][1], "Age 30");
        assert_eq!(output.masked["array"][2], "static text");
        assert_eq!(output.raw["user"]["name"], "Test McTest");
        assert_eq!(output.masked["user"]["name"], "****");
        assert_eq!(output.raw["user"]["age"], 30);
        assert_eq!(output.masked["user"]["age"], "****");
    }

    #[test]
    fn test_value_types() {
        let yaml = indoc! {"
            ---
            string: ${{ string }}
            int: ${{ int }}
            float: ${{ float }}
            boolean: ${{ boolean }}
            static_string: test
            static_int: 5
            static_float: 1.5
            static_boolean: true
        "};
        let test_value: Value = from_str(yaml).unwrap();
        let data = json!({ "string": "test_string", "int": 5, "float": 1.5, "boolean": true });
        let output = compile_value(Some(test_value), &data);

        assert_eq!(output.raw["string"], "test_string");
        assert_eq!(output.raw["int"], 5);
        assert_eq!(output.raw["float"], 1.5);
        assert_eq!(output.raw["boolean"], true);

        assert_eq!(output.raw["static_string"], "test");
        assert_eq!(output.raw["static_int"], 5);
        assert_eq!(output.raw["static_float"], 1.5);
        assert_eq!(output.raw["static_boolean"], true);
    }

    #[test]
    fn test_empty_value() {
        let data = serde_json::Value::Null;
        let output = compile_value(None, &data);

        assert_eq!(output.raw["name"], serde_yaml::Value::Null);
        assert_eq!(output.masked["name"], serde_yaml::Value::Null);
    }

    #[test]
    fn test_nested_value() {
        let yaml = indoc! {"
            ---
            nested_value: ${{ nested }}
        "};
        let test_value: Value = from_str(yaml).unwrap();
        let data = json!({ "nested": { "a": "b", "c": ["d", "e"] } });
        let output = compile_value(Some(test_value), &data);

        let expected_output = indoc! {"
            ---
            a: b
            c:
              - d
              - e
        "};

        let expected_output: Value = from_str(expected_output).unwrap();
        assert_eq!(output.raw["nested_value"], expected_output);
    }
}
