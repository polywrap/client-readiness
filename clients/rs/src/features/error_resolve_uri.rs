use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{
        file_reader::SimpleFileReader, macros::uri, package::WrapPackage, uri::Uri,
        uri_resolver_handler::UriResolverHandler,
    },
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
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let binding = std::env::current_dir()?.join("../../");
    let root_dir = binding.to_str().unwrap();

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
    // @TODO(cbrzn): Change to https://github.com/polywrap/rust-client/blob/main/packages/resolver-extensions/src/extendable_uri_resolver.rs#L39
    // one a new rust client version is released
    config.add_packages(packages).add_interface_implementation(
        uri!("wrap://ens/uri-resolver.core.polywrap.eth"),
        root_wrap_uri.clone(),
    );

    let client: PolywrapClient = PolywrapClient::new(config.into());

    let result = client.try_resolve_uri(&uri!("expected-error/wrap"), None);

    if let Err(result) = result {
        // @TODO: Extract error from result
        println!("Received error: {result}");
    }

    Ok(())
}
