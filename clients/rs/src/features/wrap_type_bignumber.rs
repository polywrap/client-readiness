use std::{error::Error};
use polywrap_client::{core::{uri::Uri, invoker::Invoker}, builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};
use serde::{Deserialize};
use serde_json::{Value};

use crate::input::expect_object;

#[derive(Deserialize)]
struct InputObj {
  args: Value,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let args = input_obj.args;

  let root = std::env::current_dir()?.join("../../../../wraps").to_str().unwrap();
  let uri: Uri = format!("fs/{root}/bignumber-type/implementations/as").try_into()?;

  let mut config: BuilderConfig = BuilderConfig::new(None);

  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  println!("Invoking method");

  let result = client.invoke_raw(
    &uri,
    "method",
    Some(&polywrap_client::msgpack::serialize(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      let bignumber: String = polywrap_client::msgpack::decode(&result)?;

      println!("Result: {bignumber}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
