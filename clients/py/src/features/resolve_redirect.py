from typing import Any

from polywrap import PolywrapClient, PolywrapClientConfigBuilder, Uri
from pydantic import BaseModel, Field


class TestCaseInput(BaseModel):
    from_: str = Field(..., alias="from")
    to: str


def run_test_case(input: Any) -> None:
    input_obj = TestCaseInput.parse_obj(input)

    from_uri = Uri.from_str(input_obj.from_)
    to_uri = Uri.from_str(input_obj.to)

    config = PolywrapClientConfigBuilder().set_redirect(from_uri, to_uri).build()

    client = PolywrapClient(config)

    print("Resolving Redirect")

    result = client.try_resolve_uri(from_uri)

    match result:
        case Uri():
            print(f"Received URI '{result.uri}'")
            print("Success!")
        case _:
            print("Failed!")
