from typing import Any, TypedDict
from pydantic import BaseModel
from polywrap_client import PolywrapClient
from polywrap_client_config_builder import PolywrapClientConfigBuilder
from polywrap_uri_resolvers import SimpleFileReader, FsUriResolver
from polywrap_core import Uri
from pathlib import Path
from decimal import Decimal


class Obj(TypedDict):
    prop1: str


class Args(TypedDict):
    arg1: str
    obj: Obj


class TestCaseInput(BaseModel):
    args: Args


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    root = Path(__file__).parent.parent.parent.parent.parent / "wraps"
    uri = Uri.from_str(f"fs/{root}/bignumber-type/implementations/as")

    config = (
        PolywrapClientConfigBuilder()
        .add_resolver(FsUriResolver(SimpleFileReader()))
        .build()
    )

    client = PolywrapClient(config)

    print("Invoking method")

    response = client.invoke(
        uri=uri,
        method="method",
        args=input_obj.args,
    )

    if not response:
        raise Exception(f"Error: {response}")

    bignumber = Decimal(response)

    print("Result:", str(bignumber))
    print("Success!")
