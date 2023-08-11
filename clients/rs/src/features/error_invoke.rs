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
struct InputObj {
    directory: String,
    uri: String,
    method: String,
    args: ArgsError,
}

#[derive(Serialize, Deserialize)]
struct ArgsError {
    error: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let binding = std::env::current_dir()?.join("../../");
    let root_dir = binding.to_str().unwrap();

    let method = input_obj.method;
    let args = input_obj.args;

    let root_wrap_directory = expect_root_dir(&input_obj.directory, root_dir)?;
    let root_manifest = fs::read(Path::new(&root_wrap_directory).join("wrap.info"))?;
    let root_wasm_module = fs::read(Path::new(&root_wrap_directory).join("wrap.wasm"))?;

    let root_wrap_package = WasmPackage::from_bytecode(
        root_wasm_module,
        Arc::new(SimpleFileReader::new()),
        Some(root_manifest),
    );
    let root_wrap_uri: Uri = input_obj.uri.try_into()?;

    let mut config = PolywrapClientConfig::new();
    let packages = vec![(
        root_wrap_uri.clone(),
        Arc::new(root_wrap_package) as Arc<dyn WrapPackage>,
    )];
    config.add_packages(packages);

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Invoking {method}");

    let result = client.invoke::<bool>(
        &root_wrap_uri,
        &method,
        Some(&polywrap_client::msgpack::to_vec(&args)?),
        None,
        None,
    );

    if let Err(result) = result {
        // @TODO: Extract error from result
        println!("Received error: {result}");
    }

    Ok(())
}
