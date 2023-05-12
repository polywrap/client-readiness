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
mod config_resolver_ext;
mod config_uri_redirect;
mod invoke_plugin;
mod invoke_wrap_wasm_v0_1;
mod resolve_instance;
mod resolve_ipfs;
mod resolve_ens_contenthash;
mod resolve_ens_text_record;
mod resolve_file;
mod resolve_http;
mod resolve_package;
mod resolve_redirect;
mod subinvoke_plugin_wrap;
mod subinvoke_wrap_plugin;
mod wrap_feature_env_vars;
mod wrap_feature_interface_invoke;
mod wrap_type_bigint;
mod wrap_type_bytes;

type RunTestCaseFn = fn(&Value) -> Result<(), Box<dyn Error>>;
type Features = HashMap<String, RunTestCaseFn>;

pub fn get() -> Features {
  let mut features: Features = HashMap::new();

  features.insert("uri".to_string(), uri::run_test_case);

  features
}
