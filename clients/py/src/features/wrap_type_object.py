from typing import Any, Dict
from pydantic import BaseModel, Field
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_uri_resolvers import SimpleFileReader, FsUriResolver
from polywrap_core import Uri
from pathlib import Path
import json

class NestedArg(BaseModel):
    prop: str

class Arg(BaseModel):
    prop: str
    nested: NestedArg

class Args(BaseModel):
    arg1: Arg

class TestCaseInput(BaseModel):
    method: str
    args: Args

def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    uri = Uri.from_str(f"fs/{root}/object-type/implementations/as")

    config = (
        PolywrapClientConfigBuilder()
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print(f"Invoking {input_obj.method}")

    response = client.invoke(
        uri=uri,
        method=input_obj.method,
        args=input_obj.args.dict()
    )

    if not response:
        raise Exception(f"Error: {response}")

    print("Result:", json.dumps(response, indent=2))
    print("Success!")
