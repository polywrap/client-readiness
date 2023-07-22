use std::error::Error;
use polywrap_client::{core::uri::Uri, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use serde::{Deserialize, Serialize};
use serde_json::{Value, to_string_pretty};

#[derive(Serialize, Deserialize)]
struct NestedInnerArg {
  prop: String
}

#[derive(Serialize, Deserialize)]
struct InnerArg {
  prop: String,
  nested: NestedInnerArg
}

#[derive(Serialize, Deserialize)]
struct Args {
  arg1: InnerArg
}

#[derive(Serialize, Deserialize)]
struct InputObj {
  method: String,
  args: Args,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let method = input_obj.method;
  let args = input_obj.args;

  let binding = std::env::current_dir()?.join("../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/object-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  println!("Invoking {method}");

  let result = client.invoke::<Value>(
    &uri,
    &method,
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      let stringified_value = to_string_pretty(&result)?;
      println!("Result: {stringified_value}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
