use crate::{
    compile::{compile_value, CompiledValue},
    workflow::WorkflowConfigStep,
};
use serde_json::Value;

pub fn create_body(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    if let Some(graphql) = &step.graphql {
        let mut graphql_yaml = serde_yaml::Mapping::new();
        graphql_yaml.insert(
            "query".to_string().into(),
            serde_yaml::Value::String(graphql.query.clone()),
        );

        if let Some(variables) = &graphql.variables {
            graphql_yaml.insert("variables".to_string().into(), variables.clone());
        }

        return compile_value(Some(graphql_yaml.into()), &workflow_data);
    }

    let body = compile_value(step.body.clone(), &workflow_data);

    body
}
