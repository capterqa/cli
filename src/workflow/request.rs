use crate::assert::AssertionResultData;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde::Serialize;
use serde_json::{json, Value};
use std::time::Instant;

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
    pub status: u16,
    pub status_text: Option<&'static str>,
    pub headers: serde_json::Value,
    pub body: Option<serde_json::Value>,
    pub response_time: i64,
    pub assertion_results: Option<Vec<AssertionResultData>>,
}

pub async fn make_request(
    request_data: &RequestData,
) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut request = client.request(
        Method::from_bytes(request_data.method.as_bytes()).unwrap_or(Method::GET),
        &request_data.url,
    );

    // add query
    if let Some(query) = &request_data.query {
        request = request.query(query);
    }

    // add body
    if let Some(body) = &request_data.body {
        let body = json!(body);
        request = request.body(body.to_string());
    }

    // add headers
    if let Some(headers) = &request_data.headers {
        if let Some(map) = headers.as_mapping() {
            for (key, value) in map {
                let key = key.as_str().unwrap();
                let value = value.as_str().unwrap_or("");
                request = request.header(key, value);
            }
        }
    }

    let request = request.build().unwrap();

    // track time
    let timer = Instant::now();

    let response = client.execute(request).await?;

    // save time
    let response_time = timer.elapsed().as_millis() as i64;

    // loop through assertions and assert against the response
    let status = response.status().as_u16();
    let status_text = response.status().canonical_reason().unwrap_or("");
    let headers = response.headers().clone();
    let body = response.json::<Value>().await.unwrap_or(json!(status_text));

    // move headers to a value
    let mut headers_json = serde_json::Map::new();
    for (key, value) in headers.iter() {
        headers_json.insert(key.to_string(), json!(value.to_str().unwrap()));
    }

    let response_data = ResponseData {
        created_at: Utc::now(),
        response_time,
        status,
        status_text: Some(status_text),
        headers: headers_json.into(),
        body: Some(body),
        assertion_results: None,
    };

    Ok(response_data)
}
