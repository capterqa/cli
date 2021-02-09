use serde_json::json;
use serde_yaml::Value;
use std::time::Instant;

/// Utility to make http requests.
/// Wrapper on top of ureq.
pub struct HttpRequest {
    request: ureq::Request,
    body: Option<Value>,
    timer: Instant,
}

impl HttpRequest {
    /// Create a new HttpRequest. You can add headers, query and body to it
    /// before calling `.call()`.
    pub fn new(url: String, method: String) -> HttpRequest {
        let request = ureq::request(&method, &url);

        HttpRequest {
            request,
            body: None,
            timer: Instant::now(),
        }
    }

    /// Add a query to the request.
    /// We're taking a Value as the argument, but it should
    /// be validated by `clap` when reading the yaml file.
    pub fn add_query(&mut self, query: &Value) {
        if let Some(mapping) = query.as_mapping() {
            for (key, value) in mapping {
                let (key, value) = parse_key_value(key, value);
                self.request = self.request.to_owned().query(&key, &value);
            }
        }
    }

    /// Add headers to the request.
    /// We're taking a Value as the argument, but it should
    /// be validated by `clap` when reading the yaml file.
    pub fn add_headers(&mut self, headers: &Value) {
        // add headers
        if let Some(mapping) = headers.as_mapping() {
            for (key, value) in mapping {
                let (key, value) = parse_key_value(key, value);
                self.request = self.request.to_owned().set(&key, &value);
            }
        }
    }

    /// Add a body to the request.
    pub fn add_body(&mut self, body: &Value) {
        self.body = Some(body.to_owned());
    }

    /// Make the request and return a response or an error.
    pub fn call(&mut self) -> Result<ureq::Response, ureq::Error> {
        // reset timer
        self.timer = Instant::now();

        // make the request
        match &self.body {
            // call with body
            Some(body) => self.request.to_owned().send_json(json!(body)),
            // or without body
            _ => self.request.to_owned().call(),
        }
    }

    /// Time elapsed since `.call()` started.
    pub fn get_elapsed(&self) -> i64 {
        self.timer.elapsed().as_millis() as i64
    }
}

/// Utility to parse key value pairs for headers and query.
fn parse_key_value(key: &serde_yaml::Value, value: &serde_yaml::Value) -> (String, String) {
    let value = match &value {
        &serde_yaml::Value::String(val) => val.to_string(),
        val => json!(val).to_string(),
    };
    (key.as_str().unwrap().to_owned(), value.to_owned())
}
