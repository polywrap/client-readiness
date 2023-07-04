use polywrap_client::{
    builder::{PolywrapClientConfig, PolywrapClientConfigBuilder},
    client::PolywrapClient,
    core::{invoker::Invoker, uri::Uri},
};
use serde::Serialize;
use serde_json::Value;
use std::error::Error;

#[derive(Serialize)]
struct InnerMostArg {
    uint8: u8,
    str: String,
}

#[derive(Serialize)]
struct InnerArg {
    arg: InnerMostArg,
}

#[derive(Serialize)]
struct InvokeArgs {
    args: InnerArg,
}

pub fn run_test_case(_: &Value) -> Result<(), Box<dyn Error>> {
    let root = std::env::current_dir()?.join("../../wraps");
    let interface_uri: Uri = "wrap://ens/interface.eth".try_into().unwrap();
    let binding = root.join("/interface-invoke/01-implementation/implementations/as");
    let implementation_path = binding.to_str().unwrap();
    let implementation_uri: Uri = format!("file/{implementation_path}").try_into().unwrap();

    let mut config = PolywrapClientConfig::new();
    config.add_interface_implementation(interface_uri, implementation_uri);

    let client: PolywrapClient = PolywrapClient::new(config.into());

    let binding = root.join("/interface-invoke/02-wrapper/implementations/as");
    let wrapper_path = binding.to_str().unwrap();
    let wrapper_uri: Uri = format!("fs/{wrapper_path}").try_into().unwrap();

    println!("Invoking moduleMethod");

    let result = client.invoke_raw(
        &wrapper_uri,
        "moduleMethod",
        Some(&polywrap_client::msgpack::to_vec(&InvokeArgs {
            args: InnerArg {
                arg: InnerMostArg {
                    uint8: 1,
                    str: "Test String 1".to_string(),
                },
            },
        })?),
        None,
        None,
    );

    if let Err(err) = result {
        panic!("{err}")
    }

    println!("Success!");

    Ok(())
}
