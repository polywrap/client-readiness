import os
import yaml

from typing import Any, Dict

from pydantic import BaseModel


class TestCases(BaseModel):
    input: Any


class Spec(BaseModel):
    required: bool
    cases: Dict[str, TestCases]


def load_feature_specs(directory: str) -> Dict[str, Spec]:
    feature_specs: Dict[str, Spec] = {}

    spec_files = os.listdir(directory)

    for spec_file in spec_files:
        with open(os.path.join(directory, spec_file), "r") as f:
            spec_yaml = f.read()
            spec = Spec.validate(yaml.load(spec_yaml, Loader=yaml.SafeLoader))
            spec_name = os.path.splitext(spec_file)[0]
            feature_specs[spec_name] = spec

    return feature_specs
