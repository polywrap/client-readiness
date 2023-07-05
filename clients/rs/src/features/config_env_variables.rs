use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::invoker::Invoker,
};
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

use crate::input::{expect_uri};

#[derive(Deserialize)]
struct InputObj {
    uri: Value,
    env: Value,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let uri = expect_uri(&input_obj.uri)?;

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
        let result = polywrap_client::msgpack::from_slice::<Value>(&result)?;
        for key in result.as_object().unwrap().keys() {
            let value = result.get(key).unwrap();
            println!("env.{key} = {value}");
        }
        println!("Success!")
    }

    Ok(())
}
