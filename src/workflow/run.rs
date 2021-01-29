use crate::{
    assert::{assert_on_response, AssertionResultData},
    workflow::{
        create_body, create_headers, create_method, create_query, create_url, make_request, mask,
        RequestData, WorkflowConfig,
    },
};
use chrono::Utc;
use serde_json::json;
use std::env;

pub enum CallbackEvent<'a> {
    RunStart(&'a WorkflowConfig),
    RunDone(&'a WorkflowConfig, bool),
    StepStart(&'a WorkflowConfig, i32),
    StepDone(&'a WorkflowConfig, i32, &'a Vec<AssertionResultData>, bool),
    StepSkipped(&'a WorkflowConfig, i32),
}

pub fn run_workflow(
    config: &WorkflowConfig,
    mut callback: impl FnMut(CallbackEvent),
) -> Result<(Vec<RequestData>, bool), Box<dyn std::error::Error>> {
    callback(CallbackEvent::RunStart(config));
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

    // all requests are saved here
    // this is what we return in the end of all this
    let mut requests: Vec<RequestData> = Vec::with_capacity(config.steps.len());

    // keeps track of what step we are processing
    let mut step_index = 0;

    let mut workflow_passed = true;

    for step in config.steps.iter() {
        callback(CallbackEvent::StepStart(config, step_index));

        if step.skip.is_some() {
            callback(CallbackEvent::StepSkipped(config, step_index));
            continue;
        }

        let url = create_url(step, &config, &workflow_data);
        let body = create_body(&step, &workflow_data);
        let query = create_query(step, &workflow_data);
        let headers = create_headers(step, &workflow_data);
        let method = create_method(&step, &config);

        let mut request_data = RequestData {
            url: url.raw,
            name: step.name.clone(),
            query: Some(query.raw),
            body: Some(body.raw),
            method,
            headers: Some(headers.raw),
            created_at: Utc::now(),
            order: step_index,
            is_graphql: step.graphql.is_some(),
            response: None,
        };

        if let Some(id) = &step.id {
            workflow_data[&id]["request"] = serde_json::to_value(&request_data)?;
        }

        // do the request
        let mut response_data = make_request(&request_data);

        // assert on response
        let assertion_result_data =
            assert_on_response(&response_data, &step.assertions, &workflow_data);

        // check if step passed
        let step_passed = !assertion_result_data.iter().any(|r| r.passed == false);

        callback(CallbackEvent::StepDone(
            config,
            step_index,
            &assertion_result_data,
            step_passed,
        ));

        // move results to response_data
        response_data.assertion_results = Some(assertion_result_data);

        // add to workflow_data if the step has an id
        if let Some(id) = &step.id {
            workflow_data[&id]["response"] = serde_json::to_value(&response_data).unwrap();
        }

        // attach response to request
        request_data.response = Some(response_data);

        // clone the response data
        // this will be the one we return to be used in the ui
        let mut masked_response_data = request_data.response.unwrap().clone();

        // mask response if the user has added masked properties
        if let Some(options) = &step.options {
            masked_response_data = mask(masked_response_data, &options);
        }

        // make sure body is a string in the data we'll send to the webhook
        if let Some(body) = masked_response_data.body {
            masked_response_data.body = Some(serde_json::to_string(&body).unwrap().into());
        }

        // create a masked version of request/response
        let masked_request_data = RequestData {
            url: url.masked,
            query: Some(query.masked),
            // make sure body is a string
            body: Some(serde_json::to_string(&body.masked).unwrap().into()),
            headers: Some(headers.masked),
            response: Some(masked_response_data),
            ..request_data
        };

        requests.push(masked_request_data);

        // is one step fails, the whole worklfow is set to fail too
        if workflow_passed == true && step_passed == false {
            workflow_passed = false;
        }

        step_index += 1;
    }

    callback(CallbackEvent::RunDone(config, workflow_passed));

    Ok((requests, workflow_passed))
}
