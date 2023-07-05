use std::{error::Error};
use polywrap_client::{core::{invoker::Invoker}, client::PolywrapClient, builder::{PolywrapClientConfigBuilder, PolywrapClientConfig}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_uri, expect_array};

#[derive(Deserialize)]
struct InputObj {
  #[serde(rename = "interfaceUri")]
  interface_uri: Value,
  implementations: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let interface_uri = expect_uri(&input_obj.interface_uri)?;
  let implementations = expect_array::<String>(&input_obj.implementations)?;
  let implementations = implementations.into_iter().map(|u| u.try_into().unwrap()).collect();

  println!("Adding Interface Implementations to ClientConfig");

  let mut config = PolywrapClientConfig::new();
  config.add_interface_implementations(interface_uri.clone(), implementations);
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Getting Implementations");

  let result = client.get_implementations(&interface_uri)?;
  let result_len = result.len();

  if result_len > 0 {
    println!("Found {result_len} Implementations");
    println!("Success!");
  }

  Ok(())
}
