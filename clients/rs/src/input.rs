use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum InputError {
    ExpectedRootDir
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::ExpectedRootDir => f.write_str("expected a string that starts with $ROOT/"),
        }
    }
}

impl Error for InputError {}

pub fn expect_root_dir(input: &str, root_dir: &str) -> Result<String, InputError> {
    if !input.contains("$ROOT/") {
        return Err(InputError::ExpectedRootDir);
    }
    Ok(input.replace("$ROOT/", root_dir))
}
