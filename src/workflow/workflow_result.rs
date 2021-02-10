use crate::{
    assert::AssertionResultData,
    workflow::{Request, RequestData, WorkflowConfig},
};
use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use serde_json::json;
use std::{env, time::Instant};

/// The result of a workflow is saved in to this struct.
/// It can be serialized to JSON and we pass it to the webhook
/// if that is set.
#[derive(Debug, Serialize)]
pub struct WorkflowResult {
    pub file: Option<String>,
    pub name: String,
    pub workflow: WorkflowConfig,
    pub created_at: DateTime<Utc>,
    pub run_time: i64,
    pub passed: bool,
    pub requests: Vec<RequestData>,
}

/// The `CallbackEvents` are called during the workflows lifetime.
/// Use the callback argument in `WorkflowResult::from_config` to
/// react to updates during the run.
pub enum CallbackEvent<'a> {
    RunStart(&'a WorkflowConfig),
    RunDone(&'a WorkflowConfig, bool),
    StepStart(&'a WorkflowConfig, i32),
    StepDone(&'a WorkflowConfig, i32, &'a Vec<AssertionResultData>, bool),
    StepSkipped(&'a WorkflowConfig, i32),
}

impl WorkflowResult {
    /// Runs through the steps defined in the `config` argument,
    /// and returns a result when all requests are done and asserted on.
    ///
    /// Use the callback argument to get continous updates from the run.
    pub fn from_config(
        config: &WorkflowConfig,
        mut callback: impl FnMut(CallbackEvent),
    ) -> Result<WorkflowResult, Box<dyn std::error::Error>> {
        // this is where we save all data available when creating the requests
        // if a workflow step as an `id` property, we will save the data like
        // `workflow_data.{id}.request`, `workflow_data.{id}.response`
        // including headers, arguments etc, which can all be used in the yaml files
        // to build up requests based on previous data
        let mut workflow_data = json!({});

        // add env to workflow data
        for (key, value) in env::vars() {
            workflow_data["env"][key] = value.into();
        }
        if let Some(env) = &config.env {
            for (key, value) in env {
                workflow_data["env"][key] = json!(value);
            }
        }

        callback(CallbackEvent::RunStart(config));

        // all requests are saved here
        // these are added to the WorkflowResult
        let mut requests: Vec<RequestData> = Vec::with_capacity(config.steps.len());

        // keeps track of what step we are processing
        let mut step_index = 0;

        let mut workflow_passed = true;

        // this is used to know for how long the workflow has run
        let timer = Instant::now();

        for step in config.steps.iter() {
            callback(CallbackEvent::StepStart(config, step_index));

            if step.skip.is_some() {
                callback(CallbackEvent::StepSkipped(config, step_index));
                continue;
            }

            let mut request = Request::new(config, step_index, &workflow_data);

            // add it to workflow_data if id is set
            if let Some(id) = &step.id {
                workflow_data[&id]["request"] = serde_json::to_value(request.data())?;
            }

            // do the request
            let response_data = request.call();

            // assert on response
            let assertion_result_data = request.assert_on_response(&step.assertions);

            // check if step passed
            let step_passed = !assertion_result_data.iter().any(|r| r.passed == false);

            callback(CallbackEvent::StepDone(
                config,
                step_index,
                &assertion_result_data,
                step_passed,
            ));

            // add to workflow_data if the step has an id
            if let Some(id) = &step.id {
                workflow_data[&id]["response"] = serde_json::to_value(response_data).unwrap();
            }

            // push masked data to our request array
            requests.push(request.data_masked());

            // is one step fails, the whole worklfow is set to fail too
            if workflow_passed == true && step_passed == false {
                workflow_passed = false;
            }

            step_index += 1;
        }

        // for how long it ran
        let run_time = timer.elapsed().as_millis() as i64;

        callback(CallbackEvent::RunDone(config, workflow_passed));

        Ok(WorkflowResult {
            name: config.name.to_owned(),
            file: config.file.to_owned(),
            workflow: config.to_owned(),
            created_at: Utc::now(),
            requests,
            run_time,
            passed: workflow_passed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::formatdoc;
    use mockito::mock;

    #[test]
    /// Tests a full workflows, almost like an integration test.
    /// The mocks will only work if body, header and query matches,
    /// so if you get a 503 error back in means something bad with those.
    fn test_run_workflow() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_body(r#"[{"id": "1"}]"#)
            .create();
        let _m2 = mock("GET", "/test/1")
            .with_status(200)
            .match_body(r#"{"id":1}"#)
            .match_header("id", "1")
            .match_query("id=1")
            .with_body(r#"{"hello": "world"}"#)
            .create();

        let yaml = formatdoc! {"
            ---
            name: test
            steps:
              - name: step 1
                id: test
                url: {url}/test
                assertions:
                  - !assert status equal 200
                  - !assert body.0.id equal 1
                  - !assert body.5.id equal 5
              - name: step 2
                url: {url}/test/{path}
                query:
                  id: {path}
                headers:
                  id: {path}
                body:
                  id: {path}
                assertions:
                  - !assert status equal 200
                  - !assert body.hello equal world
            ",
            url = url,
            path = "${{ test.response.body.0.id }}"
        };
        let workflow_config = WorkflowConfig::from_yaml(yaml.into());

        match workflow_config {
            Ok(workflow_config) => {
                let result = WorkflowResult::from_config(&workflow_config, |_| {});
                let result = result.unwrap();
                let response1 = result.requests[0].response.to_owned().unwrap();
                let response2 = result.requests[1].response.to_owned().unwrap();

                assert_eq!(result.requests.len(), 2);
                assert_eq!(response1.assertion_results[0].passed, true);
                assert_eq!(
                    response1.assertion_results[2].message,
                    Some("expected null to equal 5".to_string())
                );
                assert_eq!(response2.assertion_results[0].message, None);
            }
            _ => assert!(false),
        }
    }
}
