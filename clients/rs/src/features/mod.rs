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
mod wrap_type_bignumber;
mod wrap_type_bytes;
mod wrap_type_enum;
mod wrap_type_ints;
mod wrap_type_json;
mod wrap_type_map;
mod wrap_type_object;

type RunTestCaseFn = fn(&Value) -> Result<(), Box<dyn Error>>;
type Features = HashMap<String, RunTestCaseFn>;

pub fn get() -> Features {
  let mut features: Features = HashMap::new();

  features.insert("config_embed_wrap_package".to_string(), config_embed_wrap_package::run_test_case);
  features.insert("config_env_variables".to_string(), config_env_variables::run_test_case);
  features.insert("config_interface_implementations".to_string(), config_interface_implementations::run_test_case);
  features.insert("config_plugin_instance".to_string(), config_plugin_instance::run_test_case);
  features.insert("config_plugin_package".to_string(), config_plugin_package::run_test_case);
  features.insert("config_resolver_ext".to_string(), config_resolver_ext::run_test_case);
  features.insert("config_resolver".to_string(), config_resolver::run_test_case);
  features.insert("config_uri_redirect".to_string(), config_uri_redirect::run_test_case);
  features.insert("invoke_plugin".to_string(), invoke_plugin::run_test_case);
  features.insert("invoke_wrap_wasm_v0_1".to_string(), invoke_wrap_wasm_v0_1::run_test_case);
  features.insert("resolve_ens_contenthash".to_string(), resolve_ens_contenthash::run_test_case);
  features.insert("resolve_ens_text_record".to_string(), resolve_ens_text_record::run_test_case);
  features.insert("resolve_file".to_string(), resolve_file::run_test_case);
  features.insert("resolve_http".to_string(), resolve_http::run_test_case);
  features.insert("resolve_instance".to_string(), resolve_instance::run_test_case);
  features.insert("resolve_ipfs".to_string(), resolve_ipfs::run_test_case);
  features.insert("resolve_package".to_string(), resolve_package::run_test_case);
  features.insert("resolve_redirect".to_string(), resolve_redirect::run_test_case);
  features.insert("subinvoke_plugin_wrap".to_string(), subinvoke_plugin_wrap::run_test_case);
  features.insert("uri".to_string(), uri::run_test_case);
  features.insert("wrap_feature_env_vars".to_string(), wrap_feature_env_vars::run_test_case);
  features.insert("wrap_feature_interface_invoke".to_string(), wrap_feature_interface_invoke::run_test_case);
  features.insert("wrap_type_bigint".to_string(), wrap_type_bigint::run_test_case);
  features.insert("wrap_type_bignumber".to_string(), wrap_type_bignumber::run_test_case);
  features.insert("wrap_type_bytes".to_string(), wrap_type_bytes::run_test_case);
  features.insert("wrap_type_enum".to_string(), wrap_type_enum::run_test_case);
  features.insert("wrap_type_ints".to_string(), wrap_type_ints::run_test_case);
  features.insert("wrap_type_json".to_string(), wrap_type_json::run_test_case);
  features.insert("wrap_type_map".to_string(), wrap_type_map::run_test_case);
  features.insert("wrap_type_object".to_string(), wrap_type_object::run_test_case);

  features
}
