use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{invoker::Invoker, uri::Uri},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, error::Error};

use crate::input::expect_object;

#[derive(Deserialize)]
struct InputObj {
    #[serde(rename = "mainEnv")]
    main_env: Value,
    #[serde(rename = "extEnv")]
    ext_env: Value,
}

#[derive(Deserialize)]
struct SubinvokeMethodResult {
    local: Value,
    external: Value,
}

#[derive(Serialize)]
struct InvokeArgs {
    arg: String,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let input_obj = expect_object::<InputObj>(input)?;
    let main_env = polywrap_client::msgpack::to_vec(&input_obj.main_env)?;
    let ext_env = polywrap_client::msgpack::to_vec(&input_obj.ext_env)?;

    let root = std::env::current_dir()?.join("../../wraps");
    let binding = root.join("/env-type/00-external/implementations/as");
    let external_wrapper_path = binding.to_str().unwrap();
    let external_wrapper_uri: Uri = format!("file/{external_wrapper_path}").try_into().unwrap();

    let binding = root.join("/env-type/01-main/implementations/as");
    let wrapper_path = binding.to_str().unwrap();
    let wrapper_uri: Uri = format!("file/{wrapper_path}").try_into().unwrap();

    let mut envs: HashMap<Uri, Vec<u8>> = HashMap::new();
    envs.insert(wrapper_uri.clone(), main_env);
    envs.insert(external_wrapper_uri.clone(), ext_env);

    let mut config = PolywrapClientConfig::new();
    config.add_envs(envs);
    config.add_redirect(
        "ens/external-env.polywrap.eth".try_into().unwrap(),
        external_wrapper_uri,
    );

    let client: PolywrapClient = PolywrapClient::new(config.into());

    println!("Invoking methodRequireEnv");

    let result = client.invoke_raw(
        &wrapper_uri,
        "methodRequireEnv",
        Some(&polywrap_client::msgpack::to_vec(&InvokeArgs {
            arg: "string".to_string(),
        })?),
        None,
        None,
    );

    if let Err(err) = result {
        panic!("{err}")
    }

    println!("Success!");
    println!("Invoking subinvokeEnvMethod");

    let subinvoke_env_method_result = client.invoke::<SubinvokeMethodResult>(
        &wrapper_uri,
        "subinvokeEnvMethod",
        Some(&polywrap_client::msgpack::to_vec(&InvokeArgs {
            arg: "string".to_string(),
        })?),
        None,
        None,
    );

    match subinvoke_env_method_result {
        Ok(result) => {
            let result_local = result.local.to_string();
            let result_external = result.external.to_string();

            println!("response.local exists: {result_local}");
            println!("response.external exists: {result_external}");
            println!("Success!");
        }
        Err(err) => panic!("{err}"),
    }

    Ok(())
}
