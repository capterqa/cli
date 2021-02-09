use crate::{
    assert::{assert, parse_assertion_string, AssertionData, AssertionResultData},
    compile::compile_string,
    utils::deep_replace,
    workflow::{WorkflowConfigAssertion, WorkflowConfigStepOptions},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};

#[derive(Debug, Serialize, Clone)]
pub struct ResponseData {
    pub created_at: DateTime<Utc>,
    pub status: Option<u16>,
    pub status_text: String,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
    pub response_time: i64,
    pub assertion_results: Vec<AssertionResultData>,
}

impl Default for ResponseData {
    fn default() -> ResponseData {
        ResponseData {
            created_at: Utc::now(),
            status_text: "".to_string(),
            headers: json!(Value::Null),
            response_time: 0,
            status: None,
            body: None,
            assertion_results: vec![],
        }
    }
}

impl ResponseData {
    pub fn from_result(
        result: Result<ureq::Response, ureq::Error>,
        response_time: i64,
    ) -> ResponseData {
        match result {
            Ok(response) => {
                let status = response.status();
                let headers_names = response.headers_names();

                // create a Value from headers
                let mut headers = serde_json::Map::new();
                for name in headers_names {
                    let value = response.header(&name);
                    headers.insert(name, json!(value));
                }

                // TODO: fix
                // let status_text = response.status_text();
                let status_text = "TODO".to_string();
                let body: Value = response.into_json().unwrap_or(json!(status_text));

                ResponseData {
                    response_time,
                    status: Some(status),
                    status_text: status_text.to_owned(),
                    body: Some(body),
                    headers: headers.into(),
                    ..Default::default()
                }
            }
            Err(ureq::Error::Status(code, response)) => ResponseData {
                response_time,
                status: Some(code),
                status_text: response.status_text().to_owned(),
                ..Default::default()
            },
            Err(error) => ResponseData {
                response_time,
                status_text: error.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn assert(
        &mut self,
        assertions: &Vec<WorkflowConfigAssertion>,
        workflow_data: &Value,
    ) -> Vec<AssertionResultData> {
        let mut assertions_results: Vec<AssertionResultData> = vec![];

        for assertion_string in assertions {
            let WorkflowConfigAssertion::assert(assertion_string) = assertion_string;
            let assertion_data = AssertionData {
                status: self.status,
                duration: self.response_time,
                body: self.body.to_owned().unwrap_or(Value::Null),
                headers: self.headers.to_owned(),
            };

            let assertion_string = compile_string(assertion_string, workflow_data);
            let assertion_raw = parse_assertion_string(&assertion_string.raw);
            let assertion_masked = parse_assertion_string(&assertion_string.masked);

            let result = assert(&assertion_raw, &assertion_data);
            let passed = result.is_none();

            let assertion_result_data = AssertionResultData {
                assertion: assertion_masked,
                message: result,
                passed,
            };

            assertions_results.push(assertion_result_data);
        }

        self.assertion_results = assertions_results;

        self.assertion_results.to_owned()
    }

    pub fn into_masked(&self, options: &Option<WorkflowConfigStepOptions>) -> Option<ResponseData> {
        let mut response = match options {
            Some(options) => self.mask(options),
            _ => self.to_owned(),
        };

        // make sure body is a string
        if let Some(body) = response.body {
            response.body = Some(serde_json::to_string(&body).unwrap().into());
        }

        Some(response)
    }

    pub fn mask(&self, options: &WorkflowConfigStepOptions) -> ResponseData {
        let mut response_result = self.clone();
        if let Some(mask) = &options.mask {
            if mask.len() == 0 {
                return response_result;
            }

            let headers = deep_replace(&response_result.headers, mask);
            response_result.headers = headers;

            if let Some(body) = &response_result.body {
                let body = deep_replace(body, mask);
                response_result.body = Some(body);
            }
        }

        response_result
    }
}
