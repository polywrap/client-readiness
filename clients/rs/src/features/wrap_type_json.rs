use polywrap_client::{
    client::PolywrapClient, core::uri::Uri, msgpack::wrappers::polywrap_json::JSONString,
};
use polywrap_client_default_config::SystemClientConfig;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Args {
    Parse(ParseArgs),
    Stringify(StringifyArgs),
}

#[derive(Serialize, Deserialize)]
struct ParseArgs {
    value: JSONString,
}

#[derive(Serialize, Deserialize)]
struct StringifyArgs {
    values: Vec<JSONString>,
}

#[derive(Serialize, Deserialize)]
struct InputObj {
    method: String,
    args: Args,
}

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
  let input_obj: InputObj = serde_json::from_value(input.clone())?;
    let method = input_obj.method;
    let args = match input_obj.args {
        Args::Parse(a) => polywrap_client::msgpack::to_vec(&a)?,
        Args::Stringify(a) => polywrap_client::msgpack::to_vec(&a)?,
    };

    let binding = std::env::current_dir()?.join("../../wraps");
    let root = binding.to_str().unwrap();
    let uri: Uri = format!("fs/{root}/json-type/implementations/as")
        .try_into()
        .unwrap();

    let client: PolywrapClient = PolywrapClient::new(SystemClientConfig::default().into());

    println!("Invoking {method}");

    let result = client.invoke::<String>(&uri, &method, Some(&args), None, None);

    match result {
        Ok(result) => {
            println!("Result: {result:?}");
            println!("Success!");
        }
        Err(err) => panic!("{err}"),
    }

    Ok(())
}
