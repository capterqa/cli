use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub file: Option<String>,
    pub name: String,
    pub url: Option<String>,
    pub method: Option<String>,
    pub env: Option<BTreeMap<String, serde_yaml::Value>>,
    pub steps: Vec<WorkflowConfigStep>,
    pub skip: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WorkflowConfigStep {
    pub name: String,
    pub id: Option<String>,
    pub url: Option<String>,
    pub method: Option<String>,
    pub query: Option<HashMap<String, String>>,
    pub headers: Option<BTreeMap<String, serde_yaml::Value>>,
    pub body: Option<serde_yaml::Value>,
    pub assertions: Vec<WorkflowConfigAssertion>,
    pub options: Option<WorkflowConfigStepOptions>,
    pub graphql: Option<WorkflowConfigGraphQlConfig>,
    pub skip: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum WorkflowConfigAssertion {
    assert(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfigStepOptions {
    pub mask: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowConfigGraphQlConfig {
    pub query: String,
    pub variables: Option<serde_yaml::Value>,
}

pub fn parse_yaml(yaml: String, path: String) -> Result<WorkflowConfig, serde_yaml::Error> {
    let mut yaml: WorkflowConfig = serde_yaml::from_str(&yaml)?;
    yaml.file = Some(path);

    Ok(yaml)
}
