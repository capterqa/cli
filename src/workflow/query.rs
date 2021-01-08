use crate::{
    compile::{compile_value, CompiledValue},
    workflow::WorkflowConfigStep,
};
use serde_json::Value;

pub fn create_query(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    let query = Some(serde_yaml::to_value(&step.query).unwrap());
    compile_value(query, &workflow_data)
}
