use std::fs;
use std::error::Error;
use std::path::Path;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCases {
    pub input: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Spec {
    pub required: bool,
    pub cases: HashMap<String, TestCases>,
}

pub type FeatureSpecs = HashMap<String, Spec>;

pub fn load(directory: &Path) -> Result<FeatureSpecs, Box<dyn Error>> {
    let mut feature_specs: FeatureSpecs = HashMap::new();
    let spec_files = fs::read_dir(directory)?;

    for spec_file in spec_files {
        let spec_file = spec_file?.path();
        let spec_name = spec_file.file_stem().unwrap().to_str().unwrap().to_owned();

        let spec_yaml = fs::read_to_string(&spec_file)?;
        let spec: Spec = serde_yaml::from_str(&spec_yaml)?;

        if spec.required && spec.cases.is_empty() {
            return Err(format!("Failed to load feature-spec {}, must have non-empty 'cases'", spec_file.display()).into());
        }

        for (test_case, test_data) in &spec.cases {
            if test_data.input.is_null() {
                return Err(format!("Failed to load feature spec test case {}.cases.{}, missing 'input' property", spec_file.display(), test_case).into());
            }
        }

        feature_specs.insert(spec_name, spec);
    }

    Ok(feature_specs)
}
