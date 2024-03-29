use crate::utils::exit_with_code;
use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fs::{create_dir_all, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
};

const EXAMPLE_WORKFLOW: &str = r#"name: example
env:
  URL: https://fake-api.capter.io
steps:
  - name: check health
    url: GET ${{ env.URL }}/api/health
    assertions:
      - !expect status to_equal 200
      - !expect body.ok to_equal true
"#;

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
    pub headers: Option<BTreeMap<String, serde_yaml::Value>>,
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
    expect(String),
    expect_not(String),
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
    /// This will read the file and parse it, and exit if
    /// something goes wrong.
    pub fn from_yaml_file(path: &PathBuf) -> WorkflowConfig {
        let path = match path.clean().into_os_string().into_string() {
            Ok(path) => path,
            _ => exit_with_code(exitcode::CONFIG, Some(&format!("Invalid path"))),
        };

        let yaml = match read_to_string(&path) {
            Ok(val) => val,
            _ => exit_with_code(exitcode::CONFIG, Some(&format!("Failed to read {}", path))),
        };

        let result = WorkflowConfig::from_yaml(yaml);

        match result {
            Ok(mut workflow_config) => {
                // if the user didn't set any file, add the actual file name
                if workflow_config.file.is_none() {
                    workflow_config.file = Some(path.to_owned());
                }

                workflow_config
            }
            Err(err) => {
                let message = format!("Failed to parse {}: {}", path, &err.to_string());
                exit_with_code(exitcode::CONFIG, Some(&message));
            }
        }
    }

    pub fn from_yaml(yaml: String) -> Result<WorkflowConfig, serde_yaml::Error> {
        // we use a custom tag !!expect, which yaml-serde currently can't parse
        // so we need to replace it with !expect_not manually before parsing
        let yaml = str::replace(&yaml, "!!expect", "!expect_not");

        serde_yaml::from_str(&yaml)
    }

    pub fn create_example() {
        // make sure we have a folder
        create_dir_all(".capter").unwrap();

        // create the file
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(".capter/example.test.yml")
            .unwrap();

        // empty file
        file.set_len(0).unwrap();

        // add example code
        write!(file, "{}", EXAMPLE_WORKFLOW).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_file() {
        let config = WorkflowConfig::from_yaml_file(&PathBuf::from("./.capter/posts.yml"));
        assert_eq!(config.name, "posts");
    }

    #[test]
    fn test_from_yaml() {
        let yaml = indoc! {"
            ---
            name: test
            steps:
              - name: step 1
                id: test
                url: http://localhost:3002/test
                assertions:
                  - !expect status equal 200
              - name: step 2
                id: test
                url: http://localhost:3002/test/1
                assertions:
                  - !expect status equal 200
            "
        };
        let config = WorkflowConfig::from_yaml(yaml.into()).unwrap();
        assert_eq!(config.name, "test");
        assert_eq!(config.steps.len(), 2);
        assert_eq!(config.steps[0].name, "step 1");
    }

    #[test]
    #[should_panic(expected = "missing field `assertions`")]
    fn test_bad_no_assertions() {
        let yaml = indoc! {"
            ---
            name: test
            steps:
              - name: step 1
            "
        };
        WorkflowConfig::from_yaml(yaml.into()).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing field `name`")]
    fn test_bad_no_name() {
        let yaml = indoc! {"
            ---
            steps:
              - name: step 1
                assertions:
                  - !expect status equal 200
            "
        };
        WorkflowConfig::from_yaml(yaml.into()).unwrap();
    }
}
