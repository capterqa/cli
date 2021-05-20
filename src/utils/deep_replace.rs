use serde_json::{json, Map, Value};

/// Walks through a `Value` and maskes the value of any property with
/// a key defined in the `mask` argument.
pub fn deep_replace(value: &Value, mask: &Vec<String>) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map.iter() {
                if v.is_object() || v.is_array() {
                    new_map.insert(k.to_owned(), deep_replace(v, mask));
                    return new_map.into();
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_nested_object() {
        let data = json!({
            "user": { "name": "Test", "secret": "abc" },
            "secret": 123,
        });
        let output = deep_replace(&data, &vec!["secret".to_string()]);
        assert_eq!(output["user"]["secret"], "****");
        assert_eq!(output["secret"], "****");
        assert_eq!(output["user"]["name"], "Test");
    }

    #[test]
    fn test_array() {
        let data = json!({
            "users": [{ "name": "Test", "secret": "abc" }],
        });
        let output = deep_replace(&data, &vec!["secret".to_string()]);
        assert_eq!(output["users"][0]["secret"], "****");
        assert_eq!(output["users"][0]["name"], "Test");
    }

    #[test]
    fn test_non_object() {
        let data = json!("test");
        let output = deep_replace(&data, &vec![]);
        assert_eq!(output, "test");
    }
}
