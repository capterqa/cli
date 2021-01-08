use crate::{
    compile::{compile_string, CompiledString},
    workflow::{WorkflowConfig, WorkflowConfigStep},
};
use serde_json::Value;

pub fn create_url(
    step: &WorkflowConfigStep,
    config: &WorkflowConfig,
    workflow_data: &Value,
) -> CompiledString {
    if let Some(url) = &step.url {
        return compile_string(&url, &workflow_data);
    };

    if let Some(url) = &config.url {
        return compile_string(&url, &workflow_data);
    };

    panic!("no url found");
}
