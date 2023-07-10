use std::{error::Error};
use polywrap_client::{core::{uri::Uri}, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use polywrap_client::msgpack::bytes;
use serde::{Deserialize, Serialize};
use serde_json::{Value};

#[derive(Serialize, Deserialize)]
struct InnerArgObj {
  #[serde(with = "bytes")]
  prop: Vec<u8>
}

#[derive(Serialize, Deserialize)]
struct InnerArg {
  arg: InnerArgObj
}

#[derive(Serialize, Deserialize)]
struct InputObj {
  method: String,
  args: InnerArg,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let method = input_obj.method;
  let args = input_obj.args;

  let binding = std::env::current_dir()?.join("../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/bytes-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  println!("Invoking {method}");

  let result = client.invoke::<bytes::ByteBuf>(
    &uri,
    &method,
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  match result {
    Ok(result_bytes) => {
      let mut result = String::from("[");

      for byte in result_bytes.to_vec() {
        result.push_str(&byte.to_string());
        result.push(',');
      }

      result.pop();
      result.push(']');

      println!("Result: {result}");
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
