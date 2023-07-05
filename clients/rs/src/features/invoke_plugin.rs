use std::{error::Error, sync::{Mutex, Arc}};
use polywrap_client::{core::{invoker::Invoker}, client::PolywrapClient, wrap_manifest::versions::{WrapManifest01Abi, WrapManifest01}, plugin::{package::PluginPackage, module::PluginModule}, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::input::{expect_string};

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
        _: Option<&[u8]>,
        _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
    ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
        match method_name {
          "add" => {
            let args: AddArgs = polywrap_client::msgpack::from_slice(params)?;
            let result = self.add(&args);
            Ok(polywrap_client::msgpack::to_vec(&result)?)
          },
          _ => panic!("Unrecognized method: {method_name}")
        }
    }
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let uri = expect_string(&input_obj.uri)?;
  let method = expect_string(&input_obj.method)?;
  let args = expect_string(&input_obj.args)?;

  let plugin_package = PluginPackage::new(
  Arc::new(Mutex::new(Plugin {})),
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

  let mut config = PolywrapClientConfig::new();
  config.add_package(uri.clone().try_into().unwrap(), Arc::new(plugin_package));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Invoking {method}");

  let result = client.invoke_raw(
    &uri.try_into().unwrap(),
    &method,
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  if let Ok(result) = result {
    println!("Received: {result:?}");
    println!("Success!");
  }

  Ok(())
}
