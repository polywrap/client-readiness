use std::{error::Error};
use polywrap_client::{core::{resolvers::{uri_resolver::{UriResolverHandler}, uri_resolution_context::UriPackageOrWrapper}}, builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};
use serde_json::Value;

use crate::input::{expect_uri};

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let uri = expect_uri(input)?;
  let uri_authority = &uri.authority;
  let uri_string = uri.to_string();

  println!("URI Authority: {uri_authority}");

  let config: BuilderConfig = BuilderConfig::new(None);
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Resolving: {uri_string}");

  let result = client.try_resolve_uri(&uri.try_into()?, None)?;
  let result_type = match result {
    UriPackageOrWrapper::Uri(_) => "uri",
    UriPackageOrWrapper::Wrapper(_, _) => "wrapper",
    UriPackageOrWrapper::Package(_, _) => "package",
  };

  if let UriPackageOrWrapper::Uri(_) = result {
    println!("Received: '{result_type}'");
    println!("Success!");
  }

  Ok(())
}
