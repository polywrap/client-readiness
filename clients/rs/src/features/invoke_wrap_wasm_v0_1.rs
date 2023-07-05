use std::{error::Error, sync::{Arc}, fs, path::Path};
use polywrap_client::{core::{invoker::Invoker, file_reader::SimpleFileReader}, client::PolywrapClient, wasm::wasm_package::WasmPackage, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_root_dir};

#[derive(Deserialize)]
struct InputObj {
  directory: Value,
  method: String,
  args: String
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let wrap_dir = expect_root_dir(
    &input_obj.directory,
    std::env::current_dir()?.join("../../").to_str().unwrap()
  )?;
  let method = input_obj.method;
  let args = input_obj.args;

  let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  let wrap_package = WasmPackage::from_bytecode(
    wasm_module,
    Arc::new(SimpleFileReader::new()),
    Some(manifest)
  );

  let mut config = PolywrapClientConfig::new();
  config.add_package("embed/foo".try_into().unwrap(), Arc::new(wrap_package));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Invoking {method}");

  let result = client.invoke_raw(
    &"embed/foo".try_into().unwrap(),
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
