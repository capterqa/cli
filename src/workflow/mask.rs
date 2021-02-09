use crate::workflow::{ResponseData, WorkflowConfigStepOptions};
use serde_json::{json, Map, Value};

pub fn mask(response_result: &ResponseData, options: &WorkflowConfigStepOptions) -> ResponseData {
    let mut response_result = response_result.clone();
    if let Some(mask) = &options.mask {
        if mask.len() == 0 {
            return response_result;
        }

        let headers = deep_replace(&response_result.headers, mask);
        response_result.headers = headers;

        if let Some(body) = &response_result.body {
            let body = deep_replace(body, mask);
            response_result.body = Some(body);
        }
    }

    response_result
}

fn deep_replace(value: &Value, mask: &Vec<String>) -> Value {
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
