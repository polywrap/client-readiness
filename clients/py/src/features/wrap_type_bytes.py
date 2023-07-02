from typing import Any, Dict, List, TypedDict
from pydantic import BaseModel, Field
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_uri_resolvers import SimpleFileReader, FsUriResolver
from polywrap_core import Uri
from pathlib import Path


class Arg(TypedDict):
    prop: List[int]


class Args(TypedDict):
    arg: Arg


class TestCaseInput(BaseModel):
    method: str
    args: Args


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    uri = Uri.from_str(f"fs/{root}/bytes-type/implementations/as")

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
        args=input_obj.args,
    )

    if not response:
        raise Exception(f"Error: {response}")

    print(f"Result: [{','.join(map(str, list(response)))}]")
    print("Success!")