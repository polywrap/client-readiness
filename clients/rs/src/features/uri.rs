use std::error::Error;
use serde_json::Value;

use polywrap_client::core::uri::Uri;
use crate::input::Input;

pub fn run_test_case(input: &Value) -> Result<(), Box<dyn Error>> {
    let str = Input::expect_string(input)?;

    let uri = Uri::new(str.as_str());

    println!("WRAP URI successfully created.");
    println!("uri - {}", uri);
    println!("uri.authority - {}", uri.authority);
    println!("uri.path - {}", uri.path);
    Ok(())
}
