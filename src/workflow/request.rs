use crate::assert::AssertionResultData;
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use std::time::Instant;
use ureq;

#[derive(Debug, Serialize)]
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

#[derive(Debug, Serialize, Clone)]
pub struct ResponseData {
    pub created_at: DateTime<Utc>,
    pub status: Option<u16>,
    pub status_text: String,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
    pub response_time: i64,
    pub assertion_results: Option<Vec<AssertionResultData>>,
}

pub fn make_request(request_data: &RequestData) -> ResponseData {
    let mut request = ureq::request(&request_data.method, &request_data.url);

    // add query
    if let Some(query) = &request_data.query {
        if let Some(mapping) = query.as_mapping() {
            for (key, value) in mapping {
                let key = key.as_str().unwrap_or("");
                let value = json!(value).to_string();
                request = request.query(key, &value);
            }
        }
    }

    // add headers
    if let Some(headers) = &request_data.headers {
        if let Some(map) = headers.as_mapping() {
            for (key, value) in map {
                let key = key.as_str().unwrap();
                let value = value.as_str().unwrap_or("");
                request = request.set(key, value);
            }
        }
    }

    // track time
    let timer = Instant::now();

    let response = match &request_data.body {
        Some(body) => request.send_json(json!(body)),
        _ => request.call(),
    };

    // save time
    let response_time = timer.elapsed().as_millis() as i64;

    // loop through assertions and assert against the response
    let response_data = match response {
        Ok(response) => {
            let status = response.status();

            let headers_names = response.headers_names();

            // move headers to a value
            let mut headers = serde_json::Map::new();
            for name in headers_names {
                let value = response.header(&name);
                headers.insert(name, json!(value));
            }

            // let status_text = response.status_text();
            let status_text = "test";
            // println!("{}", &response.get_url());
            let body: Value = response.into_json().unwrap_or(json!(status_text));

            ResponseData {
                created_at: Utc::now(),
                response_time,
                status: Some(status),
                status_text: status_text.to_string(),
                body: Some(body),
                headers: headers.into(),
                assertion_results: None,
            }
        }
        Err(ureq::Error::Status(code, response)) => ResponseData {
            created_at: Utc::now(),
            response_time,
            status: Some(code),
            status_text: response.status_text().to_owned(),
            body: None,
            headers: Value::Null,
            assertion_results: None,
        },
        Err(error) => ResponseData {
            created_at: Utc::now(),
            response_time,
            status: Some(0),
            status_text: error.to_string(),
            body: None,
            headers: Value::Null,
            assertion_results: None,
        },
    };

    response_data
}
