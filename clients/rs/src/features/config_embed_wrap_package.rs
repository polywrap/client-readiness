use std::{error::Error, path::Path, fs::{File, self}, io::Read, sync::Arc};
use polywrap_client::{wasm::wasm_package::WasmPackage, core::{file_reader::SimpleFileReader, invoker::Invoker}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient};
use serde_json::Value;

use crate::input::{expect_root_dir, expect_object, expect_string};

fn read_file_to_string(file_path: &str) -> std::io::Result<String> {
  let path = Path::new(file_path);
  let mut file = File::open(path)?;
  let mut content = String::new();
  file.read_to_string(&mut content)?;
  Ok(content)
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<serde_json::Map<String, Value>>(input)?;

  let wrap_dir = expect_root_dir(
      input_obj.get("directory").ok_or("Expected directory field")?,
      &std::env::current_dir()?.join("../../../../").to_str().unwrap(),
  )?;

  let method = expect_string(input_obj.get("method").ok_or("Expected method field")?)?;
  let args = expect_object::<serde_json::Map<String, Value>>(
      input_obj.get("args").ok_or("Expected args field")?,
  )?;

  println!("Reading wrap.info & wrap.wasm from {}", wrap_dir);

  let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  println!("Creating WrapPackage from raw wrap.info & wrap.wasm bytes");

  let wrap_package = WasmPackage::new(Arc::new(SimpleFileReader::new()), Some(manifest), Some(wasm_module));

  println!("Adding WrapPackage to ClientConfig");

  let mut config = BuilderConfig::new(None);
  config.add_package("embed/foo".try_into()?, Arc::new(wrap_package));
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Invoking WrapPackage");

  let result = client.invoke_raw(
    &"embed/foo".try_into()?,
    &method,
    Some(&polywrap_client::msgpack::serialize(&args)?),
    None,
    None
  );

  if result.is_ok() {
    println!("Success!");
  }

  Ok(())
}
