import os
import yaml

from typing import Dict

class TestCases:
    def __init__(self, input: any):
        self.input = input

class Spec:
    def __init__(self, required: bool, cases: Dict[str, TestCases]):
        self.required = required
        self.cases = cases

class FeatureSpecs:
    def __init__(self, specs: Dict[str, Spec]):
        self.specs = specs

def load_feature_specs(directory: str) -> FeatureSpecs:
    feature_specs = {}
    
    spec_files = os.listdir(directory)

    for spec_file in spec_files:
        with open(os.path.join(directory, spec_file), "r") as f:
            spec_yaml = f.read()
            
            spec = yaml.load(spec_yaml, Loader=yaml.SafeLoader)

            if not isinstance(spec, dict):
                raise ValueError(f"Failed to load feature-spec {spec_file}, must be an object.")

            required = spec.get("required")
            if not isinstance(required, bool):
                raise ValueError(f"Failed to load feature-spec {spec_file}, must have property 'required'.")

            cases = spec.get("cases")
            if not isinstance(cases, dict):
                raise ValueError(f"Failed to load feature-spec {spec_file}, must have property 'cases'.")

            for test_case in cases.values():
                if "input" not in test_case:
                    raise ValueError(f"Failed to load feature spec test case {spec_file}.cases.{test_case}, missing 'input' property")

            spec_name = os.path.splitext(spec_file)[0]
            feature_specs[spec_name] = Spec(required, cases)

    return FeatureSpecs(feature_specs)
