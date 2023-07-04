use std::{error::Error};
use polywrap_client::{client::PolywrapClient, builder::{PolywrapClientConfig, PolywrapClientConfigBuilder}, core::{uri_resolver_handler::UriResolverHandler, resolution::uri_resolution_context::UriPackageOrWrapper}};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_object, expect_string};

#[derive(Deserialize)]
struct InputObj {
  from: Value,
  to: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let from = expect_string(&input_obj.from)?;
  let to = expect_string(&input_obj.to)?;

  println!("Adding URI Redirect to ClientConfig");

  let mut config = PolywrapClientConfig::new();
  config.add_redirect(from.clone().try_into().unwrap(), to.try_into().unwrap());
  
  let client: PolywrapClient = PolywrapClient::new(config.into());

  println!("Resolving Redirect");

  let result = client.try_resolve_uri(&from.try_into().unwrap(), None)?;

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received URI '{result_uri}'");
    println!("Success!");
  }

  Ok(())
}
