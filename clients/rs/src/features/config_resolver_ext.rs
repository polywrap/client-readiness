use std::{error::Error};
use polywrap_client::{core::{client::Client, uri::Uri}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient, plugin::module::PluginModule, wrap_manifest::versions::{WrapManifest01, WrapManifest01Abi}};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::input::{expect_object, expect_uri, expect_string};

#[derive(Deserialize)]
struct InputObj {
  authority: Value,
  result: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let authority = expect_string(&input_obj.authority)?;
  let result = expect_string(&input_obj.result)?;

  println!("Creating CustomResolverExt Plugin");

  #[derive(Debug)]
  struct CustomResolverExt {
    authority: String,
    result: String
  }

  #[derive(Deserialize)]
  struct TryResolveUriArgs {
    authority: String
  }

  #[derive(Deserialize, Serialize)]
  struct TryResolveUriResult {
    uri: Uri,
    manifest: WrapManifest01
  }

  impl PluginModule for CustomResolverExt {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        env: Option<&polywrap_client::core::env::Env>,
        invoker: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
    ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
        match method_name {
          "tryResolveUri" => {
            let args: TryResolveUriArgs = polywrap_client::msgpack::decode(params);

            if args.authority == self.authority {
              let result = TryResolveUriResult {
                manifest: WrapManifest01 {
                  abi: WrapManifest01Abi {
                      enum_types: None,
                      env_type: None,
                      imported_enum_types: None,
                      imported_env_types: None,
                      imported_module_types: None,
                      imported_object_types: None,
                      interface_types: None,
                      module_type: None,
                      object_types: None,
                      version: None,
                  },
                  name: String::from(""),
                  type_: String::from(""),
                  version: String::from(""),
                },
                uri: self
              };
            }
          }
        }
    }
}

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_env(uri.clone(), input_obj.env);
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Fetching Env");

  let result = client.get_env_by_uri(&uri);

  if let Some(result) = result {
    for key in result.as_object().unwrap().keys() {
      let value = result.get(key).unwrap();
      println!("env.{key} = {value}");
    }
    println!("Success!")
  }

  Ok(())
}