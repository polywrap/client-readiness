use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{
        resolution::{
            uri_resolution_context::{UriPackageOrWrapper, UriResolutionContext},
            uri_resolver::UriResolver,
        },
        uri_resolver_handler::UriResolverHandler,
    },
};
use serde::Deserialize;
use serde_json::Value;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};

#[derive(Deserialize)]
struct InputObj {
    authority: String,
    result: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let authority = input_obj.authority;
    let result = input_obj.result;

    println!("Adding Resolver to ClientConfig");

    #[derive(Debug)]
    struct Resolver {
        authority: String,
        result: String,
    }

    impl UriResolver for Resolver {
        fn try_resolve_uri(
            &self,
            uri: &polywrap_client::core::uri::Uri,
            _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
            _: Arc<Mutex<UriResolutionContext>>,
        ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
            if uri.authority() == self.authority {
                Ok(UriPackageOrWrapper::Uri(self.result.clone().try_into()?))
            } else {
                Ok(UriPackageOrWrapper::Uri(uri.clone()))
            }
        }
    }

    let mut config = PolywrapClientConfig::new();
    config.add_resolver(Arc::new(Resolver {
        authority: authority.clone(),
        result,
    }));

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Resolving a wrap://{authority} URI");

    let result = client.try_resolve_uri(
        &format!("wrap://{authority}/foo").try_into().unwrap(),
        None,
    )?;

    //TODO: in JS there's a console.log in this line

    if let UriPackageOrWrapper::Uri(result_uri) = result {
        println!("Received URI '{result_uri}'");
        println!("Success!");
    }

    Ok(())
}
