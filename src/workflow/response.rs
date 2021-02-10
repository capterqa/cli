use crate::{
    assert::{assert, AssertionData, AssertionResultData},
    utils::deep_replace,
    workflow::{WorkflowConfigAssertion, WorkflowConfigStepOptions},
};
use assert::Assertion;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use ureq::ErrorKind;

/// The result from a request. You can run assertions on it
/// using `.assert()`. This will populate `assertion_results`.
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
    /// Create a response from a `ureq` response.
    ///
    /// If the request fails, that does not mean you won't get
    /// a `ResponseData` back. Instead you will get a response
    /// with the error codes etc on it, and you can run assertions on that.
    ///
    /// Sometimes an error is what we're expecting so it's
    /// important that we handle that correctly.
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
            Err(error) => {
                // ending up here means there were NO response
                let status_text = match &error.kind() {
                    ErrorKind::Dns => format!("Could not connect to URL"),
                    _ => error.kind().to_string(),
                };

                ResponseData {
                    response_time,
                    status_text,
                    ..Default::default()
                }
            }
        }
    }

    /// Run assertions on the response.
    /// Populates `.assertion_results`.
    pub fn assert(
        &mut self,
        assertions: &Vec<WorkflowConfigAssertion>,
        workflow_data: &Value,
    ) -> Vec<AssertionResultData> {
        let mut assertions_results: Vec<AssertionResultData> = vec![];

        // loop through the assertions and run them
        for assertion_string in assertions {
            let WorkflowConfigAssertion::assert(assertion_string) = assertion_string;
            let assertion_data = AssertionData {
                status: self.status,
                duration: self.response_time,
                body: self.body.to_owned().unwrap_or(Value::Null),
                headers: self.headers.to_owned(),
            };

            let assertion = Assertion::from_str(assertion_string, workflow_data);
            let result = assertion.assert(&assertion_data);

            assertions_results.push(result);
        }

        self.assertion_results = assertions_results;
        self.assertion_results.to_owned()
    }

    /// Return a masked version of the response.
    ///
    /// It's using the `mask` property of options, and
    /// anything with a key defined in that array will have its
    /// value masked.
    pub fn into_masked(&self, options: &Option<WorkflowConfigStepOptions>) -> Option<ResponseData> {
        let response = match options {
            Some(options) => self.mask(options),
            _ => self.to_owned(),
        };

        Some(response)
    }

    /// Masks the response headers and body using a deep replace.
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::mock;

    #[test]
    fn test_successful_response() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_body(r#"{"hello": "world"}"#)
            .with_header("test-header", "test-value")
            .create();

        let result = ureq::request("GET", &format!("{}/test", url)).call();
        let response = ResponseData::from_result(result, 0);

        assert_eq!(response.status, Some(200));
        assert_eq!(response.body, Some(json!({"hello": "world"})));
    }

    #[test]
    fn test_bad_response() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/500").with_status(500).create();

        let result = ureq::request("GET", &format!("{}/500", url)).call();
        let response = ResponseData::from_result(result, 0);

        assert_eq!(response.status_text, "Internal Server Error".to_string());
        assert_eq!(response.status, Some(500));
    }

    #[test]
    fn test_no_response() {
        let result = ureq::request("GET", "http://bad-url").call();
        let response = ResponseData::from_result(result, 0);

        assert_eq!(response.status, None);
        assert_eq!(response.status_text, "Could not connect to URL");
    }

    #[test]
    fn test_mask_response() {
        let response = ResponseData {
            body: Some(json!({
                "secret": "abc"
            })),
            headers: json!({
                "secret": 123,
            }),
            ..Default::default()
        };
        let masked = response
            .into_masked(&Some(WorkflowConfigStepOptions {
                mask: Some(vec!["secret".to_string()]),
            }))
            .unwrap();

        let body = masked.body.unwrap();
        assert_eq!(body["secret"], "****");
        assert_eq!(masked.headers["secret"], "****");
    }

    #[test]
    fn test_assert() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_body(r#"{"hello": "world"}"#)
            .with_header("test-header", "test-value")
            .create();

        let result = ureq::request("GET", &format!("{}/test", url)).call();
        let mut response = ResponseData::from_result(result, 1000);

        let assertions = vec![
            WorkflowConfigAssertion::assert("status equal 200".to_string()),
            WorkflowConfigAssertion::assert("body.hello equal world".to_string()),
            WorkflowConfigAssertion::assert("headers.test-header equal test-value".to_string()),
            WorkflowConfigAssertion::assert("duration equal 500".to_string()),
        ];

        let assertion_results = response.assert(&assertions, &json!({}));
        assert_eq!(assertion_results.len(), 4);
        assert_eq!(assertion_results[0].passed, true);
        assert_eq!(assertion_results[1].passed, true);
        assert_eq!(assertion_results[2].passed, true);
        assert_eq!(assertion_results[3].passed, false);
        assert_eq!(
            assertion_results[3].message,
            Some("expected 1000 to equal 500".to_string())
        );
    }
}
