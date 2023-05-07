use serde_json::Value;

pub struct Input;

impl Input {
    pub fn expect_string(input: &Value) -> Result<String, String> {
        Ok(input.as_str().expect("expected a string").to_string())
    }
}
