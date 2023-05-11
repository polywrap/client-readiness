use std::{error::Error, sync::{Mutex, Arc}};
use polywrap_client::{core::{invoker::Invoker}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient, wrap_manifest::versions::{WrapManifest01Abi, WrapManifest01}, plugin::{package::PluginPackage, module::PluginModule}};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::input::{expect_object, expect_string};

#[derive(Deserialize)]
struct InputObj {
  uri: Value,
  method: Value,
  args: Value
}

#[derive(Serialize, Deserialize)]
struct AddArgs { 
  a: u8,
  b: u8
}

#[derive(Debug)]
struct Plugin {}

impl Plugin {
  pub fn add(&self, args: &AddArgs) -> u8 {
    args.a + args.b
  }
}

impl PluginModule for Plugin {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        _: Option<&polywrap_client::core::env::Env>,
        _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
    ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
        match method_name {
          "add" => {
            let args: AddArgs = polywrap_client::msgpack::decode(params)?;
            let result = self.add(&args);
            Ok(polywrap_client::msgpack::serialize(&result)?)
          },
          _ => panic!("Unrecognized method: {method_name}")
        }
    }
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let uri = expect_string(&input_obj.uri)?;
  let method = expect_string(&input_obj.method)?;
  let args = expect_string(&input_obj.args)?;

  let plugin_package = PluginPackage::new(
  Arc::new(Mutex::new(Box::new(Plugin {}))),
  WrapManifest01 {
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
  });

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_package(uri.try_into()?, Arc::new(plugin_package));
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Invoking {method}");

  let result = client.invoke_raw(
    &uri.try_into()?,
    &method,
    Some(&polywrap_client::msgpack::serialize(&args)?),
    None,
    None
  );

  if let Ok(result) = result {
    println!("Received: {result:?}");
    println!("Success!");
  }

  Ok(())
}