use std::{error::Error};
use polywrap_client::{client::PolywrapClient, builder::PolywrapClientConfig, core::{uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri}};
use polywrap_client_default_config::SystemClientConfig;
use serde_json::Value;

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let uri: Uri = serde_json::from_value::<String>(input.clone())?.try_into()?;
  let uri_authority = uri.authority();

  println!("URI Authority: {uri_authority}");

  let config = SystemClientConfig::default();
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Resolving: {}", input.to_string());

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