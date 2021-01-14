use regex::Regex;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CompiledString {
    pub raw: String,
    pub masked: String,
}

pub fn compile_string(value: &str, data: &Value) -> CompiledString {
    let mut raw_value = value.to_string();
    let mut masked_value = value.to_string();

    let search = Regex::new(r"\$\{\{(.*?)}}").unwrap();
    let hits = search.captures_iter(&value);

    for hit in hits {
        // nothing to do here becuase it doesn't have any value
        if hit.iter().count() < 2 {
            continue;
        }

        let original_value = &hit[0];
        let inside_value = hit[1].trim();

        // grab parts of inside_value to check if there's a mask
        let parts: Vec<&str> = inside_value.split(" ").collect();

        // figure out pointer and if we should mask value
        let (pointer, has_mask) = match parts.len() {
            1 => {
                let pointer = format!("/{}", parts[0].replace(".", "/"));
                (pointer, false)
            }
            _ => {
                if parts.len() == 2 {
                    let pointer = format!("/{}", parts[1].replace(".", "/"));
                    (pointer, true)
                } else {
                    panic!("invalid template [{}]", original_value);
                }
            }
        };

        // grab data at pointer
        let data = data.pointer(&pointer).unwrap_or(&Value::Null);

        // handle case where no data was found
        if data.is_null() {
            raw_value = raw_value.replace(original_value, "");
            masked_value = masked_value.replace(original_value, "");
            continue;
        }

        // data needs to be a string
        let new_raw_value = match data {
            Value::String(string) => string.to_string(),
            value => {
                format!("{}", value)
            }
        };
        let new_masked_value = match has_mask {
            false => new_raw_value.to_string(),
            true => "****".to_string(),
        };

        raw_value = raw_value.replace(original_value, &new_raw_value);
        masked_value = masked_value.replace(original_value, &new_masked_value);
    }

    CompiledString {
        raw: raw_value,
        masked: masked_value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_plain() {
        let data = json!({ "first_name": "Test", "last_name": "McTest" });
        let test_string = "I am ${{ first_name }} ${{ mask last_name }}";
        let output = compile_string(test_string, &data);
        assert_eq!(output.raw, "I am Test McTest");
        assert_eq!(output.masked, "I am Test ****");
    }

    #[test]
    fn test_masking() {
        let data = json!({ "name": "Test McTest" });
        let test_string = "I am ${{ mask name }}";
        let output = compile_string(test_string, &data);
        assert_eq!(output.raw, "I am Test McTest");
        assert_eq!(output.masked, "I am ****");
    }

    #[test]
    fn test_nested_data() {
        let data = json!({
            "people": [{
                "name": "Test McTest"
            }]
        });
        let test_string = "I am ${{ mask people.0.name }}";
        let output = compile_string(test_string, &data);
        assert_eq!(output.raw, "I am Test McTest");
        assert_eq!(output.masked, "I am ****");
    }

    #[test]
    fn test_single_value() {
        let data = json!({
            "test": "test_string"
        });
        let test_string = "${{ mask test }}";
        let output = compile_string(test_string, &data);
        assert_eq!(output.raw, "test_string");
        assert_eq!(output.masked, "****");
    }
}
