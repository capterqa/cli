use crate::workflow::{ResponseData, WorkflowConfigStepOptions};
use serde_json::{json, Map, Value};

pub fn mask(mut response_data: ResponseData, options: &WorkflowConfigStepOptions) -> ResponseData {
    if let Some(mask) = &options.mask {
        if mask.len() == 0 {
            return response_data;
        }

        let headers = deep_replace(&response_data.headers, mask);
        response_data.headers = headers;

        if let Some(body) = &response_data.body {
            let body = deep_replace(body, mask);
            response_data.body = Some(body);
        }
    }

    response_data
}

fn deep_replace(value: &Value, mask: &Vec<String>) -> Value {
    match value {
        Value::Object(map) => {
            let mut new_map = Map::new();
            for (k, v) in map.iter() {
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
