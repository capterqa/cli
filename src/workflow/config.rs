use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::PathBuf,
};

/// A `WorkflowConfig` is the struct we convert the yaml files into.
///
/// When the CLI runs, it will convert every yaml into a `WorkflowConfig`,
/// and run them synchronously. Each `WorkflowConfig` includes one or more
/// *requests* that will be called when running it.
///
/// This struct is used by `serde-yaml` to parse the files, so every property
/// in this struct will be a valid value in those files.
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

impl WorkflowConfig {
    /// Create a WorfklowConfig from a path to a yaml file.
    ///
    /// This will read the file and parse it, and panic if
    /// something goes wrong.
    pub fn from_yaml_file(path: &PathBuf) -> WorkflowConfig {
        let path = path
            .clean()
            .into_os_string()
            .into_string()
            .expect("Failed to parse path");

        let file_content = fs::read_to_string(&path).expect(&format!("Failed to read {}", path));
        let mut workflow_config: WorkflowConfig =
            serde_yaml::from_str(&file_content).expect(&format!("Failed to parse {}", path));

        // if the user didn't set any file, add the actual file name
        if workflow_config.file.is_none() {
            workflow_config.file = Some(path.to_owned());
        }

        workflow_config
    }
}
