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
            // use handlebars to compile value
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
