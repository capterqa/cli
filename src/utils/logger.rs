use crate::workflow::workflow_result::WorkflowResult;
use fs::OpenOptions;
use serde_json::json;
use std::{fs, io::Write};

pub struct Logger {}

impl Logger {
    pub fn new() -> Logger {
        Logger {}
    }

    pub fn log_workflow_results(&mut self, workflow_results: &Vec<WorkflowResult>) {
        fs::create_dir_all(".capter/logs").unwrap();

        for workflow in workflow_results {
            if workflow.passed {
                continue;
            }

            let workflow_path = &workflow.file.clone().unwrap();
            let workflow_name = workflow_path.split("/").last().unwrap();

            let file_path = format!(".capter/logs/{}.log", &workflow_name);

            // make sure we have a clean log file
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(file_path)
                .unwrap();

            // empty file
            file.set_len(0).unwrap();

            // title
            write!(file, "{} [{}]\n\n", workflow.name, &workflow_path).unwrap();
            write!(file, "Steps:\n\n").unwrap();

            for request in &workflow.requests {
                let step_passed = !request
                    .response
                    .clone()
                    .unwrap()
                    .assertion_results
                    .iter()
                    .any(|r| r.passed == false);

                write!(file, "  Name:\n    {}\n\n", request.name).unwrap();
                write!(file, "  Passed:\n    {}\n\n", step_passed).unwrap();
                write!(file, "  Created at:\n    {}\n\n", request.created_at).unwrap();
                write!(file, "  URL:\n    {} {}\n\n", request.method, request.url).unwrap();
                write!(file, "  Query:\n    {}\n\n", json!(request.query)).unwrap();
                write!(file, "  Headers:\n    {}\n\n", json!(request.headers)).unwrap();
                write!(file, "  Body:\n    {}\n\n", json!(request.body)).unwrap();

                let response = request.response.clone().unwrap();
                write!(file, "  Response:\n\n").unwrap();
                write!(file, "    Status:\n      {}\n\n", json!(response.status)).unwrap();
                write!(
                    file,
                    "    Status text:\n      {}\n\n",
                    json!(response.status_text)
                )
                .unwrap();
                write!(file, "    Headers:\n      {}\n\n", json!(response.headers)).unwrap();
                write!(file, "    Body:\n      {}\n\n", json!(response.body)).unwrap();

                write!(file, "  ---\n\n").unwrap();
            }

            write!(file, "---\n\n").unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{workflow::WorkflowConfig, CliOptions};
    use indoc::formatdoc;
    use mockito::mock;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_logger() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test").with_status(500).create();

        let yaml = formatdoc! {"
            ---
            name: test
            file: ./capter/logger-test.yml
            steps:
              - name: step 1
                id: test
                url: {url}/test
                assertions:
                  - !expect status to_equal 200
            ",
            url = url,
        };
        let workflow_config = WorkflowConfig::from_yaml(yaml.into());

        let result =
            WorkflowResult::from_config(&CliOptions::default(), &workflow_config.unwrap(), |_| {})
                .unwrap();
        let mut logger = Logger::new();
        logger.log_workflow_results(&vec![result]);

        let mut file = File::open(".capter/logs/logger-test.yml.log").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents.contains("test [./capter/logger-test.yml]"), true);

        fs::remove_file(".capter/logs/logger-test.yml.log").unwrap();
    }
}
