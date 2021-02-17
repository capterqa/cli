use serde_json::json;
use serde_yaml::Value;
use std::{
    env,
    time::{Duration, Instant},
};

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
    pub fn new(url: String, method: String, timeout: u64) -> HttpRequest {
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(timeout))
            .build();
        let request = agent.request(&method, &url).set(
            "User-Agent",
            &format!("capter/{}", env!("CARGO_PKG_VERSION")),
        );

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
        let request = self.request.to_owned();

        // make the request
        match &self.body {
            // call with body
            Some(body) => match body {
                Value::String(str) => request.send_string(str),
                _ => request.send_json(json!(body)),
            },
            // or without body
            _ => request.call(),
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

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use mockito::mock;
    use serde_json::json;
    use serde_yaml::from_str;

    #[test]
    fn test_basic_call() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .with_status(200)
            .with_body(r#"{"hello": "world"}"#)
            .create();

        let mut request = HttpRequest::new(format!("{}/test", url), "GET".into(), 30);
        let response = request.call();

        match response {
            Ok(response) => {
                let value: serde_json::Value = response.into_json().unwrap();
                assert_eq!(value, json!({ "hello": "world"}));
            }
            _ => {
                assert!(false);
            }
        }
    }

    #[test]
    fn test_query_header_body() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .match_body("{\"hello\":\"world\"}")
            .match_header("test-header", "test-value")
            .match_query("foo=bar")
            .with_status(200)
            .create();

        let mut request = HttpRequest::new(format!("{}/test", url), "GET".into(), 30);

        let yaml = indoc! {"
            ---
            body:
              hello: world
            headers:
              test-header: test-value
            query:
              foo: bar
        "};
        let config: Value = from_str(yaml).unwrap();

        request.add_body(&config["body"]);
        request.add_headers(&config["headers"]);
        request.add_query(&config["query"]);

        let response = request.call();

        match response {
            Ok(response) => {
                assert_eq!(response.status(), 200);
            }
            Err(err) => {
                println!("{}", err);
                assert!(false);
            }
        }
    }

    #[test]
    fn test_query_string_body() {
        let url = &mockito::server_url();
        let _m = mock("GET", "/test")
            .match_body("hello world")
            .with_status(200)
            .create();

        let mut request = HttpRequest::new(format!("{}/test", url), "GET".into(), 30);

        let yaml = indoc! {"
            ---
            body: hello world
        "};
        let config: Value = from_str(yaml).unwrap();

        println!("{:#?}", config["body"]);

        request.add_body(&config["body"]);

        let response = request.call();

        match response {
            Ok(response) => {
                assert_eq!(response.status(), 200);
            }
            Err(err) => {
                println!("{}", err);
                assert!(false);
            }
        }
    }
}
