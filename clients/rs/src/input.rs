use polywrap_client::core::uri::Uri;
use serde_json::Value;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InputError {
    ExpectedRootDir,
    ExpectedString,
    ExpectedUri,
    ExpectedArray,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::ExpectedRootDir => f.write_str("expected a string that starts with $ROOT/"),
            InputError::ExpectedString => f.write_str("expected a string"),
            InputError::ExpectedUri => f.write_str("expected a valid WRAP URI"),
            InputError::ExpectedArray => f.write_str("expected an array"),
        }
    }
}

impl Error for InputError {}

pub fn expect_root_dir(input: &Value, root_dir: &str) -> Result<String, InputError> {
    let s = input.as_str().ok_or(InputError::ExpectedRootDir)?;
    if !s.contains("$ROOT/") {
        return Err(InputError::ExpectedRootDir);
    }
    Ok(s.replace("$ROOT/", root_dir))
}

pub fn expect_string(input: &Value) -> Result<String, InputError> {
    input.as_str().map(String::from).ok_or(InputError::ExpectedString)
}

pub fn expect_uri(input: &Value) -> Result<Uri, InputError> {
    let s = input.as_str().ok_or(InputError::ExpectedUri)?;
    Uri::try_from(s).map_err(|_| InputError::ExpectedUri)
}

pub fn expect_array<T: serde::de::DeserializeOwned>(input: &Value) -> Result<Vec<T>, InputError> {
    if !input.is_array() {
        return Err(InputError::ExpectedArray);
    }
    serde_json::from_value(input.clone()).map_err(|_| InputError::ExpectedArray)
}