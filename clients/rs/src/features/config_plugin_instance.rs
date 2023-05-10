use std::{error::Error, sync::{Mutex, Arc}};
use polywrap_client::{core::{client::Client, invoker::Invoker}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient, plugin::{implementor::plugin_impl, module::PluginModule, wrapper::PluginWrapper}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_object, expect_uri, expect_string};

#[derive(Deserialize)]
struct InputObj {
  uri: Value,
  method: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let uri = expect_uri(&input_obj.uri)?;
  let method = expect_string(&input_obj.method)?;

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
      self.counter = self.counter + 1;
    }
  }

  impl PluginModule for Plugin {
      fn _wrap_invoke(
          &mut self,
          method_name: &str,
          params: &[u8],
          env: Option<&polywrap_client::core::env::Env>,
          invoker: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
      ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
          match method_name {
            "increment" => {
              self.increment();
              Ok(vec![])
            },
            _ => panic!("Unrecognized method: {method_name}")
          }
      }
  }

  let plugin = Plugin::new();
  
  println!("Adding Plugin Instance to ClientConfig");

  let plugin_wrapper = PluginWrapper::new(Arc::new(Mutex::new(Box::new(plugin))));

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_wrapper(uri.clone(), Arc::new(plugin_wrapper));
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  for _ in 0..2 {
    println!("Invoking Plugin Instance");

    let result = client.invoke_raw(
      &uri,
      &method,
      None,
      None,
      None
    );

    // TODO: no ergonomic way of retrieving plugin state
    let plugin_counter = todo!();

    // if result.is_ok() {
    //   println!("counter = {plugin_counter}")
    // }
  }

  println!("Success!");

  Ok(())
}