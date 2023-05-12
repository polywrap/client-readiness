use std::{error::Error};
use polywrap_client::{core::{uri::Uri, invoker::Invoker}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient, msgpack::msgpack};
use serde::{Deserialize};
use serde_json::{Value};

#[derive(Deserialize)]
struct InputObj {
  #[serde(rename = "mainEnv")]
  main_env: Value,
  #[serde(rename = "extEnv")]
  ext_env: Value
}

#[derive(Deserialize)]
struct SubinvokeMethodResult {
  local: Value,
  external: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let root = std::env::current_dir()?.join("../../../../wraps");
  let interface_uri: Uri = "wrap://ens/interface.eth".try_into()?;
  let implementation_path = root.join("/interface-invoke/01-implementation/implementations/as").to_str().unwrap();
  let implementation_uri: Uri = format!("file/{implementation_path}").try_into()?;

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_interface_implementation(interface_uri, implementation_uri);

  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  let wrapper_path = root.join("/interface-invoke/02-wrapper/implementations/as").to_str().unwrap();
  let wrapper_uri: Uri = format!("fs/{wrapper_path}").try_into()?;

  println!("Invoking moduleMethod");

  let result = client.invoke_raw(
    &wrapper_uri,
    "moduleMethod",
    Some(&msgpack!({
      "args": {
        "arg": {
          "uint8": 1,
          "str": "Test String 1"
        }
      }
    })),
    None,
    None
  );

  if let Err(err) = result {
    panic!("{err}")
  }

  println!("Success!");
  
  Ok(())
}
