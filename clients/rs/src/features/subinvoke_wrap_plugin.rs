use polywrap_client::{
  plugin::{module::PluginModule, package::PluginPackage},
  wasm::wasm_package::WasmPackage, core::{file_reader::SimpleFileReader, package::WrapPackage, invoker::Invoker}, client::PolywrapClient, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use std::{
  error::Error,
  fs, path::Path, sync::{Arc, Mutex},
};

use crate::{input::{expect_object, expect_root_dir, expect_string, expect_uri}, utils::get_default_manifest};

#[derive(Deserialize)]
struct InputObj {
  #[serde(rename = "rootWrap")]
  root_wrap: Value,
  #[serde(rename = "subWrapUri")]
  sub_wrap_uri: Value,
  method: Value,
  args: Value,
}

#[derive(Deserialize)]
struct RootWrap {
  directory: Value,
  uri: Value
}

#[derive(Serialize, Deserialize)]
struct ArgsAdd {
  a: u32,
  b: u32
}

#[derive(Debug)]
struct Plugin {}

impl PluginModule for Plugin {
  fn _wrap_invoke(
      &mut self,
      method_name: &str,
      params: &[u8],
      _: Option<&[u8]>,
      _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
  ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
      match method_name {
          "add" => {
              let args: ArgsAdd = polywrap_client::msgpack::from_slice(params)?;
              let result = args.a + args.b;

              Ok(polywrap_client::msgpack::to_vec(&result)?)
          }
          _ => panic!("Unrecognized method: {method_name}"),
      }
  }
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let binding = std::env::current_dir()?
      .join("../../");
  let root_dir = binding
      .to_str()
      .unwrap();
  let sub_wrap_uri = expect_uri(&input_obj.sub_wrap_uri)?;
  let method = expect_string(&input_obj.method)?;
  let args = input_obj.args;
  
  let root_wrap_obj = expect_object::<RootWrap>(&input_obj.root_wrap)?;

  let root_wrap_directory = expect_root_dir(
    &root_wrap_obj.directory,
    root_dir
  )?;

  let manifest = fs::read(Path::new(&root_wrap_directory).join("wrap.info"))?;
  let wasm_module = fs::read(Path::new(&root_wrap_directory).join("wrap.wasm"))?;

  let wrap_package = WasmPackage::from_bytecode(wasm_module, Arc::new(SimpleFileReader::new()), Some(manifest));
  let root_wrap_uri = expect_uri(&root_wrap_obj.uri)?;
  
  let sub_wrap_package = PluginPackage::new(
      Arc::new(Mutex::new(Plugin {})),
      get_default_manifest(),
  );

  let mut config = PolywrapClientConfig::new();
  let packages = vec![
    (root_wrap_uri.clone(), Arc::new(wrap_package) as Arc<dyn WrapPackage>),
    (sub_wrap_uri, Arc::new(sub_wrap_package))
  ];
  config.add_packages(packages);

  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Invoking {method}");

  let result = client.invoke_raw(
    &root_wrap_uri,
      &method,
      Some(&polywrap_client::msgpack::to_vec(&args)?),
      None,
      None,
  );

  if let Ok(result) = result {
      println!("Received: {result:?}");
      println!("Success!");
  }

  Ok(())
}
