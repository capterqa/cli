use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};
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
) -> Result<(), RenderError> {
    let param = h
        .param(0)
        .ok_or(RenderError::new("path is required for format helper."))?;
    let rendered = format!("{}", param.value());
    out.write(rendered.as_ref())?;
    return Ok(());
}
