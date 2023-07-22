use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{invoker::Invoker, uri::Uri},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize)]
struct Env {
  str: String,
  num: u8
}

#[derive(Deserialize)]
struct InputObj {
    uri: String,
    env: Env,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let uri: Uri = input_obj.uri.try_into()?;

    println!("Adding Env to ClientConfig");

    let mut config = PolywrapClientConfig::new();
    config.add_env(
        uri.clone(),
        polywrap_client::msgpack::to_vec(&input_obj.env)?,
    );

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Fetching Env");

    let result = client
        .get_env_by_uri(&uri);

    if let Some(result) = result {
        let result = polywrap_client::msgpack::from_slice::<Env>(&result)?;

        // TODO: section altered because of key-ordering

        let value = result.str;
        println!("env.str = {}", value);

        let value = result.num;
        println!("env.num = {}", value);

        println!("Success!")
    }

    Ok(())
}
