use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{file_reader::SimpleFileReader, macros::uri, package::WrapPackage, uri::Uri},
    wasm::wasm_package::WasmPackage,
};
use serde::Deserialize;
use serde_json::Value;
use std::{error::Error, fs, path::Path, sync::Arc};

use crate::input::expect_root_dir;

#[derive(Deserialize)]
struct WrapDir {
    directory: String,
    uri: String,
}

#[derive(Deserialize)]
struct InputObj {
    resolver: Option<WrapDir>,
    uri: String,
    #[serde(rename = "expectedError")]
    expected_error: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let binding = std::env::current_dir()?.join("../../");
    let root_dir = binding.to_str().unwrap();

    let mut config = PolywrapClientConfig::new();
    if let Some(resolver) = input_obj.resolver {
        let root_wrap_directory = expect_root_dir(&resolver.directory, root_dir)?;
        let root_manifest = fs::read(Path::new(&root_wrap_directory).join("wrap.info"))?;
        let root_wasm_module = fs::read(Path::new(&root_wrap_directory).join("wrap.wasm"))?;

        let root_wrap_package = WasmPackage::from_bytecode(
            root_wasm_module,
            Arc::new(SimpleFileReader::new()),
            Some(root_manifest),
        );
        let resolver_uri: Uri = resolver.uri.try_into()?;
        config
            .add_package(
                resolver_uri.clone(),
                Arc::new(root_wrap_package) as Arc<dyn WrapPackage>,
            )
            // @TODO(cbrzn):
            // Change to https://github.com/polywrap/rust-client/blob/main/packages/resolver-extensions/src/extendable_uri_resolver.rs#L39
            // one a new rust client version is released
            .add_interface_implementation(
                uri!("wrap://ens/uri-resolver.core.polywrap.eth"),
                resolver_uri,
            );
    }

    let client: PolywrapClient = PolywrapClient::new(config.into());
    let uri: Uri = input_obj.uri.try_into()?;
    let result = client.invoke::<bool>(&uri, "", None, None, None);

    if let Err(result) = result {
        if result.to_string().contains(&input_obj.expected_error) {
            println!("Expected error received")
        } else {
            println!(
                "Expected error {}, but received {}",
                &input_obj.expected_error,
                result.to_string()
            );
        }
    }

    Ok(())
}
