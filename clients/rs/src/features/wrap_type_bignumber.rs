use std::error::Error;
use polywrap_client::{core::uri::Uri, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
struct InnerArgObj {
  prop1: String
}

#[derive(Serialize, Deserialize)]
struct Args {
  arg1: String,
  obj: InnerArgObj
}

#[derive(Serialize, Deserialize)]
struct InputObj {
  args: Args,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let args = input_obj.args;

  let binding = std::env::current_dir()?.join("../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/bignumber-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  println!("Invoking method");

  let result = client.invoke::<Value>(
    &uri,
    "method",
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      let result = result.as_str().unwrap();
      println!("Result: {result}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
