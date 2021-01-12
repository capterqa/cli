use handlebars::JsonRender;
use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext, RenderError};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct CompiledString {
    pub raw: String,
    pub masked: String,
}

pub fn compile_string(value: &str, data: &Value) -> CompiledString {
    let mut handlebars_raw = Handlebars::new();
    handlebars_raw.register_helper("mask", Box::new(fake_mask_helper));

    let mut handlebars_masked = Handlebars::new();
    handlebars_masked.register_helper("mask", Box::new(mask_helper));

    let value = value.replace("${{", "{{");

    // create a raw version where data is visible
    let raw = handlebars_raw.render_template(&value, data).unwrap();

    // create a masked version where mask is converted to ****
    let masked = handlebars_masked.render_template(&value, data).unwrap();

    CompiledString { raw, masked }
}

fn mask_helper(
    _: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    out.write("****".as_ref())?;
    return Ok(());
}

fn fake_mask_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).unwrap();
    out.write(param.value().render().as_ref())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_plain() {
        let data = json!({ "name": "Test McTest" });
        let test_string = "I am ${{ name }}";
        let output = compile_string(test_string, &data);
        assert_eq!(output.raw, "I am Test McTest");
        assert_eq!(output.masked, "I am Test McTest");
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
