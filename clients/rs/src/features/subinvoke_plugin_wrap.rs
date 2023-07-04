use polywrap_client::{
    client::PolywrapClient,
    core::{invoker::Invoker, uri::Uri, file_reader::SimpleFileReader, package::WrapPackage},
    plugin::{module::PluginModule, package::PluginPackage},
    wasm::wasm_package::WasmPackage, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
};
use serde::{Deserialize};
use serde_json::{Value};
use std::{
    error::Error,
    sync::{Arc, Mutex}, fs, path::Path,
};

use crate::{input::{expect_object, expect_root_dir, expect_string}, utils::get_default_manifest};

#[derive(Deserialize)]
struct InputObj {
    directory: Value,
    method: Value,
    args: Value,
}

#[derive(Debug)]
struct Plugin {
    subinvoke_args: Value,
    subinvoke_method: String,
    subinvoke_uri: Uri,
}

impl Plugin {
    pub fn new(subinvoke_args: Value, subinvoke_method: String, subinvoke_uri: Uri) -> Self {
        Self {
            subinvoke_args,
            subinvoke_method,
            subinvoke_uri,
        }
    }

    pub fn perform_subinvoke(
        &self,
        invoker: Arc<dyn Invoker>,
    ) -> Result<bool, polywrap_client::plugin::error::PluginError> {
        let subinvoke_method = &self.subinvoke_method;
        println!("Subinvoking {subinvoke_method}");
        let res = invoker.invoke_raw(
            &self.subinvoke_uri,
            subinvoke_method,
            Some(&polywrap_client::msgpack::to_vec(&self.subinvoke_args)?),
            None,
            None,
        );

        if let Ok(result) = res {
            println!("Received {result:?}");
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl PluginModule for Plugin {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        _: &[u8],
        _: Option<&[u8]>,
        invoker: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
    ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
        match method_name {
            "performSubinvoke" => {
                let result = self.perform_subinvoke(invoker)?;
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
    let wrap_dir = expect_root_dir(&input_obj.directory, root_dir)?;
    let subinvoke_method = expect_string(&input_obj.method)?;
    let subinvoke_args = input_obj.args;
    
    let manifest = fs::read(Path::new(&wrap_dir).join("wrap.info"))?;
    let wasm_module = fs::read(Path::new(&wrap_dir).join("wrap.wasm"))?;

    let wrap_package = WasmPackage::from_bytecode(wasm_module, Arc::new(SimpleFileReader::new()), Some(manifest));
    let wrap_uri: Uri =  "embed/foo".try_into().unwrap();

    let plugin_package = PluginPackage::new(
        Arc::new(Mutex::new(Plugin::new(
          subinvoke_args,
          subinvoke_method,
          wrap_uri.clone(),
        ))),
        get_default_manifest(),
    );

    let mut config = PolywrapClientConfig::new();
    let packages = vec![
      (wrap_uri, Arc::new(wrap_package) as Arc<dyn WrapPackage>),
      ("plugin/bar".try_into().unwrap(), Arc::new(plugin_package))
    ];
    config.add_packages(packages);

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Invoking Plugin");

    let result = client.invoke_raw(
      &"plugin/bar".try_into().unwrap(),
        "performSubinvoke",
        None,
        None,
        None,
    );

    if result.is_ok() {
        println!("Success!");
    }

    Ok(())
}
