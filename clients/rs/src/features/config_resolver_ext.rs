use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{
        macros::uri, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
        uri_resolver_handler::UriResolverHandler,
    },
    plugin::{module::PluginModule, package::PluginPackage},
    msgpack::bytes
};
use polywrap_resolver_extensions::extendable_uri_resolver::ExtendableUriResolver;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use crate::utils::get_default_manifest;

#[derive(Debug)]
struct CustomResolverExt {
    authority: String,
    result: String,
}

#[derive(Deserialize, Clone, Debug)]
struct TryResolveUriArgs {
    authority: String,
}

#[derive(Deserialize, Serialize)]
struct TryResolveUriResult {
    uri: String,
    #[serde(with = "bytes")]
    manifest: Option<Vec<u8>>,
}

#[derive(Deserialize)]
struct InputObj {
    authority: String,
    result: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let authority = input_obj.authority;
    let result = input_obj.result;

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
                            manifest: None,
                            uri: self.result.clone().try_into().unwrap(),
                        };

                        return Ok(polywrap_client::msgpack::to_vec(&result)?);
                    }

                    let msgpack_null = vec![192];
                    Ok(msgpack_null)
                }
                _ => panic!("Method '{method_name}' not found"),
            }
        }
    }

    println!("Adding CustomResolverExt & ExtendableUriResolver to ClientConfig");

    let plugin_package = PluginPackage::new(
        Arc::new(Mutex::new(CustomResolverExt {
            authority: authority.clone(),
            result,
        })),
        get_default_manifest(),
    );

    let mut config = PolywrapClientConfig::new();
    let resolver_uri = uri!("wrap://plugin/custom-resolver");

    config.add_package(resolver_uri.clone(), Arc::new(plugin_package));

    config.add_interface_implementation(
        uri!("wrap://ens/uri-resolver.core.polywrap.eth"),
        resolver_uri,
    );

    config.add_resolver(Arc::new(ExtendableUriResolver::new(None)));

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Resolving a wrap://{authority} URI");

    let uri: Uri = format!("wrap://{authority}/foo").try_into().unwrap();
    let result = client.try_resolve_uri(&uri, None)?;

    if let UriPackageOrWrapper::Uri(result_uri) = result {
        let result_uri = result_uri.uri();
        println!("Received URI '{result_uri}'");
        println!("Success!");
    }

    Ok(())
}
