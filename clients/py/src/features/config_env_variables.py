from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from pydantic import BaseModel, validator
from typing import Any, TypedDict
import sys
from validators import UriStr, validate_uri


Env = TypedDict("Env", {"str": str, "num": int})


class TestCaseInput(BaseModel):
    uri: UriStr
    env: Env

    @validator("uri")
    def valid_uri(cls, v: Any):
        return validate_uri(v)


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.validate(input)

    print("Adding Env to ClientConfig")

    config = PolywrapClientConfigBuilder().set_env(input_obj.uri, input_obj.env).build()
    client = PolywrapClient(config)

    print("Fetching Env")

    if result := client.get_env_by_uri(input_obj.uri):
        for key in result.keys():
            print(f"env.{key} = {result[key]}")
        print("Success!")
    else:
        print("Failed to fetch env.", file=sys.stderr)
