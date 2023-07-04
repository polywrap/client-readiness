use std::{error::Error, sync::{Arc, Mutex}};
use polywrap_client::{core::{invoker::Invoker}, client::PolywrapClient, plugin::{module::PluginModule, package::PluginPackage}, wrap_manifest::versions::{WrapManifest01, WrapManifest01Abi}, builder::{PolywrapClientConfigBuilder, PolywrapClientConfig}};
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

use crate::input::{expect_object, expect_uri, expect_string};

#[derive(Deserialize)]
struct InputObj {
  uri: Value,
  method: Value,
  args: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let uri = expect_uri(&input_obj.uri)?;
  let method = expect_string(&input_obj.method)?;
  let args = expect_object::<Map<String, Value>>(&input_obj.args)?;

  println!("Creating PluginPackage");
  
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
    }
  );

  println!("Adding PluginPackage to ClientConfig");

  let mut config = PolywrapClientConfig::new();
  config.add_package(uri.clone(), Arc::new(plugin_package));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Invoking PluginPackage");

  let result = client.invoke_raw(
    &uri,
    &method,
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  if result.is_ok() {
    println!("Success!");
  }

  Ok(())
}
