use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{file_reader::SimpleFileReader, invoker::Invoker, package::WrapPackage, uri::Uri},
    plugin::{module::PluginModule},
    wasm::wasm_package::WasmPackage,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    error::Error,
    fs,
    path::Path,
    sync::{Arc},
};

use crate::{
    input::{expect_root_dir}
};

#[derive(Deserialize)]
struct InputObj {
    #[serde(rename = "rootWrap")]
    root_wrap: WrapDir,
    #[serde(rename = "subWrap")]
    sub_wrap: WrapDir,
    method: String,
    args: ArgsAdd,
}

#[derive(Deserialize)]
struct WrapDir {
    directory: String,
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct ArgsAdd {
    a: u32,
    b: u32,
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
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let binding = std::env::current_dir()?.join("../../");
    let root_dir = binding.to_str().unwrap();

    let method = input_obj.method;
    let args = input_obj.args;

    let root_wrap_obj = input_obj.root_wrap;

    let root_wrap_directory = expect_root_dir(&root_wrap_obj.directory, root_dir)?;

    let sub_wrap_obj = input_obj.sub_wrap;

    let sub_wrap_directory = expect_root_dir(&sub_wrap_obj.directory, root_dir)?;

    let root_manifest = fs::read(Path::new(&root_wrap_directory).join("wrap.info"))?;
    let root_wasm_module = fs::read(Path::new(&root_wrap_directory).join("wrap.wasm"))?;

    let root_wrap_package = WasmPackage::from_bytecode(
        root_wasm_module,
        Arc::new(SimpleFileReader::new()),
        Some(root_manifest),
    );
    let root_wrap_uri: Uri = root_wrap_obj.uri.try_into()?;

    let sub_manifest = fs::read(Path::new(&sub_wrap_directory).join("wrap.info"))?;
    let sub_wasm_module = fs::read(Path::new(&sub_wrap_directory).join("wrap.wasm"))?;

    let sub_wrap_package = WasmPackage::from_bytecode(
        sub_wasm_module,
        Arc::new(SimpleFileReader::new()),
        Some(sub_manifest),
    );
    let sub_wrap_uri: Uri = sub_wrap_obj.uri.try_into()?;

    let mut config = PolywrapClientConfig::new();
    let packages = vec![
        (
            root_wrap_uri.clone(),
            Arc::new(root_wrap_package) as Arc<dyn WrapPackage>,
        ),
        (
          sub_wrap_uri.clone(),
          Arc::new(sub_wrap_package) as Arc<dyn WrapPackage>,
      ),
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
