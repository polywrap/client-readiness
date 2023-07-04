use std::{error::Error, fs, path::Path, sync::Arc};
use polywrap_client::{core::{file_reader::SimpleFileReader, uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper}, client::PolywrapClient, wasm::{wasm_package::WasmPackage}, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_uri, expect_object, expect_root_dir};

#[derive(Deserialize)]
struct InputObj {
  uri: Value,
  directory: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let uri = expect_uri(&input_obj.uri)?;
  let wrap_dir = expect_root_dir(
    &input_obj.directory,
    std::env::current_dir()?.join("../../").to_str().unwrap()
  )?;

  let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  let wrap_package = WasmPackage::from_bytecode(wasm_module, Arc::new(SimpleFileReader::new()), Some(manifest));

  let mut config = PolywrapClientConfig::new();
  config.add_package(uri.clone(), Arc::new(wrap_package));
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  let uri_string = uri.to_string();
  println!("Resolving URI: {uri_string}");

  let result = client.try_resolve_uri(&uri, None);

  if let Ok(result) = result {
    let result_type = match result {
        UriPackageOrWrapper::Uri(_) => "uri",
        UriPackageOrWrapper::Wrapper(_, _) => "wrapper",
        UriPackageOrWrapper::Package(_, _) => "package",
    };
    println!("Received: {result_type}");
    println!("Success!")
  }

  Ok(())
}
