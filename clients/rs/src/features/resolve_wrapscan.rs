use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{
        resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri,
        uri_resolver_handler::UriResolverHandler,
    },
};
use polywrap_client_default_config::SystemClientConfig;
use serde_json::Value;
use std::error::Error;

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let uri: Uri = serde_json::from_value::<String>(input.clone())?.try_into()?;
    let uri_authority = uri.authority();
    let uri_string = uri.to_string();

    println!("URI Authority: {uri_authority}");

    let mut config = PolywrapClientConfig::new();
    config.add(SystemClientConfig::default().into());

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Resolving: {uri_string}");

    let result = client.try_resolve_uri(&uri.try_into()?, None)?;
    let result_type = match result {
        UriPackageOrWrapper::Uri(_) => "uri",
        UriPackageOrWrapper::Wrapper(_, _) => "wrapper",
        UriPackageOrWrapper::Package(_, _) => "package",
    };

    if let UriPackageOrWrapper::Package(_, _) = result {
        println!("Received: {result_type}");
        println!("Success!");
    }

    Ok(())
}
