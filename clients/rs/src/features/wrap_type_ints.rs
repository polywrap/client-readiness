use std::{error::Error};
use polywrap_client::{core::{uri::Uri, invoker::Invoker}, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_msgpack_serde;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::input::expect_object;

#[derive(Serialize, Deserialize)]
struct Args {
  first: i32,
  second: i32
}

#[derive(Serialize, Deserialize)]
struct InputObj {
  method: String,
  args: Args,
}

pub fn run_test_case(input_obj: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(&input_obj)?;
  let method = input_obj.method;
  let args = input_obj.args;

  let binding = std::env::current_dir()?.join("../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/numbers-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  println!("Invoking {method}");

  // TODO: int size?
  let result = client.invoke_raw(
    &uri,
    &method,
    Some(&polywrap_msgpack_serde::to_vec(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      let result = polywrap_msgpack_serde::from_slice::<i32>(&result)?;
      println!("Result: {result:?}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
