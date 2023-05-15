use std::env;
use std::error::Error;
use std::path::PathBuf;

mod feature_specs;
mod features;
mod input;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    // Optional 2nd argument, spec filter
    let filter = env::args().nth(1);

    let specs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../specs");
    let specs = feature_specs::load(
        specs_dir.as_path()
    )?;

    let features = features::get();

    for (spec_name, spec) in specs.iter() {
        if let Some(filter) = &filter {
            if filter != spec_name {
                continue;
            }
        }

        let feature = features.get(spec_name).unwrap();

        println!("====================================");
        println!("Begin Feature Spec Test Cases [{}]", spec_name);
        println!("====================================");

        for (test_case_name, test_case) in spec.cases.iter() {
            println!("$Test Start [{}.{}]", spec_name, test_case_name);

            match feature(&test_case.input) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("!Test Error [{}.{}]", spec_name, test_case_name);
                    eprintln!("{}", error);
                }
            }
        }

        println!("====================================");
        println!("End Feature Spec Test Cases [{}]", spec_name);
        println!("====================================");
    }

    Ok(())
}
