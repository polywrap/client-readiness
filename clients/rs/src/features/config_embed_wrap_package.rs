use std::{error::Error, path::Path, fs::{self}, sync::Arc};
use polywrap_client::{wasm::wasm_package::WasmPackage, core::{file_reader::SimpleFileReader, invoker::Invoker}, client::PolywrapClient, builder::{PolywrapClientConfigBuilder, PolywrapClientConfig}};
use serde_json::Value;

use crate::input::{expect_root_dir, expect_string};

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: serde_json::Map<String, Value> = serde_json::from_value(input.clone())?;

  let wrap_dir = expect_root_dir(
      input_obj.get("directory").ok_or("Expected directory field")?,
      std::env::current_dir()?.join("../../").to_str().unwrap(),
  )?;

  let method = expect_string(input_obj.get("method").ok_or("Expected method field")?)?;
  let args: serde_json::Map<String, Value> = serde_json::from_value(
      input_obj.get("args").ok_or("Expected args field")?.clone(),
  )?;

  println!("Reading wrap.info & wrap.wasm from {wrap_dir}");

  let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  println!("Creating WrapPackage from raw wrap.info & wrap.wasm bytes");

  let wrap_package = WasmPackage::from_bytecode(wasm_module, Arc::new(SimpleFileReader::new()), Some(manifest));

  println!("Adding WrapPackage to ClientConfig");

  let mut config = PolywrapClientConfig::new();
  config.add_package("embed/foo".try_into().unwrap(), Arc::new(wrap_package));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Invoking WrapPackage");

  let result = client.invoke_raw(
    &"embed/foo".try_into().unwrap(),
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
