use std::{error::Error, sync::Arc};
use polywrap_client::{core::{resolvers::{uri_resolver::{UriResolver, UriResolverHandler}, uri_resolution_context::UriPackageOrWrapper, uri_resolver_like::UriResolverLike}}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_object, expect_string};

#[derive(Deserialize)]
struct InputObj {
  authority: Value,
  result: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let authority = expect_string(&input_obj.authority)?;
  let result = expect_string(&input_obj.result)?;

  println!("Adding Resolver to ClientConfig");

  #[derive(Debug)]
  struct Resolver {
    authority: String,
    result: String
  };

  impl UriResolver for Resolver {
    fn try_resolve_uri(
            &self,
            uri: &polywrap_client::core::uri::Uri,
            _: std::sync::Arc<dyn polywrap_client::core::invoker::Invoker>,
            _: &mut polywrap_client::core::resolvers::uri_resolution_context::UriResolutionContext,
        ) -> Result<UriPackageOrWrapper, polywrap_client::core::error::Error> {
      if uri.authority == self.authority {
        Ok(UriPackageOrWrapper::Uri(self.result.clone().try_into()?))
      } else {
        Ok(UriPackageOrWrapper::Uri(uri.clone()))
      }
    }
  }

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_resolver(UriResolverLike::Resolver(Arc::new(Resolver {
    authority: authority.clone(),
    result,
  })));
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Resolving a wrap://{authority} URI");

  let result = client.try_resolve_uri(&format!("wrap://${authority}/foo").try_into()?, None)?;
  
  //TODO: in JS there's a console.log in this line

  if let UriPackageOrWrapper::Uri(result_uri) = result {
    println!("Received URI '{result_uri}'");
    println!("Success!");
  }

  Ok(())
}
