use serde_json::{json, Map, Value};

/// Walks through a `Value` and maskes the value of any property with
/// a key defined in the `mask` argument.
pub fn deep_replace(value: &Value, mask: &Vec<String>) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map.iter() {
                if v.is_object() || v.is_array() {
                    return deep_replace(v, mask);
                };
                if mask.contains(k) {
                    new_map.insert(k.to_owned(), json!("****"));
                } else {
                    new_map.insert(k.to_owned(), v.clone());
                }
            }
            return new_map.into();
        }
        Value::Array(vec) => {
            let mut new_vec = vec![];
            for v in vec {
                new_vec.push(deep_replace(v, mask));
            }
            return new_vec.into();
        }
        _ => value.clone(),
    }
}
