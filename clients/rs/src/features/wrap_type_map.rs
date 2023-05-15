use std::{error::Error, collections::{BTreeMap}};
use polywrap_client::{core::{uri::Uri}, builder::types::{BuilderConfig, ClientConfigHandler}, client::PolywrapClient};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use polywrap_client::msgpack::extensions::generic_map::GenericMap;

use crate::input::{expect_object};

#[derive(Deserialize)]
struct InputObj {
  map: Value
}

#[derive(Serialize)]
struct ReturnMapArgs {
  map: GenericMap<String, i32>
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj = expect_object::<InputObj>(input)?;
  let map = expect_object::<GenericMap<String, i32>>(&input_obj.map)?;

  let root = std::env::current_dir()?.join("../../../../wraps").to_str().unwrap();
  let uri: Uri = format!("fs/{root}/map-type/implementations/as").try_into()?;

  let mut config: BuilderConfig = BuilderConfig::new(None);

  let config = config.build();
  let client: PolywrapClient = PolywrapClient::new(config);

  let mut map_class: BTreeMap<String, i32> = BTreeMap::new();

  for entry in map.0 {
    map_class.insert(entry.0, entry.1);
  }

  println!("Invoking returnMap");

  let generic_map_class = GenericMap(map_class);
  let args = ReturnMapArgs { map: generic_map_class };

  let result = client.invoke::<GenericMap<String, Value>>(
    &uri,
    "returnMap",
    Some(&polywrap_client::msgpack::serialize(&args)?),
    None,
    None
  );

  match result {
    Ok(result) => {
      for entry_key in result.0.keys() {
        let entry_value = result.0.get(entry_key).unwrap();
        println!("key '{entry_key}' = {entry_value}");
      }
      
      println!("Success!");
    },
    Err(err) => panic!("{err}"),
  }
  
  Ok(())
}
