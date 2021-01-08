use crate::workflow::{WorkflowConfig, WorkflowConfigStep};

pub fn create_method(step: &WorkflowConfigStep, config: &WorkflowConfig) -> String {
    if let Some(method) = &step.method {
        return method.clone();
    };

    if let Some(method) = &config.method {
        return method.clone();
    };

    if step.graphql.is_some() {
        return "POST".to_string();
    }

    "GET".to_string()
}
