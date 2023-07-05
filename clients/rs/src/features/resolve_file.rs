use std::{error::Error};
use polywrap_client::{client::PolywrapClient, builder::PolywrapClientConfig, core::{uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri}};
use serde_json::Value;

use crate::input::{expect_root_dir};

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_uri: Uri = serde_json::from_value::<String>(input.clone())?.try_into()?;
  let root_dir = expect_root_dir(
    &Value::String(input_uri.path().to_string()),
    std::env::current_dir()?.join("../../").to_str().unwrap()
  )?;
  let uri_authority = input_uri.authority();
  let uri = format!("{uri_authority}/{root_dir}");
  
  let uri_string = uri.to_string();

  println!("URI Authority: {uri_authority}");

  let config = PolywrapClientConfig::new();
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Resolving: {uri_string}");

  let result = client.try_resolve_uri(&uri.try_into().unwrap(), None)?;
  let result_type = match result {
    UriPackageOrWrapper::Uri(_) => "uri",
    UriPackageOrWrapper::Wrapper(_, _) => "wrapper",
    UriPackageOrWrapper::Package(_, _) => "package",
  };

  if let UriPackageOrWrapper::Uri(_result_uri) = result {
    println!("Received: '{result_type}'");
    println!("Success!");
  }

  Ok(())
}