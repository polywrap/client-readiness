use std::{error::Error};
use polywrap_client::{core::{resolvers::{uri_resolver::{UriResolverHandler}, uri_resolution_context::UriPackageOrWrapper, uri_resolver_like::UriResolverLike}, uri::Uri}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient};
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

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_redirect(from.try_into()?, to.try_into()?);
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Resolving Redirect");

  let result = client.try_resolve_uri(&from.try_into()?, None)?;

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received URI '{result_uri}'");
    println!("Success!");
  }

  Ok(())
}
