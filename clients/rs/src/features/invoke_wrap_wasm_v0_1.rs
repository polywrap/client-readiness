use std::{error::Error, sync::{Arc}, fs, path::Path};
use polywrap_client::{core::{invoker::Invoker, file_reader::SimpleFileReader}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient, wasm::wasm_package::WasmPackage};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_object, expect_string, expect_root_dir};

#[derive(Deserialize)]
struct InputObj {
  directory: Value,
  method: Value,
  args: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let wrap_dir = expect_root_dir(
    &input_obj.directory,
    std::env::current_dir()?.join("../../../../").to_str().unwrap()
  )?;
  let method = expect_string(&input_obj.method)?;
  let args = expect_string(&input_obj.args)?;

  let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  let wrap_package = WasmPackage::new(
    Arc::new(SimpleFileReader::new()),
    Some(manifest),
    Some(wasm_module)
  );

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_package("embed/foo".try_into()?, Arc::new(wrap_package));
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Invoking {method}");

  let result = client.invoke_raw(
    &"embed/foo".try_into()?,
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
