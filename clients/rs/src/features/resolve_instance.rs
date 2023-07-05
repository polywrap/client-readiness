use std::{error::Error, fs, path::Path, sync::Arc};
use polywrap_client::{client::PolywrapClient, wasm::{wasm_wrapper::WasmWrapper}, core::{file_reader::SimpleFileReader, uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri}, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_root_dir};

#[derive(Deserialize)]
struct InputObj {
  uri: String,
  directory: String
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let uri: Uri = input_obj.uri.try_into()?;
  let wrap_dir = expect_root_dir(
    &input_obj.directory,
    std::env::current_dir()?.join("../../").to_str().unwrap()
  )?;

  dbg!(Path::new(&wrap_dir).join("wrap.info"));

  let _manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

  let wrap_instance = WasmWrapper::try_from_bytecode(&wasm_module, Arc::new(SimpleFileReader::new()))?;

  let mut config = PolywrapClientConfig::new();
  config.add_wrapper(uri.clone(), Arc::new(wrap_instance));
  
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
