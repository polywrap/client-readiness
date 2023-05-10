use std::{error::Error};
use polywrap_client::{core::{client::Client}, builder::types::{BuilderConfig, ClientBuilder, ClientConfigHandler}, client::PolywrapClient};
use serde::{Deserialize};
use serde_json::Value;

use crate::input::{expect_object, expect_uri};

#[derive(Deserialize)]
struct InputObj {
  uri: Value,
  env: Value
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let uri = expect_uri(&input_obj.uri)?;

  println!("Adding Env to ClientConfig");

  let mut config: BuilderConfig = BuilderConfig::new(None);
  config.add_env(uri.clone(), input_obj.env);
  
  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Fetching Env");

  let result = client.get_env_by_uri(&uri);

  if let Some(result) = result {
    for key in result.as_object().unwrap().keys() {
      let value = result.get(key).unwrap();
      println!("env.{key} = {value}");
    }
    println!("Success!")
  }

  Ok(())
}
