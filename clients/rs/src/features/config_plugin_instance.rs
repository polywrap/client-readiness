use std::{error::Error, sync::{Mutex, Arc}};
use polywrap_client::{core::{invoker::Invoker, uri::Uri}, client::PolywrapClient, plugin::{module::PluginModule, wrapper::PluginWrapper}, builder::{PolywrapClientConfigBuilder, PolywrapClientConfig}};
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize)]
struct InputObj {
  uri: String,
  method: String
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let uri: Uri = input_obj.uri.try_into()?;
  let method = input_obj.method;

  println!("Creating Plugin Instance");

  #[derive(Debug)]
  struct Plugin {
    pub counter: u8
  }

  impl Plugin {
    pub fn new() -> Self {
      Self { counter: 0 }
    }

    pub fn increment(&mut self) {
      self.counter += 1;
    }
  }

  impl PluginModule for Plugin {
      fn _wrap_invoke(
          &mut self,
          method_name: &str,
          _: &[u8],
          _: Option<&[u8]>,
          _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
      ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
          match method_name {
            "increment" => {
              self.increment();
              let msgpack_null = vec![192];
              Ok(msgpack_null)
            },
            _ => panic!("Unrecognized method: {method_name}")
          }
      }
  }

  let plugin = Plugin::new();
  
  println!("Adding Plugin Instance to ClientConfig");

  let plugin_module = Arc::new(Mutex::new(plugin));
  let plugin_wrapper = PluginWrapper::new(plugin_module.clone());

  let mut config = PolywrapClientConfig::new();
  config.add_wrapper(uri.clone(), Arc::new(plugin_wrapper));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  for _ in 0..2 {
    println!("Invoking Plugin Instance");

    let result = client.invoke_raw(
      &uri,
      &method,
      None,
      None,
      None
    );

    if result.is_ok() {
      let plugin_counter = plugin_module.lock().unwrap().counter;
      println!("counter = {plugin_counter}")
    }
  }

  println!("Success!");

  Ok(())
}