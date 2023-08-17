from typing import Any

from polywrap import (
    PolywrapClient,
    PolywrapClientConfigBuilder,
    Uri,
    UriPackageOrWrapper,
    UriResolver,
)
from pydantic import BaseModel

from validators import UriStr


class TestCaseInput(BaseModel):
    authority: str
    result: UriStr


class CustomResolver(UriResolver):
    def __init__(self, authority: str, result: str):
        self.authority = authority
        self.result = result

    def try_resolve_uri(self, uri: Uri, *_: Any) -> UriPackageOrWrapper:
        return Uri.from_str(self.result) if uri.authority == self.authority else uri


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    print("Adding Resolver to ClientConfig")

    resolver = CustomResolver(input_obj.authority, input_obj.result.uri)

    config = PolywrapClientConfigBuilder().add_resolver(resolver).build()

    client = PolywrapClient(config)

    print(f"Resolving a wrap://{input_obj.authority} URI")

    res = client.try_resolve_uri(Uri.from_str(f"wrap://{input_obj.authority}/foo"))

    match res:
        case Uri() as uri:
            print(f"Received URI '{uri}'")
            print("Success!")
        case _:
            print("Failed to resolve URI.")
