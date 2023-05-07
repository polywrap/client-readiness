use std::collections::HashMap;
use std::error::Error;
use serde_json::Value;

mod uri;

type RunTestCaseFn = fn(&Value) -> Result<(), Box<dyn Error>>;
type Features = HashMap<String, RunTestCaseFn>;

pub fn get() -> Features {
  let mut features: Features = HashMap::new();

  features.insert("uri".to_string(), uri::run_test_case);

  features
}
