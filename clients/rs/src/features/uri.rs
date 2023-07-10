use std::error::Error;
use serde_json::Value;

use polywrap_client::core::uri::Uri;

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let str = serde_json::from_value::<String>(input.clone())?;

    let uri = Uri::try_from(str);

    match uri {
        Ok(uri) => {
          println!("WRAP URI successfully created.");
          println!("uri - {uri}");
          println!("uri.authority - {}", uri.authority());
          println!("uri.path - {}", uri.path());
        },
        Err(e) => {
          println!("!Test Error [uri.invalid_uri]");
          println!("{e}");
        },
    }

    Ok(())
}
