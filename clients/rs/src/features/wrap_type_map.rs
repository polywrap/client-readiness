use std::{error::Error, collections::HashMap};
use polywrap_client::{core::uri::Uri, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
struct InputObj {
  map: HashMap<String, i32>
}

#[derive(Serialize)]
struct ReturnMapArgs {
  map: HashMap<String, i32>
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
  let map = input_obj.map;

  let binding = std::env::current_dir()?.join("../../wraps");
  let root = binding.to_str().unwrap();
  let uri: Uri = format!("fs/{root}/map-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  let mut map_class: HashMap<String, i32> = HashMap::new();

  for entry in map {
    map_class.insert(entry.0, entry.1);
  }

  println!("Invoking returnMap");

  let args = ReturnMapArgs { map: map_class };

  let result = client.invoke::<HashMap<String, Value>>(
    &uri,
    "returnMap",
    Some(&polywrap_client::msgpack::to_vec(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      for entry_key in result.keys() {
        let entry_value = result.get(entry_key).unwrap();
        println!("key '{entry_key}' = {entry_value}");
      }
      
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
