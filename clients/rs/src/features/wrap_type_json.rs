use std::{error::Error};
use polywrap_client::{core::{uri::Uri}, builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};
use serde::{Deserialize};
use serde_json::{Value};

use crate::input::{expect_object, expect_string};

#[derive(Deserialize)]
struct InputObj {
  method: Value,
  args: Value,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let method = expect_string(&input_obj.method)?;
  let args = input_obj.args;

  let binding = std::env::current_dir()?.join("../../../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/json-type/implementations/as").try_into()?;

  let config: BuilderConfig = BuilderConfig::new(None);

  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Invoking {method}");

  let result = client.invoke::<String>(
    &uri,
    &method,
    Some(&polywrap_client::msgpack::serialize(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      println!("Result: {result:?}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
