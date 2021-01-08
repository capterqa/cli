use crate::{
    compile::{compile_value, CompiledValue},
    workflow::WorkflowConfigStep,
};
use serde_json::Value;

pub fn create_headers(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    let mut headers = serde_yaml::to_value(&step.headers).unwrap();

    if step.graphql.is_some() {
        headers["content-type"] = serde_yaml::Value::String("application/json".to_string());
    }

    compile_value(Some(headers), &workflow_data)
}
