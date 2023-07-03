use std::{error::Error, sync::{Arc, Mutex}};
use polywrap_client::{core::{uri::Uri, resolution::uri_resolution_context::UriPackageOrWrapper}, client::PolywrapClient, plugin::{module::PluginModule, package::PluginPackage}, wrap_manifest::versions::{WrapManifest01, WrapManifest01Abi}, builder::PolywrapClientConfigBuilder};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{input::{expect_object, expect_string}, utils::get_default_manifest};

#[derive(Debug)]
struct CustomResolverExt {
  authority: String,
  result: String
}

#[derive(Deserialize)]
struct TryResolveUriArgs {
  authority: String
}

#[derive(Deserialize, Serialize)]
struct TryResolveUriResult {
  uri: String,
  manifest: WrapManifest01
}

#[derive(Deserialize)]
struct InputObj {
  authority: Value,
  result: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let authority = expect_string(&input_obj.authority)?;
  let result = expect_string(&input_obj.result)?;

  println!("Creating CustomResolverExt Plugin");

  impl PluginModule for CustomResolverExt {
    fn _wrap_invoke(
        &mut self,
        method_name: &str,
        params: &[u8],
        _: Option<&[u8]>,
        _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
    ) -> Result<Vec<u8>, polywrap_client::plugin::error::PluginError> {
        match method_name {
          "tryResolveUri" => {
            let args: TryResolveUriArgs = polywrap_client::msgpack::from_slice(params)?;

            if args.authority == self.authority {
              let result = TryResolveUriResult {
                manifest: WrapManifest01 {
                  abi: WrapManifest01Abi {
                      enum_types: None,
                      env_type: None,
                      imported_enum_types: None,
                      imported_env_types: None,
                      imported_module_types: None,
                      imported_object_types: None,
                      interface_types: None,
                      module_type: None,
                      object_types: None,
                      version: None,
                  },
                  name: String::from(""),
                  type_: String::from(""),
                  version: String::from(""),
                },
                uri: self.result.clone().try_into().unwrap()
              };

              return Ok(polywrap_client::msgpack::to_vec(&result)?)
            }

            let msgpack_null = vec![192]; 
            Ok(msgpack_null)
          },
          _ => panic!("Method '{method_name}' not found")
        }
    }
}

  println!("Adding CustomResolverExt & ExtendableUriResolver to ClientConfig");

  let plugin_package = PluginPackage::new(
    Arc::new(Mutex::new(Box::new(CustomResolverExt { authority: authority.clone(), result }))),
    get_default_manifest()
  );

  let mut config = PolywrapClientConfigBuilder::new(None);
  config.add_package(
    "wrap://plugin/custom-resolver".try_into()?,
    Arc::new(plugin_package)
  );
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Resolving a wrap://{authority} URI");

  let uri: Uri = format!("wrap://{authority}/foo").try_into()?;
  let result = client.try_resolve_uri(&uri, None)?;

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received URI '{result_uri}'");
    println!("Success!");
  }

  Ok(())
}