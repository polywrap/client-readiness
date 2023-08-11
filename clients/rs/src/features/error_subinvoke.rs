use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{file_reader::SimpleFileReader, package::WrapPackage, uri::Uri},
    wasm::wasm_package::WasmPackage,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{error::Error, fs, path::Path, sync::Arc};

use crate::input::expect_root_dir;

#[derive(Deserialize)]
struct InvokeWrap {
    directory: String,
    uri: String,
    method: String,
    args: ArgsError,
}

#[derive(Deserialize)]
struct SubinvokeWrap {
    directory: String,
    uri: String,
}

#[derive(Deserialize)]
struct InputObj {
    #[serde(rename = "subinvokeWrap")]
    subinvoke_wrap: SubinvokeWrap,
    #[serde(rename = "invokeWrap")]
    invoke_wrap: InvokeWrap,
}

#[derive(Serialize, Deserialize)]
struct ArgsError {
    error: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let binding = std::env::current_dir()?.join("../../");
    let root_dir = binding.to_str().unwrap();

    let subinvoke_wrap = input_obj.subinvoke_wrap;
    let subinvoke_wrap_directory = expect_root_dir(&subinvoke_wrap.directory, root_dir)?;
    let subinvoke_manifest = fs::read(Path::new(&subinvoke_wrap_directory).join("wrap.info"))?;
    let subinvoke_wasm_module = fs::read(Path::new(&subinvoke_wrap_directory).join("wrap.wasm"))?;
    let subinvoke_wrap_package = WasmPackage::from_bytecode(
        subinvoke_wasm_module,
        Arc::new(SimpleFileReader::new()),
        Some(subinvoke_manifest),
    );
    let subinvoke_wrap_uri: Uri = subinvoke_wrap.uri.try_into()?;

    let invoke_wrap = input_obj.invoke_wrap;

    let method = invoke_wrap.method;
    let args = invoke_wrap.args;

    let invoke_wrap_directory = expect_root_dir(&invoke_wrap.directory, root_dir)?;
    let invoke_manifest = fs::read(Path::new(&invoke_wrap_directory).join("wrap.info"))?;
    let invoke_wasm_module = fs::read(Path::new(&invoke_wrap_directory).join("wrap.wasm"))?;

    let invoke_wrap_package = WasmPackage::from_bytecode(
        invoke_wasm_module,
        Arc::new(SimpleFileReader::new()),
        Some(invoke_manifest),
    );
    let invoke_wrap_uri: Uri = invoke_wrap.uri.try_into()?;

    let mut config = PolywrapClientConfig::new();
    let packages = vec![
        (
            invoke_wrap_uri.clone(),
            Arc::new(invoke_wrap_package) as Arc<dyn WrapPackage>,
        ),
        (
            subinvoke_wrap_uri.clone(),
            Arc::new(subinvoke_wrap_package) as Arc<dyn WrapPackage>,
        ),
    ];
    config.add_packages(packages);

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Invoking {method}");

    let result = client.invoke::<bool>(
        &invoke_wrap_uri,
        &method,
        Some(&polywrap_client::msgpack::to_vec(&args)?),
        None,
        None,
    );

    if let Err(result) = result {
        println!("Received error: {result}");
    }

    Ok(())
}
