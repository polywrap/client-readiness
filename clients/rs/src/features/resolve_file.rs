use std::{error::Error};
use polywrap_client::{core::{resolvers::{uri_resolver::{UriResolverHandler}, uri_resolution_context::UriPackageOrWrapper, uri_resolver_like::UriResolverLike}, uri::Uri}, builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};
use serde_json::Value;

use crate::input::{expect_uri, expect_root_dir};

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_uri = expect_uri(input)?;
  let root_dir = expect_root_dir(
    &Value::String(input_uri.path),
    &std::env::current_dir()?.join("../../../../").to_str().unwrap()
  )?;
  let uri_authority = input_uri.authority;
  let uri = format!("{uri_authority}/{root_dir}");
  
  let uri_string = uri.to_string();

  println!("URI Authority: {uri_authority}");

  let mut config: BuilderConfig = BuilderConfig::new(None);
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Resolving: {uri_string}");

  let result = client.try_resolve_uri(&uri.try_into()?, None)?;
  let result_type = match result {
    UriPackageOrWrapper::Uri(_) => "uri",
    UriPackageOrWrapper::Wrapper(_, _) => "wrapper",
    UriPackageOrWrapper::Package(_, _) => "package",
  };

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received: '{result_type}'");
    println!("Success!");
  }

  Ok(())
}