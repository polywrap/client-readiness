use std::collections::HashMap;
use std::error::Error;
use serde_json::Value;

mod uri;
mod config_embed_wrap_package;
mod config_env_variables;
mod config_interface_implementations;
mod config_plugin_instance;
mod config_plugin_package;
mod config_resolver;

type RunTestCaseFn = fn(&Value) -> Result<(), Box<dyn Error>>;
type Features = HashMap<String, RunTestCaseFn>;

pub fn get() -> Features {
  let mut features: Features = HashMap::new();

  features.insert("uri".to_string(), uri::run_test_case);

  features
}
