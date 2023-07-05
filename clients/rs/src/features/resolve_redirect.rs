use std::{error::Error};
use polywrap_client::{client::PolywrapClient, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}, core::{uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper, uri::Uri}};
use serde::{Deserialize};
use serde_json::Value;

#[derive(Deserialize)]
struct InputObj {
  from: String,
  to: String
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let from: Uri = input_obj.from.try_into()?;
  let to: Uri = input_obj.to.try_into()?;

  println!("Adding URI Redirect to ClientConfig");

  let mut config = PolywrapClientConfig::new();
  config.add_redirect(from.clone(), to);
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Resolving Redirect");

  let result = client.try_resolve_uri(&from, None)?;

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received URI '{result_uri}'");
    println!("Success!");
  }

  Ok(())
}
