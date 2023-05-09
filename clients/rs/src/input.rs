use serde::{de::{Deserializer, self}, Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct InputDir(String);

#[derive(Deserialize)]
struct InputDirIntermediate {
    input: String,
    #[serde(rename = "rootDir")]
    root_dir: String,
}

impl<'de> Deserialize<'de> for InputDir {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let intermediate = InputDirIntermediate::deserialize(deserializer)?;

        if intermediate.input.starts_with("$ROOT/") {
          Ok(InputDir(intermediate.input.replace("$ROOT/", &intermediate.root_dir)))
        } else {
          Err(de::Error::custom("expected a string that starts with $ROOT/"))
        }
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use crate::input::InputDir;

  #[test]
  pub fn it_deserializes_root_dir() {
    let value: Value = json!({
      "input": "$ROOT/foo/bar",
      "rootDir": "hee/"
    });


    let obj: InputDir = serde_json::from_value(value).unwrap();

    assert_eq!(obj.0, "hee/foo/bar")
  }
}