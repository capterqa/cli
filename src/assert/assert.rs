use crate::utils::exit_with_code;
use crate::{
    assert::{ValueAssertions, ASSERTION_TYPES},
    compile::{compile_string, CompiledString},
    workflow::WorkflowConfigAssertion,
};
use serde::Serialize;
use serde_json::{json, Value};

/// Assertion turns an assertion string from the yaml workflows
/// into a real assertion that can be used on a payload.
///
/// `- !assert status equal {{ env.STATUS }}` will be parsed and
/// can then be used to assert "data" by calling `.assert(data)`.
pub struct Assertion {
    assertion_string: CompiledString,
    /// inverts the test if true
    /// `expected a to NOT be b`
    not: bool,
}

/// The result of an assertion. Can be serialized to JSON
/// and sent to a webhook.
#[derive(Serialize, Clone, Debug)]
pub struct AssertionResultData {
    pub passed: bool,
    pub message: Option<String>,
    pub assertion: AssertionTest,
}

/// The source data used to assert against.
#[derive(Debug, Serialize)]
pub struct AssertionData {
    pub status: Option<u16>,
    pub body: serde_json::Value,
    pub headers: serde_json::Value,
    pub duration: i64,
}

/// The result from a parsed assertion string. This is
/// used by the assert methods to assert.
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct AssertionTest {
    /// `to_equal`, `to_be_above` etc.
    /// See `ASSERTION_TYPES`.
    pub test: String,
    /// The key we want to assert on.
    /// Can be chained like `my.property.0.path`.
    pub property: String,
    /// The value we want the property to match.
    pub value: serde_json::Value,
    /// true if the test is inverted
    pub not: bool,
}

impl Assertion {
    /// Create a new Assertion from a string.
    /// The format of the string is `status equal 200`,
    pub fn from_assertion(
        assertion_string: &WorkflowConfigAssertion,
        workflow_data: &Value,
    ) -> Assertion {
        let (not, assertion_string) = match assertion_string {
            WorkflowConfigAssertion::assert(val) => (false, compile_string(val, workflow_data)),
            WorkflowConfigAssertion::assert_not(val) => (true, compile_string(val, workflow_data)),
        };

        Assertion {
            not,
            assertion_string,
        }
    }

    /// Assert on the data passed in. Returns
    /// a masked `AssertionResultData` that can be displayed
    /// to the user or sent to the webhook.
    pub fn assert(&self, assertion_data: &AssertionData) -> AssertionResultData {
        let assertion_data_json = json!(&assertion_data);
        let assertion = parse_assertion_string(&self.assertion_string.raw, self.not);

        // create a path to the property and get the data
        let path = format!("/{}", assertion.property.replace(".", "/"));
        let data = assertion_data_json.pointer(&path).unwrap_or(&Value::Null);

        let assert_fn = ValueAssertions::get(&assertion.test);
        let result = assert_fn(data, &assertion.value, self.not);
        let passed = result.is_none();

        let is_masked = self.assertion_string.raw != self.assertion_string.masked;

        AssertionResultData {
            assertion: self.get_masked_assertion_test(),
            message: match is_masked {
                true => Some("Hidden because of mask".to_string()),
                false => result,
            },
            passed,
        }
    }

    /// Create a masked version of the `AssertionTest`.
    pub fn get_masked_assertion_test(&self) -> AssertionTest {
        parse_assertion_string(&self.assertion_string.masked, self.not)
    }
}

/// Parse an assertion string.
///
/// It splits the string up and tries to figure out
/// the different parts of an `AssertionTest`. Will exit
/// if it can't parse the input.
pub fn parse_assertion_string(assertion_string: &str, not: bool) -> AssertionTest {
    let mut parts = assertion_string.split(' ').collect::<Vec<&str>>();

    // pull the property from the array
    let property = parts[0];
    parts.remove(0);

    // !assert x isArray
    if parts.len() == 1 && ASSERTION_TYPES.contains(&parts[0]) {
        return AssertionTest {
            test: parts[0].to_owned(),
            property: property.to_owned(),
            value: Value::Null,
            not,
        };
    }

    // !assert x data.0.title isNotEmpty
    if parts.len() == 2 && ASSERTION_TYPES.contains(&parts[1]) {
        return AssertionTest {
            test: parts[1].to_owned(),
            property: format!("{}.{}", property, parts[0]),
            value: Value::Null,
            not,
        };
    }

    // !assert x isAbove 5
    if parts.len() > 1 && ASSERTION_TYPES.contains(&parts[0]) {
        let mut value = parts.clone();
        value.remove(0);
        let value = value.join(" ");

        return AssertionTest {
            test: parts[0].to_owned(),
            property: property.to_owned(),
            value: json!(value),
            not,
        };
    };

    // !assert x data.0.id equal 0
    if parts.len() > 2 && ASSERTION_TYPES.contains(&parts[1]) {
        let mut value = parts.clone();
        value.remove(0);
        value.remove(0);
        let value = value.join(" ");

        return AssertionTest {
            test: parts[1].to_owned(),
            property: format!("{}.{}", property, parts[0]),
            value: json!(value),
            not,
        };
    }
    exit_with_code(
        exitcode::CONFIG,
        Some(&format!(
            "Could not parse assertion: `{}`",
            assertion_string
        )),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertion() {
        let data = json!({
            "env": { "name": "Test McTest" }
        });
        let assertion_data = AssertionData {
            body: json!({
                "user": { "name": "Test McTest" },
            }),
            headers: json!({
                "content-type": "application/json",
            }),
            duration: 500,
            status: Some(200),
        };

        let assertion = Assertion::from_assertion(
            &WorkflowConfigAssertion::assert("body.user.name to_equal ${{ env.name }}".to_string()),
            &data,
        );
        let result = assertion.assert(&assertion_data);
        assert_eq!(result.passed, true);

        let assertion = Assertion::from_assertion(
            &WorkflowConfigAssertion::assert_not("body.user.name to_equal bad value".to_string()),
            &data,
        );
        let result = assertion.assert(&assertion_data);
        assert_eq!(result.passed, true);

        let assertion = Assertion::from_assertion(
            &WorkflowConfigAssertion::assert("headers.nope to_be_undefined".to_string()),
            &data,
        );
        let result = assertion.assert(&assertion_data);
        assert_eq!(result.passed, true);

        let assertion = Assertion::from_assertion(
            &WorkflowConfigAssertion::assert_not("body to_be_empty".to_string()),
            &data,
        );
        let result = assertion.assert(&assertion_data);
        assert_eq!(result.passed, true);

        let assertion = Assertion::from_assertion(
            &WorkflowConfigAssertion::assert("body.name to_equal ${{ mask env.name }}".to_string()),
            &data,
        );
        let result = assertion.assert(&assertion_data);
        assert_eq!(result.passed, false);
        assert_eq!(result.message, Some("Hidden because of mask".to_string()));
    }
}
