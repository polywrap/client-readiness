use core::fmt;
use std::{error::Error};
use polywrap_client::{core::{uri::Uri}, client::PolywrapClient};
use polywrap_client_default_config::SystemClientConfig;
use serde::{Deserialize, Serialize, de, Deserializer};
use serde_json::{Value};

#[derive(Serialize)]
enum En {
  OPTION1,
  OPTION2,
  OPTION3
}

impl<'de> Deserialize<'de> for En {
  fn deserialize<D: Deserializer<'de>>(d:D) -> Result<En, D::Error> {
      struct Visitor;
      impl<'de> de::Visitor<'de> for Visitor {
          type Value = En;

          fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
              formatter.write_str("a prime as integer or text")
          }

          fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
              Ok(match value {
                  0 => En::OPTION1,
                  1 => En::OPTION2,
                  2 => En::OPTION3,
                  _ => return Err(E::invalid_value(de::Unexpected::Unsigned(value), &self)),
              })
          }

          fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
              Ok(match value {
                  "OPTION1" => En::OPTION1,
                  "OPTION2" => En::OPTION2,
                  "OPTION3" => En::OPTION3,
                  _ => return Err(E::invalid_value(de::Unexpected::Str(value), &self)),
              })
          }
      }
      d.deserialize_any(Visitor)
  }
}

#[derive(Serialize, Deserialize)]
struct Args {
  en: En
}

#[derive(Deserialize)]
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
  let uri: Uri = format!("fs/{root}/enum-type/implementations/as").try_into().unwrap();

  let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

  println!("Invoking {method}");

  let result = client.invoke::<u8>(
    &uri,
    &method,
    Some(&polywrap_client::msgpack::to_vec(&args)?),
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
