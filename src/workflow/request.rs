use super::{response::ResponseData, WorkflowConfigAssertion};
use crate::{
    assert::AssertionResultData,
    compile::CompiledString,
    compile::{compile_string, compile_value, CompiledValue},
    utils::{exit_with_code, HttpRequest},
    workflow::{WorkflowConfig, WorkflowConfigStep},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

pub const HTTP_METHODS: &[&str] = &[
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

/// Used create a request from a WorkflowConfigStep,
/// call it, and assert on the response.
///
/// It is serializable to JSON, and is passed to the webhook
/// after the run is completed.
///
/// `response` will be empty until the request is made. We then
/// populate it with the data we get back.
#[derive(Debug, Serialize)]
pub struct Request {
    step: WorkflowConfigStep,
    step_index: i32,
    workflow_data: Value,
    workflow_config: WorkflowConfig,
    pub created_at: DateTime<Utc>,
    url: CompiledString,
    method: String,
    headers: CompiledValue,
    query: CompiledValue,
    body: CompiledValue,
    response: Option<ResponseData>,
}

#[derive(Debug, Serialize, Clone)]
pub struct RequestData {
    pub created_at: DateTime<Utc>,
    pub url: String,
    pub name: String,
    pub method: String,
    pub headers: Option<serde_yaml::Value>,
    pub query: Option<serde_yaml::Value>,
    pub body: Option<serde_yaml::Value>,
    pub order: i32,
    pub is_graphql: bool,
    pub response: Option<ResponseData>,
}

impl Request {
    /// Creates a new request. This will setup it up
    /// and make sure we have all the properties set, like
    /// `url`, `body` etc. You can use `.call()` to make the
    /// request and then assert on it using `.assert_on_response()`.
    pub fn new(
        workflow_config: &WorkflowConfig,
        step_index: i32,
        workflow_data: &Value,
    ) -> Request {
        let step = workflow_config
            .steps
            .get(step_index as usize)
            .expect("Step index out of bounds");

        let (url, mut method) = get_url(step, workflow_data, workflow_config);
        let body = get_body(step, workflow_data);
        let query = get_query(step, workflow_data);
        let headers = get_headers(step, workflow_data);
        // if method is missing in the url string
        // we figure it out here
        if method.is_none() {
            method = Some(get_method(step, workflow_config));
        }

        Request {
            url,
            query,
            body,
            headers,
            step_index,
            method: method.unwrap_or("GET".to_string()),
            created_at: Utc::now(),
            step: step.to_owned(),
            workflow_data: workflow_data.to_owned(),
            workflow_config: workflow_config.to_owned(),
            response: None,
        }
    }

    /// Makes the requests and set the response. This needs to
    /// be called before doing any assertions.
    pub fn call(&mut self) -> Option<ResponseData> {
        let data = self.data();
        let mut request = HttpRequest::new(data.url, data.method);

        // add query
        if let Some(query) = &data.query {
            request.add_query(query);
        }

        // add headers
        if let Some(headers) = &data.headers {
            request.add_headers(headers);
        }

        if let Some(body) = &data.body {
            request.add_body(body);
        }

        let result = request.call();
        let response_time = request.get_elapsed();
        self.response = Some(ResponseData::from_result(result, response_time));

        self.response.to_owned()
    }

    /// Run assertions on the request response.
    /// Make sure to call `.call()` before running this.
    pub fn assert_on_response(
        &mut self,
        assertions: &Vec<WorkflowConfigAssertion>,
    ) -> Vec<AssertionResultData> {
        if let Some(mut response) = self.response.to_owned() {
            let result = response.assert(assertions, &self.workflow_data);
            response.assertion_results = result.clone();
            self.response = Some(response);

            return result.to_owned();
        }

        return vec![];
    }

    /// Return the copy data for this request.
    pub fn data(&self) -> RequestData {
        RequestData {
            created_at: self.created_at,
            url: self.url.raw.to_owned(),
            name: self.step.name.to_owned(),
            query: Some(self.query.raw.to_owned()),
            body: Some(self.body.raw.to_owned()),
            method: self.method.to_owned(),
            headers: Some(self.headers.raw.to_owned()),
            order: self.step_index,
            is_graphql: self.step.graphql.is_some(),
            response: self.response.to_owned(),
        }
    }

    /// Return the copy data for this request,
    /// where sensitive data is masked. This will
    /// return a masked version of response too.
    pub fn data_masked(&self) -> RequestData {
        let data = self.data();
        let response = match data.response {
            Some(response) => response.into_masked(&self.step.options),
            _ => None,
        };

        RequestData {
            url: self.url.masked.to_owned(),
            query: Some(self.query.masked.to_owned()),
            headers: Some(self.headers.masked.to_owned()),
            response,
            ..data
        }
    }
}

/// Get the method for a request.
///
/// It will use the step method if set, and fallback to the workflow if not.
/// If the step has the graphql property set, we use POST.
/// Defaults to GET.
fn get_method(step: &WorkflowConfigStep, workflow_config: &WorkflowConfig) -> String {
    if let Some(method) = &step.method {
        return method.to_owned();
    };

    if let Some(method) = &workflow_config.method {
        return method.to_owned();
    };

    if step.graphql.is_some() {
        return "POST".to_string();
    }

    "GET".to_string()
}

/// Get the headers for a request.
///
/// If graphql is used, we add a json header by default.
fn get_headers(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    let mut headers = serde_yaml::to_value(&step.headers).unwrap();

    if step.graphql.is_some() {
        headers["content-type"] = serde_yaml::Value::String("application/json".to_string());
    }

    compile_value(Some(headers), &workflow_data)
}

/// Get the query for a request.
fn get_query(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    let query = Some(serde_yaml::to_value(&step.query).unwrap());
    compile_value(query, &workflow_data)
}

/// Get the body for a request.
///
/// If the step has the `graphql` property set, this will
/// return a body that works with a graphql request.
fn get_body(step: &WorkflowConfigStep, workflow_data: &Value) -> CompiledValue {
    if let Some(graphql) = &step.graphql {
        let mut graphql_yaml = serde_yaml::Mapping::new();
        graphql_yaml.insert(
            "query".to_string().into(),
            serde_yaml::Value::String(graphql.query.clone()),
        );

        if let Some(variables) = &graphql.variables {
            graphql_yaml.insert("variables".to_string().into(), variables.clone());
        }

        return compile_value(Some(graphql_yaml.into()), &workflow_data);
    }

    compile_value(step.body.clone(), &workflow_data)
}

/// Get the url for a request.
///
/// Will use the step url if it's set,
/// and fallback to the workflow url if it's not.
///
/// The second part of the return is an optional method that can be passed like
/// `POST https://api.com`
///
/// This exit if no url is found, because it's required.
fn get_url(
    step: &WorkflowConfigStep,
    workflow_data: &Value,
    workflow_config: &WorkflowConfig,
) -> (CompiledString, Option<String>) {
    let mut url = step.url.to_owned();
    if let Some(step_url) = &url {
        url = Some(step_url.to_owned());
    };

    if let Some(workflow_url) = &workflow_config.url {
        url = Some(workflow_url.to_owned());
    };

    if let Some(url) = url {
        // remove method if it's set in the URL
        // the method `get_method` will extract and return this
        // so here we just remove it
        let mut parts: Vec<&str> = url.split(" ").collect();

        // if there's just one part, return it
        if parts.len() == 1 {
            return (compile_string(&url, &workflow_data), None);
        }

        // if there's 2 parts more we might have
        // GET https://url.com, so we want to split out the first part
        if parts.len() > 1 {
            // if the first part is a method
            if HTTP_METHODS.contains(&parts[0]) {
                let method = parts[0].to_owned();
                parts.remove(0);
                let url = parts.join(" ");
                return (compile_string(&url, &workflow_data), Some(method));
            }

            // if first part is not a method, just return the whole url
            return (compile_string(&url, &workflow_data), None);
        }
    }

    exit_with_code(
        exitcode::CONFIG,
        Some(&format!("No url found for step: `{}`", step.name)),
    );
}
