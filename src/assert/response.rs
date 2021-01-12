use crate::compile::compile_string;
use crate::{
    assert::{assert, parse_assertion_string, Assertion},
    workflow::{ResponseData, WorkflowConfigAssertion},
};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, Clone)]
pub struct AssertionResultData {
    pub passed: bool,
    pub message: Option<String>,
    pub assertion: Assertion,
}

#[derive(Debug, Serialize)]
pub struct AssertionData {
    pub status: Option<u16>,
    pub body: serde_json::Value,
    pub headers: serde_json::Value,
    pub duration: i64,
}

pub async fn assert_on_response(
    response: &ResponseData,
    assertions: &Vec<WorkflowConfigAssertion>,
    workflow_data: &Value,
) -> Vec<AssertionResultData> {
    let mut assertions_results: Vec<AssertionResultData> = vec![];

    for assertion_string in assertions {
        let WorkflowConfigAssertion::assert(assertion_string) = assertion_string;

        let assertion_data = AssertionData {
            status: response.status,
            duration: response.response_time,
            body: response.body.clone().unwrap_or(Value::Null),
            headers: response.headers.clone().into(),
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

    assertions_results
}
